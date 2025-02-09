// lib.rs
use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::app;
use calimero_storage::collections::Vector;
use calimero_storage::collections::UnorderedMap;
use calimero_sdk::serde::{Deserialize, Serialize};
use ed25519_dalek::{Signature, SigningKey, VerifyingKey};

mod repr;
mod key;
use repr::{Repr, Raw, ReprBytes};
use calimero_sdk::env;


#[cfg(test)]
mod tests;


// #[app::state]
// #[derive(Default, BorshSerialize, BorshDeserialize)]
// #[borsh(crate = "calimero_sdk::borsh")]
// struct HelloApp {}

// #[app::logic]
// impl HelloApp {
//     #[app::init]
//     pub fn init() -> Self {
//         HelloApp {}
//     }

//     pub fn say_hello(&self) -> String {
//         "Hello from Calimero Node!".to_string()
//     }
// }

// lib.rs

//
// Define events so that off-chain clients (or other apps) can react to uploads or exchanges.
//
#[app::event]
pub enum FileEvent<'a> {
    FileUploaded {
        file_id: u64,
        owner: &'a str,
        filename: &'a str,
    },
    FileExchanged {
        file_id: u64,
        from: &'a str,
        to: &'a str,
    },
}

//
// A simple file structure with an ID, the uploader (owner), a filename, and the content.
//
#[derive(Debug, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct User {
    peer_id: String,                           // Calimero P2P network ID
    public_key: Repr<VerifyingKey, repr::Raw>, // For cryptographic verification
    name: Option<String>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct File {
    pub id: u64,
    pub owner: User,
    pub filename: String,
    pub content: Vec<u8>,
}

// Error handling
#[derive(Debug, Serialize)]
#[serde(crate = "calimero_sdk::serde")]
pub enum FileError {
    NotFound,
    InvalidSignature,
    NotAuthorized,
    UnauthorizedTransfer,
    InvalidRecipient,
}

//
// The state of our app holds an UnorderedMap of files and a counter for generating unique file IDs.
//
#[app::state(emits = for<'a> FileEvent<'a>)]
#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct FileExchangeApp {
    files: UnorderedMap<u64, File>,
    next_file_id: u64,
}

//
// Application logic: methods to initialize, upload, exchange, and query files.
//
#[app::logic]
impl FileExchangeApp {
    /// The initializer for the app. It sets up an empty map for storing files and initializes the file ID counter.
    #[app::init]
    pub fn init() -> Self {
        Self {
            files: UnorderedMap::new(),
            next_file_id: 0,
        }
    }

    /// Allows a user to upload a file.
    ///
    /// # Arguments
    /// * `filename` - The name of the file.
    /// * `content` - The content of the file as a vector of bytes.
    /// * `peer_id` - The peer ID of the user uploading the file.
    /// * `public_key` - The public key of the user uploading the file.
    /// * `signature` - The signature of the file.
    ///
    /// # Returns
    /// * `Result<u64, FileError>` - The unique ID assigned to the file or an error if it fails.
    pub fn upload_file(
        &mut self, 
        filename: String, 
        content: Vec<u8>,
        peer_id: String,
        public_key: Repr<VerifyingKey, Raw>,
        signature: Signature
    ) -> Result<u64, FileError> {
        // Verify the signature matches the public key
        if !verify_signature(&public_key, &signature) {
            return Err(FileError::InvalidSignature);
        }

        let user = User {
            peer_id,
            public_key,
            name: None,
        };

        let file_id = self.next_file_id;
        let file = File {
            id: file_id,
            owner: user,
            filename: filename.clone(),
            content,
        };

        self.files.insert(file_id, file).expect("Storage error");
        self.next_file_id += 1;

        app::emit!(FileEvent::FileUploaded {
            file_id,
            owner: &peer_id,
            filename: &filename,
        });

        Ok(file_id)
    }

    /// Allows the owner of a file to exchange (transfer) it to another user.
    ///
    /// # Arguments
    /// * `file_id` - The unique ID of the file.
    /// * `recipient` - The address (as a `String`) of the recipient.
    ///
    /// # Returns
    /// * `Result<(), FileError>` - Returns Ok(()) if the exchange is successful, or an error if it fails.
    pub fn exchange_file(&mut self, file_id: u64, recipient: String) -> Result<(), FileError> {
        let mut file = self.files.get(&file_id)
            .expect("Storage error")
            .ok_or(FileError::NotFound)?;
        
        // Create new user for recipient
        let new_owner = User {
            peer_id: recipient.clone(),  // Changed from address
            public_key: file.owner.public_key.clone(),  // Keep the same public key
            name: None,
        };

        // Update file ownership
        file.owner = new_owner;
        self.files.insert(file_id, file).expect("Storage error");

        app::emit!(FileEvent::FileExchanged {
            file_id,
            from: &file.owner.peer_id,
            to: &recipient,
        });

        Ok(())
    }

    /// A simple getter to retrieve file details by file ID.
    ///
    /// # Arguments
    /// * `file_id` - The unique ID of the file.
    ///
    /// # Returns
    /// * `Option<File>` - Returns the file if it exists.
    pub fn get_file(&self, file_id: u64) -> Option<File> {
        self.files.get(&file_id).expect("Storage error")
    }
}

fn verify_signature(public_key: &Repr<VerifyingKey>, signature: &Signature) -> bool {
    // Implementation needed
    true  // Temporary return for compilation
}

#[derive(Debug, Clone, PartialEq)]
struct FileId(u64);

impl AsRef<[u8]> for FileId {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(&self.0 as *const u64 as *const u8, std::mem::size_of::<u64>()) }
    }
}
