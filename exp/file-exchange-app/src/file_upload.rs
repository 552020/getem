// src/logic/file-exchange-app/src/file_upload.rs
// this file is not used in the app, but it's just an example of how to upload a file to the app

use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::app;
use calimero_storage::collections::Vector;

#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct FileData {
    name: String,
    content: Vec<u8>,
}

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct HelloApp {
    files: Vector<FileData>
}

#[app::logic]
impl HelloApp {
    #[app::init]
    pub fn init() -> Self {
        HelloApp {
            files: Vector::new()
        }
    }

    pub fn upload_file(&mut self, name: String, content: Vec<u8>) -> Result<String, String> {
        let file = FileData {
            name: name.clone(),
            content,
        };
        
        self.files.push(file);
        Ok(format!("File '{}' uploaded successfully", name))
    }

    pub fn get_file_names(&self) -> Vec<String> {
        self.files.iter().map(|f| f.name.clone()).collect()
    }
}