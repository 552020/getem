#[cfg(test)]
mod tests {
    use embedding_app::EmbeddingApp;

    #[test]
    fn test_embedding_generation() {
        // let app = EmbeddingApp::init("models/minilm.onnx".to_string());
		let app = EmbeddingApp::init("../../models/minilm.onnx".to_string());
        let embedding = app.generate_embedding("Test sentence".to_string());
        
        // MiniLM should output 384-dimensional vectors
        assert_eq!(embedding.len(), 384);
        
        // Values should be floating points between -1 and 1
        for value in embedding {
            assert!(value >= -1.0 && value <= 1.0);
        }
    }
}