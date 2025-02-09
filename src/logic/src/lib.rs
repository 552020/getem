use calimero_sdk::{
    app,
    borsh::{BorshDeserialize, BorshSerialize},
    serde::{Serialize, Deserialize},
    env,
    env::ext::{AccountId, ProposalId},
    types::Error,
};
use calimero_storage::collections::{UnorderedMap, Vector};
use calimero_context_config::icp::types::ICSigned;
use calimero_context_config::icp::ICProxyMutateRequest;
use ed25519_dalek::{SigningKey, VerifyingKey};
use calimero_context_config::icp::{
    ICProposal, 
    ICProposalAction, 
    ICProposalApprovalWithSigner,
    ProposalAction,
};
use rand::Rng;

// ---------------- FileExchange Types ----------------

#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct FileEntry {
    name: String,
    content: String,
    owner: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "calimero_sdk::serde")]
pub enum FileExchangeError {
    FileNotFound,
    ProposalError(String),
    StorageError(String),
    InvalidOperation,
    Unauthorized,
}

// ---------------- Proposal Types ----------------

#[derive(
    Clone, Debug, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize, Serialize, Deserialize,
)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct Message {
    id: String,
    proposal_id: String,
    author: String,
    text: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "calimero_sdk::serde")]
pub struct CreateProposalRequest {
    pub action_type: String,
    pub params: serde_json::Value,
}

// ---------------- Events ----------------

#[app::event]
pub enum Event {
    // File exchange events
    FileUploaded { name: String, owner: String },
    FileDownloaded { name: String, downloader: String },
    FileDeleted { name: String },
    // Proposal events
    ProposalCreated { id: ProposalId },
    ApprovedProposal { id: ProposalId },
}

// ---------------- Main State ----------------

#[app::state(emits = Event)]
#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct FileExchangeState {
    // FileExchange storage (using std::collections::HashMap)
    files: UnorderedMap<String, FileEntry>,
    // Proposal-related storage (using calimero_storage collections)
    proposal_messages: UnorderedMap<ProposalId, Vector<Message>>,
}

// ---------------- Logic Implementation ----------------

#[app::logic]
impl FileExchangeState {
    #[app::init]
    pub fn init() -> FileExchangeState {
        FileExchangeState {
            files: UnorderedMap::new(),
            proposal_messages: UnorderedMap::new(),
        }
    }

    // ===== File Exchange Functions =====

    pub fn upload_file(
        &mut self,
        name: String,
        content: String,
        owner: String,
    ) -> Result<(), FileExchangeError> {
        if self.files.get(&name).map_err(|e| FileExchangeError::StorageError(e.to_string()))?.is_some() {
            return Err(FileExchangeError::InvalidOperation);
        }

        let file = FileEntry {
            name: name.clone(),
            content,
            owner: owner.clone(),
        };
        
        self.files
            .insert(name.clone(), file)
            .map_err(|e| FileExchangeError::StorageError(e.to_string()))?;

        app::emit!(Event::FileUploaded { name, owner });
        Ok(())
    }

    pub fn download_file(
        &self,
        name: String,
        downloader: String,
    ) -> Result<String, FileExchangeError> {
        let file = self.files
            .get(&name)
            .map_err(|e| FileExchangeError::StorageError(e.to_string()))?
            .ok_or(FileExchangeError::FileNotFound)?;

        app::emit!(Event::FileDownloaded {
            name,
            downloader
        });

        Ok(file.content)
    }

    pub fn delete_file(
        &mut self,
        name: String,
        requester: String,
    ) -> Result<(), FileExchangeError> {
        let file = self.files
            .get(&name)
            .map_err(|e| FileExchangeError::StorageError(e.to_string()))?
            .ok_or(FileExchangeError::FileNotFound)?;

        if file.owner != requester {
            return Err(FileExchangeError::Unauthorized);
        }
        
        self.files
            .remove(&name)
            .map_err(|e| FileExchangeError::StorageError(e.to_string()))?;

        app::emit!(Event::FileDeleted { name });
        Ok(())
    }

    pub fn list_files(&self) -> Result<Vec<String>, FileExchangeError> {
        self.files
            .entries()
            .map_err(|e| FileExchangeError::StorageError(e.to_string()))
            .map(|entries| entries.map(|(key, _)| key).collect())
    }

    // ===== Proposal Functions =====

    pub fn create_new_proposal(
        &mut self,
        request: CreateProposalRequest,
        signer: &SigningKey
    ) -> Result<ProposalId, Error> {
        env::log("Starting create_new_proposal");
        
        let proposal = match request.action_type.as_str() {
            "ExternalFunctionCall" => {
                env::log("Processing ExternalFunctionCall");
                let receiver_id = request.params["receiver_id"]
                    .as_str()
                    .ok_or_else(|| Error::msg("receiver_id is required"))?;
                let method_name = request.params["method_name"]
                    .as_str()
                    .ok_or_else(|| Error::msg("method_name is required"))?;
                let args = request.params["args"]
                    .as_str()
                    .ok_or_else(|| Error::msg("args is required"))?;
                let deposit = request.params["deposit"]
                    .as_str()
                    .ok_or_else(|| Error::msg("deposit is required"))?
                    .parse::<u128>()?;

                env::log(&format!(
                    "Parsed values: receiver_id={}, method_name={}, args={}, deposit={}",
                    receiver_id, method_name, args, deposit
                ));

                Self::external()
                    .propose()
                    .external_function_call(
                        receiver_id.to_string(),
                        method_name.to_string(),
                        args.to_string(),
                        deposit,
                    )
            }
            "Transfer" => {
                env::log("Processing Transfer");
                let receiver_id = request.params["receiver_id"]
                    .as_str()
                    .ok_or_else(|| Error::msg("receiver_id is required"))?;
                let amount = request.params["amount"]
                    .as_str()
                    .ok_or_else(|| Error::msg("amount is required"))?
                    .parse::<u128>()?;

                Self::external()
                    .propose()
                    .transfer(AccountId(receiver_id.to_string()), amount)
            }
            "SetContextValue" => {
                env::log("Processing SetContextValue");
                let key = request.params["key"]
                    .as_str()
                    .ok_or_else(|| Error::msg("key is required"))?
                    .as_bytes()
                    .to_vec()
                    .into_boxed_slice();
                let value = request.params["value"]
                    .as_str()
                    .ok_or_else(|| Error::msg("value is required"))?
                    .as_bytes()
                    .to_vec()
                    .into_boxed_slice();

                Self::external()
                    .propose()
                    .set_context_value(key, value)
            }
            "SetNumApprovals" => Self::external()
                .propose()
                .set_num_approvals(
                    request.params["num_approvals"]
                        .as_u64()
                        .ok_or(Error::msg("num_approvals is required"))? as u32,
                ),
            "SetActiveProposalsLimit" => Self::external()
                .propose()
                .set_active_proposals_limit(
                    request.params["active_proposals_limit"]
                        .as_u64()
                        .ok_or(Error::msg("active_proposals_limit is required"))?
                        as u32,
                ),
            "DeleteProposal" => Self::external()
                .propose()
                .delete(ProposalId(
                    hex::decode(
                        request.params["proposal_id"]
                            .as_str()
                            .ok_or_else(|| Error::msg("proposal_id is required"))?,
                    )?
                    .try_into()
                    .map_err(|_| Error::msg("Invalid proposal ID length"))?,
                )),
            _ => return Err(Error::msg("Invalid action type")),
        };

        // Use the helper method to sign and send
        let proposal_id = self.sign_and_send_proposal(proposal, signer)?;
        
        // Store proposal messages
        self.proposal_messages.insert(proposal_id, Vector::new())?;
        
        app::emit!(Event::ProposalCreated { id: proposal_id });
        Ok(proposal_id)
    }

    pub fn sign_and_send_proposal(
        &self,
        proposal: ProposalAction,
        signer: &SigningKey
    ) -> Result<ProposalId, Error> {
        // Create the ICProposal structure
        let ic_proposal = ICProposal {
            id: self.generate_proposal_id()?,
            author_id: signer.verifying_key().to_bytes().into(),
            actions: vec![proposal.into()],
        };

        let request = ICProxyMutateRequest::Propose { 
            proposal: ic_proposal 
        };

        let signed_request = ICSigned::new(request, |bytes| Ok(signer.sign(bytes)))?;

        // Send the signed request and handle the response
        match Self::external().send_signed_proposal(signed_request)? {
            Some(proposal_with_approvals) => Ok(proposal_with_approvals.proposal_id),
            None => Err(Error::msg("Failed to create proposal"))
        }
    }

    // Helper function to generate a proposal ID using env::random_seed
    fn generate_proposal_id(&self) -> Result<ProposalId, Error> {
        let random_bytes = env::random_seed();
        if random_bytes.len() < 32 {
            return Err(Error::msg("Insufficient random bytes"));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&random_bytes[..32]);
        Ok(ProposalId(bytes))
    }

    pub fn approve_proposal(
        &self,
        proposal_id: ProposalId,
        signer: &SigningKey
    ) -> Result<(), Error> {
        let approval = ICProposalApprovalWithSigner {
            proposal_id,
            signer_id: signer.verifying_key().to_bytes().into(),
        };

        let request = ICProxyMutateRequest::Approve { approval };
        let signed_request = ICSigned::new(request, |bytes| Ok(signer.sign(bytes)))?;

        Self::external().send_signed_proposal(signed_request)?;
        app::emit!(Event::ApprovedProposal { id: proposal_id });
        Ok(())
    }

    pub fn get_proposal_messages(&self, proposal_id: ProposalId) -> Result<Vec<Message>, Error> {
        let Some(msgs) = self.proposal_messages.get(&proposal_id)? else {
            return Ok(vec![]);
        };
        
        // Create a new vector and collect the entries
        let entries: Vec<Message> = msgs.entries()?.collect();
        Ok(entries)
    }

    pub fn send_proposal_messages(
        &mut self,
        proposal_id: ProposalId,
        message: Message,
    ) -> Result<(), Error> {
        let mut messages = self.proposal_messages.get(&proposal_id)?.unwrap_or_default();
        messages.push(message)?;
        self.proposal_messages.insert(proposal_id, messages)?;
        Ok(())
    }
}
