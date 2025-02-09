mod embedding;

use clap::Parser;
use std::path::Path;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    model_path: String,

    #[clap(short, long)]
    input_text: String,
}

#[derive(Debug)]
pub enum EmbeddingError {
    ModelLoadError(String),
    TokenizationError(String),
    InferenceError(String)
}

impl std::error::Error for EmbeddingError {}

pub struct ModelConfig {
    max_length: usize,
    pad_token_id: i64,
    dimension: usize,
}

impl ModelConfig {
    pub fn new(model_type: &str) -> Self {
        match model_type {
            "minilm" => ModelConfig {
                max_length: 512,
                pad_token_id: 0,
                dimension: 384,
            },
            "bge" => ModelConfig {
                max_length: 512,
                pad_token_id: 0,
                dimension: 768,
            },
            _ => panic!("Unsupported model type")
        }
    }
}

fn text_to_tensor(text: &str, tokenizer_path: &str) -> Vec<f32> {
    let tokenizer = Tokenizer::from_file(tokenizer_path)
        .expect("Failed to load tokenizer");
    
    let encoding = tokenizer.encode(text, true)
        .expect("Failed to encode text");
    
    // Convert to input tensor format
    let input_ids: Vec<f32> = encoding.get_ids()
        .iter()
        .map(|&id| id as f32)
        .collect();
    
    input_ids
}

pub fn generate_embedding(model_path: &str, text: &str) -> Result<Vec<f32>, EmbeddingError> {
    // Implementation of the function
    Ok(Vec::new())
}

fn main() {
    let args = Args::parse();
    
    // Initialize the embedding app
    let app = embedding_app::EmbeddingApp::init(args.model_path);
    
    // Generate embedding
    let embedding = app.generate_embedding(args.input_text);
    
    println!("Generated embedding with {} dimensions", embedding.len());
    println!("First few values: {:?}", &embedding[..5]);
}
