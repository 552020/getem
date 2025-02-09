import re
import json
from sentence_transformers import SentenceTransformer

# Load the pretrained model
model = SentenceTransformer("all-MiniLM-L6-v2")

# Read the full text of Divina Commedia
with open("../raw_data/dante_inferno.txt", "r", encoding="utf-8") as f:
    full_text = f.read()

# Remove Project Gutenberg header and footer
main_text = re.search(r"CANTICA I: INFERNO.*?(?=\*\*\* END)", full_text, re.DOTALL)
if main_text:
    full_text = main_text.group(0)

# Split based on "Canto" markers with Roman numerals
# This regex captures both the Canto header and its number
chunks = re.split(r"(?=Canto [IVXLCDM]+\.?)", full_text)
chunks = [chunk.strip() for chunk in chunks if chunk.strip()]

print(f"Found {len(chunks)} chunks (cantos).")

# Generate embeddings for each chunk
embeddings = model.encode(chunks)
embeddings_list = [embedding.tolist() for embedding in embeddings]

# Create a list of entries with metadata
data = []
for i, (chunk, emb_list) in enumerate(zip(chunks, embeddings_list), start=1):
    # Extract the Canto number from the first line
    canto_match = re.match(r"Canto ([IVXLCDM]+)", chunk)
    canto_num = canto_match.group(1) if canto_match else f"{i}"

    entry = {
        "chunk_id": i,
        "title": "Divina Commedia: Inferno",
        "canto": canto_num,
        "label": f"Canto {canto_num}",
        "text_excerpt": chunk[:200] + "..." if len(chunk) > 200 else chunk,
        "full_text": chunk,
        "embedding": emb_list,
    }
    data.append(entry)

# Save to a JSON file
output_filename = "dante_inferno_embeddings.json"
with open(output_filename, "w", encoding="utf-8") as f:
    json.dump(data, f, indent=2, ensure_ascii=False)

print(f"Embeddings for Dante's Inferno have been saved to {output_filename}.")
