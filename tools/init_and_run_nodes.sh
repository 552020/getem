#!/bin/bash
set -e

# Configuration
APPLICATION_WASM_PATH="./src/backend/res/hello_app.wasm"
SESSION_NAME="calimero_nodes"
base_dir="$HOME/.calimero"
USE_TMUX=1

# Global variables for context and keys
member_public_key=""
context_id=""
app_id=""

# Global variables for node keys
node2_public_key=""
node2_private_key=""
node3_public_key=""
node3_private_key=""

# Define node names and their respective ports
nodes=("node1" "node2" "node3")
server_ports=(2427 2428 2429)
swarm_ports=(2527 2528 2529)

save_configuration() {
    local app_id=$1
    local context_id=$2
    local member_public_key=$3
    local output_file=".env"

    echo "Saving configuration to $output_file..."
    
    # Create or overwrite the env file
    cat > "$output_file" << EOL
# Calimero Node Configuration
CALIMERO_APP_ID='${app_id}'
CALIMERO_CONTEXT_ID='${context_id}'
CALIMERO_MEMBER_PUBLIC_KEY='${member_public_key}'

# Node Configuration
CALIMERO_NODE1_NAME='${nodes[0]}'
CALIMERO_NODE2_NAME='${nodes[1]}'
CALIMERO_NODE3_NAME='${nodes[2]}'

# Port Configuration
CALIMERO_SERVER_PORTS='${server_ports[*]}'
CALIMERO_SWARM_PORTS='${swarm_ports[*]}'

# Node Public Keys
CALIMERO_NODE2_PUBLIC_KEY='${node2_public_key}'
CALIMERO_NODE3_PUBLIC_KEY='${node3_public_key}'

# Generated on: $(date)
EOL

    echo "Configuration saved successfully to $output_file"
}

# Function to generate invitation payloads
generate_invitation_payloads() {
    local context_id=$1
    local member_public_key=$2
    
    echo "DEBUG: Starting generate_invitation_payloads" >&2
    echo "DEBUG: context_id: $context_id" >&2
    echo "DEBUG: member_public_key: $member_public_key" >&2
    
    for i in {1..2}; do
        echo "DEBUG: Processing node index $i (${nodes[$i]})" >&2
        
        # Debug print the stored public key
        echo "DEBUG: Looking for node$((i+1))_public_key" >&2
        eval "node_public_key=\$node$((i+1))_public_key"
        echo "DEBUG: Retrieved public key: $node_public_key" >&2
        
        echo "Generating invitation payload for ${nodes[$i]}..." >&2
        
        if [ -z "$node_public_key" ]; then
            echo "ERROR: Empty public key for ${nodes[$i]}" >&2
            continue
        fi
        
        invitation_payload=$(meroctl --node-name ${nodes[0]} --output-format json context invite "$context_id" "$member_public_key" "$node_public_key")
        echo "DEBUG: Generated payload: $invitation_payload" >&2
        
        # Save the invitation payload for later use
        eval "node${i}_payload=\$invitation_payload"
    done
}

# Function to invite nodes to join
invite_nodes() {
    local context_id=$1
    
    echo "DEBUG: Starting invite_nodes" >&2
    echo "DEBUG: context_id: $context_id" >&2
    
    for i in {1..2}; do
        echo "DEBUG: Processing node index $i (${nodes[$i]})" >&2
        
        # Debug print the stored private key
        echo "DEBUG: Looking for node$((i+1))_private_key" >&2
        eval "node_private_key=\$node$((i+1))_private_key"
        echo "DEBUG: Retrieved private key: $node_private_key" >&2
        
        eval "payload=\$node${i}_payload"
        echo "DEBUG: Retrieved payload: $payload" >&2
        
        encoded_invitation=$(echo "$payload" | grep -o '"data":"[^"]*"' | cut -d'"' -f4)
        echo "DEBUG: Encoded invitation: $encoded_invitation" >&2
        
        join_output=$(meroctl --node-name ${nodes[$i]} context join "$node_private_key" "$encoded_invitation")
        echo "${nodes[$i]} join output:" >&2
        echo "$join_output" >&2
    done
}

# Function to install the application
# It is necessary to redirect the output to stderr to avoid it being captured by the caller
install_application() {
    echo "Installing application on ${nodes[0]}..." >&2
    echo "pwd: $(pwd)" >&2
    echo "APPLICATION_WASM_PATH: $APPLICATION_WASM_PATH" >&2
    full_output=$(meroctl --node-name ${nodes[0]} app install -p "$APPLICATION_WASM_PATH")
    echo "Full command output:" >&2
    echo "$full_output" >&2

    # Extract the ID - be more precise with the extraction
    app_id=$(echo "$full_output" | grep "^id:" | awk '{print $2}' | tr -d '\n\r')
    echo "Application installed successfully!" >&2
    echo "Application ID: $app_id" >&2
    # Return the app_id on stdout
    echo "$app_id"
}

# Function to create context
create_context() {
    local input_app_id=$1
    echo "Creating context..." >&2
    context_output=$(meroctl --node-name ${nodes[0]} context create --application-id "$input_app_id" --protocol "icp")
    echo "Context creation output:" >&2
    echo "$context_output" >&2

    # Set global variables
    context_id=$(echo "$context_output" | grep "^id:" | awk '{print $2}' | tr -d '\n\r')
    member_public_key=$(echo "$context_output" | grep "^member_public_key:" | awk '{print $2}' | tr -d '\n\r')
}

# Function to create identities for other nodes
create_identities() {
    for i in {1..2}; do
        echo "DEBUG: Creating identity for node index $i (${nodes[$i]})" >&2
        node_output=$(meroctl --node-name "${nodes[$i]}" identity generate)
        
        # Store into node2_public_key for i=1 and node3_public_key for i=2
        eval "node$((i+1))_public_key=\$(echo \"$node_output\" | grep \"public_key:\" | awk '{print \$2}')"
        eval "node$((i+1))_private_key=\$(echo \"$node_output\" | grep \"private_key:\" | awk '{print \$2}')"
        
        # Debug prints
        echo "DEBUG: For ${nodes[$i]}:" >&2
        echo "DEBUG: Stored in node$((i+1))_public_key: $(eval echo \$node$((i+1))_public_key)" >&2
        echo "DEBUG: Stored in node$((i+1))_private_key: $(eval echo \$node$((i+1))_private_key)" >&2
        echo "---" >&2
    done
}

# Function to check if tmux is installed
check_tmux() {
    if ! command -v tmux >/dev/null 2>&1; then
        echo "tmux is not installed. Would you like to:"
        echo "1) Exit and install tmux manually (recommended)"
        echo "2) Continue without tmux (will open separate terminal windows)"
        read -p "Enter your choice (1 or 2): " choice
        case $choice in
            1)
                echo "Please install tmux and try again:"
                echo "  - On MacOS: brew install tmux"
                echo "  - On Ubuntu/Debian: sudo apt-get install tmux"
                echo "  - On other systems: use your package manager to install tmux"
                exit 1
                ;;
            2)
                echo "Proceeding without tmux..."
                export USE_TMUX=0
                return
                ;;
            *)
                echo "Invalid choice. Exiting."
                exit 1
                ;;
        esac
    fi
    export USE_TMUX=1
}

# Function to check ports
check_ports() {
  for port in "${server_ports[@]}" "${swarm_ports[@]}"; do
    if lsof -i:"$port" &>/dev/null; then
      echo "Port $port is in use."
      read -p "Kill the process using port $port? (y/n): " choice
      if [[ $choice == "y" || $choice == "Y" ]]; then
        lsof -ti:"$port" | xargs kill -9
        echo "Process on port $port killed."
      else
        echo "Port $port is still in use. Exiting."
        exit 1
      fi
    else
      echo "Port $port is free."
    fi
  done
}

# Function to check if node directories exist
check_node_dirs() {
    for node in "${nodes[@]}"; do
        node_dir="$base_dir/$node"
        if [ -d "$node_dir" ]; then
            echo "Directory for $node already exists at $node_dir."
            read -p "Do you want to reinitialize $node? (y/n): " choice
            if [[ $choice == "y" || $choice == "Y" ]]; then
                echo "Removing and reinitializing $node..."
                rm -rf "$node_dir"
                initialize_node "$node"
            else
                echo "Skipping initialization of $node."
                continue
            fi
        else
            echo "No existing directory found for $node. Proceeding with initialization."
            initialize_node "$node"
        fi
    done
}

# Function to initialize a node
initialize_node() {
    local node=$1
    local index=$(echo "${nodes[@]}" | tr ' ' '\n' | grep -n "^$node$" | cut -d: -f1)
    local server_port=${server_ports[$((index - 1))]}
    local swarm_port=${swarm_ports[$((index - 1))]}

    echo "Initializing $node on server port $server_port and swarm port $swarm_port."
    mkdir -p "$base_dir/$node"  # Ensure the directory exists
    
    if [ "$USE_TMUX" -eq 1 ]; then
        tmux send-keys -t ${SESSION_NAME}:script "merod --node-name $node init --server-port $server_port --swarm-port $swarm_port" C-m
    else
        # For non-tmux, just run the init command directly
        merod --node-name $node init --server-port $server_port --swarm-port $swarm_port
    fi
}

# Function to run nodes
run_nodes() {
    if [ "$USE_TMUX" -eq 1 ]; then
        # Setup nodes in separate windows
        for node in "${nodes[@]}"; do
            node_dir="$base_dir/$node"
            if [ ! -d "$node_dir" ]; then
                echo "Error: Directory for $node does not exist at $node_dir. Please initialize the node first."
                exit 1
            fi

            echo "Starting $node in a new tmux window..."
            tmux new-window -t ${SESSION_NAME} -n "$node"
            tmux send-keys -t ${SESSION_NAME}:"$node" "cd \"$node_dir\" && merod --node-name $node run" C-m
        done
        
        # Switch back to script window
        tmux select-window -t ${SESSION_NAME}:script
        
        echo "Nodes are running in other windows. Use Ctrl-b n/p to switch between windows"
    else
        # Non-tmux version - open new terminal windows
        for node in "${nodes[@]}"; do
            node_dir="$base_dir/$node"
            if [ ! -d "$node_dir" ]; then
                echo "Error: Directory for $node does not exist at $node_dir. Please initialize the node first."
                exit 1
            fi

            echo "Starting $node in a new terminal window..."
            if [[ "$OSTYPE" == "darwin"* ]]; then
                # macOS
                osascript -e "tell app \"Terminal\" to do script \"merod --node-name $node run\""
            else
                # Linux (assuming x-terminal-emulator is available)
                x-terminal-emulator -e "merod --node-name $node run" &
            fi
        done
        
        echo "Nodes started in separate terminal windows."
        echo "Please keep these windows open to maintain node operation."
    fi
}

# Main script execution
check_tmux
check_ports
check_node_dirs

echo "Current tmux sessions before starting nodes:"
tmux ls || echo "No tmux sessions exist"

# Create the initial tmux session if using tmux
if [ "$USE_TMUX" -eq 1 ]; then
    echo "Creating new tmux session: ${SESSION_NAME}"
    tmux new-session -d -s ${SESSION_NAME} -n "script"
    echo "Session created. Current sessions:"
    tmux ls
fi

# Run nodes only if using tmux
if [ "$USE_TMUX" -eq 1 ]; then
    echo "Starting nodes in tmux windows..."
    echo "SESSION_NAME is: ${SESSION_NAME}"
    run_nodes
else
    echo "Starting nodes in separate terminal windows..."
    run_nodes
fi

# Add delay to ensure nodes are ready
echo "Waiting for nodes to initialize..."
sleep 3  # Wait 10 seconds for nodes to start up

# After nodes are running, setup the application
echo "Checking if node1 is ready..."
max_retries=5
retry_count=0
while ! curl -s http://127.0.0.1:2427/health > /dev/null; do
    if [ $retry_count -ge $max_retries ]; then
        echo "Error: Node1 failed to start after $max_retries attempts"
        exit 1
    fi
    echo "Node1 not ready yet, waiting..."
    sleep 5
    retry_count=$((retry_count + 1))
done

echo "Nodes are ready, proceeding with application setup..."

# After nodes are running, setup the application
echo "Starting application installation..."
app_id=$(install_application)
echo "Application ID: $app_id"

# Simply call create_context without capturing its output
create_context "$app_id"
echo "Context ID: $context_id"
echo "Member Public Key: $member_public_key"


# **Call create_identities to generate keys for node2 and node3**
create_identities

# Now both global variables are properly set
generate_invitation_payloads "$context_id" "$member_public_key"
invite_nodes "$context_id"

# Save the configuration
save_configuration "$app_id" "$context_id" "$member_public_key"

echo "Application setup completed:"
echo "Application ID: $app_id"
echo "Context ID: $context_id"
