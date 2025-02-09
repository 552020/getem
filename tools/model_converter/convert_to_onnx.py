import torch
from transformers import AutoModel, AutoTokenizer
import argparse
import os


def convert_to_onnx(model_name, output_path):
    print(f"🚀 Loading model: {model_name}")
    model = AutoModel.from_pretrained(model_name)
    tokenizer = AutoTokenizer.from_pretrained(model_name)

    # Ensure the output directory exists
    os.makedirs(os.path.dirname(output_path), exist_ok=True)

    # Define dummy input for the model (adjust if needed)
    dummy_input = torch.randint(
        0, 100, (1, 512)
    )  # Adjust based on model's max input length
    print(f"✅ Model loaded. Converting to ONNX...")

    # Export model to ONNX format
    torch.onnx.export(
        model,
        dummy_input,
        output_path,
        opset_version=17,
        input_names=["input"],
        output_names=["output"],
    )
    print(f"🎉 Conversion complete! Model saved as: {output_path}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Convert Hugging Face Transformer to ONNX"
    )
    parser.add_argument(
        "--model",
        type=str,
        required=True,
        help="Hugging Face model name (e.g., 'sentence-transformers/all-MiniLM-L6-v2')",
    )
    parser.add_argument(
        "--output", type=str, default="model.onnx", help="Output ONNX file path"
    )
    args = parser.parse_args()

    convert_to_onnx(args.model, args.output)
