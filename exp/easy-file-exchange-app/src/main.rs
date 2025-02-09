use easy_file_exchange_app::FileExchangeApp; 

fn main() {
    let mut app = FileExchangeApp::init();

    // Test upload
    let file_id = app.upload_file("photo.jpg".to_string(), b"image binary data".to_vec());
    println!("Uploaded file with ID: {}", file_id);

    // Test retrieval
    if let Some(file) = app.get_file(file_id) {
        println!("Retrieved file: {}", file.filename);
    } else {
        println!("File not found");
    }

    // Test exchange ownership
    if app.exchange_file(file_id, "alice".to_string()) {
        println!("File {} exchanged successfully", file_id);
    } else {
        println!("Exchange failed");
    }

    // Test retrieval after exchange
    if let Some(file) = app.get_file(file_id) {
        println!("New owner of file: {}", file.owner);
    }
}
