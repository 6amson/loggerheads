#!/bin/bash

# CPU Stress Test Script
# This script creates multiple processes that consume CPU to test process monitoring

# Default values
CPU_THRESHOLD=${1:-10.0}  # CPU threshold from command line or default 10%
NUM_PROCESSES=${2:-3}     # Number of processes to spawn
DURATION=${3:-30}         # Duration in seconds

echo "Starting CPU stress test..."
echo "CPU Threshold: ${CPU_THRESHOLD}%"
echo "Number of processes: ${NUM_PROCESSES}"
echo "Duration: ${DURATION} seconds"
echo "PIDs will be saved to /tmp/stress_pids.txt"

# Clean up any existing stress processes
cleanup() {
    echo "Cleaning up stress processes..."
    if [ -f /tmp/stress_pids.txt ]; then
        while read pid; do
            if kill -0 "$pid" 2>/dev/null; then
                kill "$pid" 2>/dev/null
                echo "Killed process $pid"
            fi
        done < /tmp/stress_pids.txt
        rm -f /tmp/stress_pids.txt
    fi
    echo "Cleanup complete"
}

# Set up signal handlers
trap cleanup EXIT INT TERM

# Clear PID file
> /tmp/stress_pids.txt

# Function to create CPU intensive process
create_cpu_stress() {
    local process_id=$1
    local intensity=$2
    
    # This creates a CPU-intensive process using a busy loop
    (
        echo "Starting stress process $process_id (PID: $$) with ${intensity}% intensity"
        
        while true; do
            # Busy work for the specified intensity
            for ((i=0; i<intensity*1000; i++)); do
                : # No-op command that still consumes CPU
            done
            
            # Brief sleep to control CPU usage (adjust as needed)
            sleep 0.001
        done
    ) &
    
    local pid=$!
    echo $pid >> /tmp/stress_pids.txt
    echo "Started stress process $process_id with PID: $pid"
}

# Alternative stress function using dd and /dev/zero
create_dd_stress() {
    local process_id=$1
    
    (
        echo "Starting dd stress process $process_id (PID: $$)"
        while true; do
            dd if=/dev/zero of=/dev/null bs=1M count=100 2>/dev/null
        done
    ) &
    
    local pid=$!
    echo $pid >> /tmp/stress_pids.txt
    echo "Started dd stress process $process_id with PID: $pid"
}

# Alternative stress function using yes command
create_yes_stress() {
    local process_id=$1
    
    (
        echo "Starting yes stress process $process_id (PID: $$)"
        yes > /dev/null
    ) &
    
    local pid=$!
    echo $pid >> /tmp/stress_pids.txt
    echo "Started yes stress process $process_id with PID: $pid"
}

# Create different types of stress processes
echo "Creating stress processes..."

# Method 1: Custom busy loop (adjustable intensity)
for ((i=1; i<=NUM_PROCESSES; i++)); do
    intensity=$((15 + i * 5))  # 20%, 25%, 30%, etc.
    create_cpu_stress $i $intensity
    sleep 1  # Stagger process creation
done

# Method 2: dd-based stress (uncomment to use)
# create_dd_stress "dd_1"
# create_dd_stress "dd_2"

# Method 3: yes-based stress (uncomment to use)
# create_yes_stress "yes_1"

echo "All stress processes started. Check with 'top' or 'htop'"
echo "PIDs:"
cat /tmp/stress_pids.txt

# Wait for specified duration
echo "Running for $DURATION seconds..."
sleep $DURATION

echo "Stress test complete!"
cleanup