// lib.rs
use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::app;
// use calimero_storage;

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct HelloApp {}

#[app::logic]
impl HelloApp {
    #[app::init]
    pub fn init() -> Self {
        HelloApp {}
    }

    pub fn say_hello(&self) -> String {
        "Hello from Calimero Node!".to_string()
    }
}
