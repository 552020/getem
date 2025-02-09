use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::app;
use calimero_storage::collections::UnorderedMap;

// Simple File structure
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct File {
    pub id: u64,
    pub owner: String,
    pub filename: String,
    pub content: Vec<u8>,
}

// App Events
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

// App State
#[app::state(emits = for<'a> FileEvent<'a>)]
#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct FileExchangeApp {
    files: UnorderedMap<Vec<u8>, File>,
    next_file_id: u64,
}

// App Logic
#[app::logic]
impl FileExchangeApp {
    #[app::init]
    pub fn init() -> Self {
        Self {
            files: UnorderedMap::new(),
            next_file_id: 0,
        }
    }

    // Upload a file
    pub fn upload_file(&mut self, filename: String, content: Vec<u8>) -> u64 {
        let file_id = self.next_file_id;
        let owner = "test-owner".to_string();

        let file = File {
            id: file_id,
            owner: owner.clone(),
            filename: filename.clone(),
            content,
        };

        let key = file_id.to_le_bytes().to_vec();
        self.files.insert(key, file).expect("Storage error");
        self.next_file_id += 1;

        app::emit!(FileEvent::FileUploaded {
            file_id,
            owner: &owner,
            filename: &filename,
        });

        file_id
    }

    // Exchange a file
    pub fn exchange_file(&mut self, file_id: u64, new_owner: String) -> bool {
        let key = file_id.to_le_bytes().to_vec();
        if let Some(mut file) = self.files.get(&key).expect("Storage error") {
            let old_owner = file.owner.clone();
            file.owner = new_owner.clone();
            
            self.files.insert(key, file).expect("Storage error");

            app::emit!(FileEvent::FileExchanged {
                file_id,
                from: &old_owner,
                to: &new_owner,
            });

            true
        } else {
            false
        }
    }

    // Get file details
    pub fn get_file(&self, file_id: u64) -> Option<File> {
        let key = file_id.to_le_bytes().to_vec();
        self.files.get(&key).expect("Storage error")
    }
}
