Here’s a list of the available external methods (callable through `meroctl`) along with an example for each:  

---

### **Available Methods**
1. **`upload_file(name: String, content: String, owner: String) -> bool`**  
   Uploads a file with a given name and content, assigning ownership.

2. **`download_file(name: String, downloader: String) -> Option<String>`**  
   Retrieves a file’s content if it exists.

3. **`delete_file(name: String, requester: String) -> bool`**  
   Deletes a file if the requester is the owner.

4. **`list_files() -> Vec<String>`**  
   Returns a list of all stored file names.

---

### **Example `meroctl` Commands**

#### 1️⃣ Upload a file  
```sh
meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> upload_file --args '{
    "name": "example.txt",
    "content": "This is a test file",
    "owner": "user123"
}'
```

#### 2️⃣ Download a file  
```sh
meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> download_file --args '{
    "name": "example.txt",
    "downloader": "user123"
}'
```

#### 3️⃣ Delete a file  
```sh
meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> delete_file --args '{
    "name": "example.txt",
    "requester": "user123"
}'
```

#### 4️⃣ List all files  
```sh
meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> list_files --args '{}'
```

These commands interact with the `FileExchangeState` contract, allowing users to store and retrieve files efficiently. Let me know if you need further modifications! 🚀