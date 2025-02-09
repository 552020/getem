use calimero_sdk::{
    app,
    borsh::{BorshDeserialize, BorshSerialize},
    serde::{Serialize, Deserialize},
    env,
    env::ext::{AccountId, ProposalId},
    types::Error,
};
use calimero_storage::collections::{UnorderedMap, Vector};

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
    ) -> Result<ProposalId, Error> {
        env::log("Starting create_new_proposal");
        env::log(&format!("Request type: {}", request.action_type));

        let proposal_id = match request.action_type.as_str() {
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
                    .send()
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
                    .send()
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
                    .send()
            }
            "SetNumApprovals" => Self::external()
                .propose()
                .set_num_approvals(
                    request.params["num_approvals"]
                        .as_u64()
                        .ok_or(Error::msg("num_approvals is required"))? as u32,
                )
                .send(),
            "SetActiveProposalsLimit" => Self::external()
                .propose()
                .set_active_proposals_limit(
                    request.params["active_proposals_limit"]
                        .as_u64()
                        .ok_or(Error::msg("active_proposals_limit is required"))?
                        as u32,
                )
                .send(),
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
                ))
                .send(),
            _ => return Err(Error::msg("Invalid action type")),
        };

        app::emit!(Event::ProposalCreated { id: proposal_id });

        let old = self.proposal_messages.insert(proposal_id, Vector::new())?;
        if old.is_some() {
            return Err(Error::msg("proposal already exists"));
        }

        Ok(proposal_id)
    }

    pub fn approve_proposal(&self, proposal_id: ProposalId) -> Result<(), Error> {
        // Optionally, check for the proposal's existence before approving.
        Self::external().approve(proposal_id);
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
