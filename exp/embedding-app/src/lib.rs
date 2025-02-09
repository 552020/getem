use onnxruntime::{
    environment::Environment,
    GraphOptimizationLevel,
};
use tokenizers::Tokenizer;
use ndarray::Array;
use std::sync::Arc;

pub struct EmbeddingApp {
    env: Arc<Environment>,
    model_path: String,
    tokenizer: Tokenizer,
}

impl EmbeddingApp {
    pub fn init(model_path: String) -> Self {
        // Create environment and wrap in Arc
        let env = Arc::new(
            Environment::builder()
                .with_name("embedding_environment")
                .build()
                .expect("Failed to create environment")
        );
        
        let tokenizer = Tokenizer::from_file("tokenizer.json")
            .expect("Failed to load tokenizer");
            
        EmbeddingApp { 
            env, 
            model_path,
            tokenizer 
        }
    }

    pub fn generate_embedding(&self, text: String) -> Vec<f32> {
        // Create a new session for each embedding generation
        let mut session = self.env  // Make session mutable
            .new_session_builder()
            .expect("Failed to create session builder")
            .with_optimization_level(GraphOptimizationLevel::Basic)
            .expect("Failed to set optimization level")
            .with_model_from_file(&self.model_path)
            .expect("Failed to load model");

        let encoding = self.tokenizer.encode(text, true)
            .expect("Failed to encode text");
        
        let input_ids: Vec<i64> = encoding.get_ids()
            .iter()
            .map(|&id| id as i64)
            .collect();
            
        let array = Array::from_shape_vec(
            (1, input_ids.len()),
            input_ids
        ).expect("Failed to create input array");

        let outputs = session
            .run(vec![array])
            .expect("Failed to run model");
            
        outputs[0].view().as_slice().unwrap().to_vec()
    }
}
