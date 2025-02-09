Here is a similar documentation structure for creating and running nodes, installing an app, setting up contexts, and using external methods through `meroctl`, based on the commands you've shared:

---

### **Steps to Set Up and Run Nodes**

#### 1️⃣ **Create and Run Nodes**
To initialize and run two or more nodes:

```sh
# Initialize Node 1
merod --node-name node1 init --server-port 2427 --swarm-port 2527

# Initialize Node 2
merod --node-name node2 init --server-port 2428 --swarm-port 2528

# Run Node 1
merod --node-name node1 run

# Run Node 2
merod --node-name node2 run
```

#### 2️⃣ **Install an App on One Node**
Install the blockchain application on a specific node (e.g., node1):

```sh
application install file <PATH_TO_blockchain.wasm_FILE>
```

**Response:**
```
Installed application: <APPLICATION_ID>
```

#### 3️⃣ **Create a New Context**
Create a new context for the application with the specified protocol (`icp` in this case):

```sh
context create <APPLICATION_ID> --protocol icp
```

**Response:**
```
Created context <CONTEXT_ID> with identity <CONTEXT_IDENTITY>
```

#### 4️⃣ **Create a New Identity for Node 2**
Generate a new identity for `node2`:

```sh
identity new
```

**Response:**
```
Private key: <PRIVATE_KEY>
Public key: <PUBLIC_KEY>
```

#### 5️⃣ **Invite Node 2 to the Context from Node 1**
Send an invitation from `node1` to `node2` for joining the context:

```sh
context invite <CONTEXT_ID> <CONTEXT_IDENTITY> <PUBLIC_KEY_OF_NODE2>
```

**Response:**
```
Invitation payload: <INVITATION_PAYLOAD>
```

#### 6️⃣ **Accept Invitation from Node 2**
Accept the invitation on `node2` by providing the private key and the invitation payload:

```sh
context join <PRIVATE_KEY_OF_NODE2> <INVITATION_PAYLOAD>
```

---

### **Retrieve Values from the Nodes**

To view the status and details of contexts and identities, you can use the following commands:

#### 1️⃣ **List Contexts**
List all contexts available on a node:

```sh
context ls
```

#### 2️⃣ **List Identities for a Context**
List all identities associated with a given context:

```sh
identity ls <CONTEXT_ID>
```
