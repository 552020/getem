# Rust ONNX Embedding Generator

This project enables **local embedding generation** using a **Rust-WASM** runtime and an **ONNX model** for lightweight, decentralized AI.  
Designed to run inside **Calimero nodes**, but can be tested as a standalone Rust app first.

## Notes

Python 3.11 is required

## **📌 Features**

✅ Converts **Hugging Face models** (e.g., MiniLM, BGE) to **ONNX**  
✅ Runs **ONNX Runtime in Rust** to generate text embeddings locally  
✅ Designed for **Calimero node integration**  
✅ **Fully offline** execution – no external API calls needed

---

## ** Convert a Model to ONNX**

Before running the Rust app, we need an ONNX model.

### ** Step 1: Install Python Dependencies**

Make sure you have a version of Python installed between the 3.8 and 3.11. The newest Python version are this moment not suppored. Create a virtual environemnt then run.

```sh
pip install torch transformers onnx onnxruntime
```

or

```
pip install -r requirements.txt
```

### ** Step 2: Convert a Hugging Face Model to ONNX**

Use the **conversion script** to turn a **MiniLM** or **BGE** model into an ONNX format.

#### Example: Convert MiniLM to ONNX

```sh
pythonX convert_to_onnx.py --model "sentence-transformers/all-MiniLM-L6-v2" --output "models/minilm.onnx"
```

#### Example: Convert BGE to ONNX

```sh
python convert_to_onnx.py --model "BAAI/bge-base-en" --output "models/bge.onnx"
```

### **🔹 Step 3: Verify Output**

After conversion, check that your **ONNX model exists**:

```sh
ls models/
# Should see: minilm.onnx or bge.onnx
```

---

## **2️⃣ Run the Rust Embedding App**

Now that we have an ONNX model, we can use **Rust** to generate embeddings locally.

### **🔹 Step 1: Install Rust & Dependencies**

Ensure Rust is installed:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the project and install dependencies:

```sh
git clone https://github.com/your-username/rust-embedding-app.git
cd rust-embedding-app
cargo build --release
```

### **🔹 Step 2: Run the Embedding Generator**

#### ✅ Run with Direct Text Input

```sh
./target/release/rust-embedding-app --input-text "This is a test" --model-path "models/minilm.onnx"
```

#### ✅ Run with a Text File

```sh
./target/release/rust-embedding-app --input-file "input.txt" --model-path "models/minilm.onnx"
```

#### ✅ Check the Output

Embeddings are saved as JSON:

```sh
cat output.json
```

---

## **3️⃣ Next Steps**

🚀 **Optimize the WASM binary** for Calimero node execution.  
🚀 **Integrate with decentralized storage inside the node.**  
🚀 **Experiment with different embedding models for performance.**

---

## **🤝 Contributing**

💡 PRs & discussions welcome! If you have **a better model suggestion** or **want to help optimize ONNX for Rust-WASM**, open an issue.

---

## **🔗 References**

- ONNX Runtime: [https://onnxruntime.ai/](https://onnxruntime.ai/)
- Hugging Face Models: [https://huggingface.co/models](https://huggingface.co/models)
- Rust-WASM: [https://rustwasm.github.io/book/](https://rustwasm.github.io/book/)

```

---

## **📌 Summary**
- **README.md** explains **why** the project exists and **how to use it**.
- Covers **converting models**, **running the Rust app**, and **next steps**.
- **Easy-to-follow CLI commands** for both Python & Rust.

💡 **Do you need any additional setup details in the README?** 🚀
```
