### Embedding Generation and AI Agent Functionality in a Calimero Setup

The three possible options to create embeddings locally are the following: SentenceTransformers, ONNX Runtime, and Llama.cpp. Additionally, embeddings can also be created in other ways, such as using the Vercel AI SDK for convenience. This document will also consider how the setup to create embeddings would work for full agent functionalities.

#### **1. What Are SentenceTransformers, ONNX Runtime, and Llama.cpp?**

**SentenceTransformers**

- **What It Is**: A Python library based on the `transformers` library from Hugging Face. It provides pre-trained models for generating embeddings, designed for tasks like text similarity, clustering, and classification.
- **Usage**: Runs on Python and uses models like BERT, RoBERTa, or MiniLM.
- **Limitations**: Requires Python; not inherently decentralized or WASM-compatible.

**ONNX Runtime**

- **What It Is**: A runtime engine for running machine learning models in the ONNX (Open Neural Network Exchange) format. ONNX models can be exported from frameworks like PyTorch and TensorFlow.
- **Usage**:
  - Runs locally on various platforms (CPU/GPU/WASM).
  - Supports Rust (via `ort`) and WASM (to run models in browsers).
- **Key Advantage**: Optimized for lightweight, fast inference on edge devices or browsers.

**Llama.cpp**

- **What It Is**: A C++ framework for running large language models (LLMs) like LLaMA, Mistral, and others locally, without requiring a cloud-based API.
- **Usage**: Works on low-resource hardware (e.g., CPUs) and does not require Python or GPU.
- **Decentralization**: Designed to run fully offline and independently of centralized services, making it ideal for decentralized apps.
- **Limitations**: Requires more memory compared to ONNX Runtime.

---

### **2. Comparative Overview of SentenceTransformers, ONNX Runtime, and Llama.cpp**

#### **Embedding Creation System**

| **Feature**              | **SentenceTransformers** | **ONNX Runtime**              | **Llama.cpp**               |
| ------------------------ | ------------------------ | ----------------------------- | --------------------------- |
| **WASM Compatibility**   | ❌ No                    | ✅ Yes                        | ❌ No                       |
| **Rust Compatibility**   | ❌ No                    | ✅ Yes (via `ort` crate)      | ✅ Yes (via `llama_cpp_rs`) |
| **Decentralization**     | ❌ Not decentralized     | ✅ Partial                    | ✅ Fully decentralized      |
| **In-Browser Support**   | ❌ No                    | ✅ Yes (via WASM)             | ❌ No                       |
| **Model Size**           | 50MB - 400MB             | 20MB - 300MB                  | 1GB+ (GGUF models)          |
| **Dependencies**         | Python runtime           | Lightweight (WASM/CPU)        | C++ runtime                 |
| **Compute Needs**        | Low (CPU/GPU)            | Very Low (optimized for edge) | Medium (RAM-intensive)      |
| **Model Setup**          | Download once            | One-time conversion           | Download GGUF models        |
| **Total Space Required** | ~500MB - 1GB             | ~100MB - 300MB                | ~1GB - 7GB                  |

#### **AI Agent with RAG (Retrieval-Augmented Generation)**

| **Feature**            | **SentenceTransformers** | **ONNX Runtime**           | **Llama.cpp**              |
| ---------------------- | ------------------------ | -------------------------- | -------------------------- |
| **RAG Support**        | ✅ Text embeddings only  | ✅ Full inference possible | ✅ Full inference possible |
| **Model Size for RAG** | 50MB - 400MB             | 20MB - 300MB               | 1GB+                       |
| **Compute Needs**      | Low                      | Low                        | Medium                     |
| **Dependencies**       | Python                   | Lightweight (Rust/WASM)    | C++ runtime                |
| **Inference Speed**    | Moderate                 | Fast                       | Medium                     |
| **Fully Offline**      | ❌ No                    | ✅ Yes                     | ✅ Yes                     |

---

### **3. Detailed Analysis**

#### **Fully Decentralization: Llama.cpp vs. ONNX Runtime**

- **Llama.cpp**:
  - Designed for complete decentralization.
  - Requires no external dependencies (e.g., Python, cloud services, or ONNX).
  - Ideal for setups where independence from centralized systems is critical.
  - Uses **GGUF models**, which are optimized for local execution, even on CPUs.
- **ONNX Runtime**:
  - Supports local execution but requires pre-trained ONNX models, often exported from centralized frameworks like PyTorch or TensorFlow.
  - Decentralization is partial because ONNX models are typically trained in centralized environments before deployment. However, for our use case, this does not matter since we are not training models but using pre-trained ones. See the paragraph at the end for further explanation."

#### **WASM Compatibility**

- **ONNX Runtime**:
  - Supports WASM, enabling model execution in browsers or WASM runtimes like Calimero.
  - Allows for lightweight, edge-based inference without server dependencies.
- **SentenceTransformers and Llama.cpp**:
  - Not WASM-compatible. Both require more specialized runtimes (Python for SentenceTransformers, C++ for Llama.cpp).

#### **In-Browser AI Processing**

- **ONNX Runtime in the Browser**:
  - ONNX models can be deployed directly in a browser using `onnxruntime-web`.
  - In-browser inference allows users to run models locally without needing to send data to a server.
  - **Calimero Node Integration**: While ONNX Runtime can run in the browser, deploying models in Calimero nodes (WASM runtime) provides greater flexibility for P2P networks. Nodes can handle heavier workloads, ensuring decentralized processing across the network.
- **Comparison**:
  - **Browser Deployment**: Ideal for lightweight, user-facing tasks (e.g., embeddings, simple NLP tasks).
  - **Node Deployment**: Suitable for heavier AI tasks and RAG setups within a decentralized network.

---

### **4. GGUF Models in ONNX Runtime**

While ONNX Runtime does not natively support GGUF models, it offers similar functionality through quantization techniques (e.g., INT8, FP16). These techniques reduce model size and improve inference speed, making ONNX a viable alternative to GGUF for lightweight and decentralized setups. However, GGUF models are specifically tailored for **Llama.cpp**, providing more streamlined performance for large-scale LLM tasks in fully offline environments.

---

### **Summary**

- **For Embedding Creation**: ONNX Runtime is the best choice for WASM and Rust compatibility, offering lightweight and fast inference for decentralized setups.
- **For Full Decentralization**: Llama.cpp provides unmatched independence and is ideal for environments where centralized training and dependencies are not feasible. But for our usecase it doesn't matter because we are not gonna train models in this phase.
- **For In-Browser Processing**: ONNX Runtime enables local AI processing in browsers, but deploying models in Calimero nodes offers better scalability for P2P networks.
- **Model Setup and Space**: ONNX Runtime is lighter and easier to integrate, while Llama.cpp requires more memory but delivers greater decentralization.

### Embedding Generation and AI Agent Functionality in a Calimero Setup

The three possible options to create embeddings locally are the following: SentenceTransformers, ONNX Runtime, and Llama.cpp. Additionally, embeddings can also be created in other ways, such as using the Vercel AI SDK for testing purposes. This document will also consider how the setup to create embeddings would work for full agent functionalities.

---

### **5. Pre-Trained Models for Inference**

For our use case, we are not training models but using pre-trained ones for embedding creation and full agent functionalities (e.g., RAG). This applies whether we use ONNX Runtime, Llama.cpp, or SentenceTransformers.

- **Training is separate**: Training models requires vast computational resources (GPUs, TPUs) and large datasets, which are typically handled by centralized organizations (e.g., OpenAI, Meta, Hugging Face).
- **Focus on inference**: We are deploying pre-trained models for local execution in Calimero nodes.
- **Decentralization still matters**: While Llama.cpp emphasizes full offline execution, ONNX Runtime allows for broader WASM and browser-based scenarios, making it lighter and more versatile for certain decentralized setups.

Regardless of the framework, we are focused on running pre-trained models efficiently rather than building new ones from scratch.

---

### 6 **Running E5 Locally or in a Calimero Node**

**E5**

- **What It Is**: A family of text embedding models optimized for retrieval and search tasks, developed by Microsoft.
- **Usage**:
  - Typically runs using PyTorch or TensorFlow.
  - Can be converted to ONNX for optimized inference in WASM environments like Calimero.
- **Key Advantage**: Provides high-quality embeddings, optimized for retrieval-augmented generation (RAG), making it ideal for search and ranking tasks.
- **Limitations**: Requires conversion to ONNX to run efficiently in decentralized WASM environments.

- **E5 in the Picture**: E5 is a strong choice for embedding creation due to its high-quality retrieval optimization. However, since it is not natively WASM-compatible, it needs to be converted to ONNX for execution in Calimero nodes. To run E5 locally, it can be executed using PyTorch or TensorFlow on a standard machine with a Python environment. However, since Calimero nodes operate in a WASM runtime, E5 must be converted to ONNX to run efficiently. The converted ONNX model can then be deployed inside a Calimero node and executed using ONNX Runtime for WASM, ensuring lightweight and efficient embedding generation in a decentralized network. Compared to SentenceTransformers, it provides **better search and ranking performance**, making it ideal for **retrieval-augmented generation (RAG)** scenarios in decentralized AI agents.

To run E5 locally, it can be executed using **PyTorch or TensorFlow** on a standard machine with a Python environment. However, since **Calimero nodes operate in a WASM runtime**, E5 must be **converted to ONNX** to run efficiently. The converted ONNX model can then be deployed inside a Calimero node and executed using **ONNX Runtime for WASM**, ensuring lightweight and efficient embedding generation in a decentralized network.
