// tests.rs - all test code
use super::*;
use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;

fn create_test_user() -> (User, SigningKey) {
    // Generate a random keypair
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
    
    let user = User {
        peer_id: "test-peer-id".to_string(),
        public_key: Repr::<VerifyingKey, Raw>::from(verifying_key),
        name: Some("Test User".to_string()),
    };

    (user, signing_key)
}

#[test]
fn test_file_upload() {
    // Initialize the app
    let mut app = FileExchangeApp::init();
    
    // Create a test user
    let (user, signing_key) = create_test_user();
    
    // Create test file data
    let filename = "test.txt".to_string();
    let content = b"Hello, World!".to_vec();
    
    // Sign the file content
    let signature = signing_key.sign(&content);

    // Upload the file
    let result = app.upload_file(
        filename.clone(),
        content,
        user.peer_id.clone(),
        user.public_key.clone(),
        signature,
    );

    // Assert the upload was successful
    assert!(result.is_ok());
    let file_id = result.unwrap();
    
    // Verify the file exists in storage
    let stored_file = app.files.get(&file_id).expect("Storage error").expect("File should exist");
    assert_eq!(stored_file.filename, filename);
    assert_eq!(stored_file.owner.peer_id, user.peer_id);
}

#[test]
fn test_file_exchange() {
    let mut app = FileExchangeApp::init();
    
    // Create two test users
    let (user1, signing_key1) = create_test_user();
    let (user2, _) = create_test_user();
    
    // Upload a file as user1
    let filename = "test.txt".to_string();
    let content = b"Hello, World!".to_vec();
    let signature = signing_key1.sign(&content);

    let file_id = app.upload_file(
        filename,
        content,
        user1.peer_id.clone(),
        user1.public_key.clone(),
        signature,
    ).unwrap();

    // Try to exchange the file to user2
    let result = app.exchange_file(
        file_id,
        user2.peer_id.clone(),
        // You'll need to implement signature verification for exchange as well
    );

    assert!(result.is_ok());
    
    // Verify the file ownership changed
    let stored_file = app.files.get(&file_id).expect("File should exist");
    assert_eq!(stored_file.owner.peer_id, user2.peer_id);
}