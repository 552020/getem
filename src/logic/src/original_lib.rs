use calimero_sdk::{
    app,
    borsh::{BorshDeserialize, BorshSerialize},
    serde::{Serialize, Deserialize},
    env,
};
use std::collections::HashMap;

#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
#[serde(crate = "calimero_sdk::serde")]
pub struct FileEntry {
    name: String,
    content: String,
    owner: String,
}

#[app::event]
pub enum Event {
    FileUploaded { name: String, owner: String },
    FileDownloaded { name: String, downloader: String },
    FileDeleted { name: String },
}

#[app::state(emits = Event)]
#[derive(Default, BorshDeserialize, BorshSerialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct FileExchangeState {
    files: HashMap<String, FileEntry>,
}

#[app::logic]
impl FileExchangeState {
    #[app::init]
    pub fn init() -> FileExchangeState {
        FileExchangeState {
            files: HashMap::new(),
        }
    }

    pub fn upload_file(&mut self, name: String, content: String, owner: String) -> bool {
        if self.files.contains_key(&name) {
            env::log("File with this name already exists");
            return false;
        }

        let file = FileEntry {
            name: name.clone(),
            content,
            owner: owner.clone(),
        };
        
        self.files.insert(name.clone(), file);
        app::emit!(Event::FileUploaded { name, owner });
        true
    }

    pub fn download_file(&self, name: String, downloader: String) -> Option<String> {
        if let Some(file) = self.files.get(&name) {
            app::emit!(Event::FileDownloaded { name: name.clone(), downloader });
            Some(file.content.clone())
        } else {
            env::log("File not found");
            None
        }
    }

    pub fn delete_file(&mut self, name: String, requester: String) -> bool {
        if let Some(file) = self.files.get(&name) {
            if file.owner != requester {
                env::log("Only the owner can delete the file");
                return false;
            }
            
            self.files.remove(&name);
            app::emit!(Event::FileDeleted { name });
            true
        } else {
            env::log("File not found");
            false
        }
    }

    pub fn list_files(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }
}

