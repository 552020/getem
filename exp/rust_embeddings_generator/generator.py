from sentence_transformers import SentenceTransformer
import json

# Load the pretrained all-MiniLM-L6-v2 model
model = SentenceTransformer("all-MiniLM-L6-v2")

# Define the sentences you want to embed
sentences = [
    "This is an example sentence.",
    "Sentence embeddings are really useful.",
    "How well does this model capture meaning?",
]

# Generate embeddings for the sentences
embeddings = model.encode(sentences)

# Convert each embedding (a NumPy array) to a list
embeddings_list = [embedding.tolist() for embedding in embeddings]

# Create a dictionary to hold your data
data = {"sentences": sentences, "embeddings": embeddings_list}

# Save the dictionary to a JSON file
with open("embeddings.json", "w") as f:
    json.dump(data, f, indent=2)

# Print the embeddings
for sentence, embedding in zip(sentences, embeddings):
    print(f"Sentence: {sentence}")
    print(f"Embedding (shape {embedding.shape}):\n{embedding}\n")
