#!/bin/bash

TEST_DIR="/tmp/watch_test"

echo "🔍 Testing file watcher with directory: $TEST_DIR"
echo "👤 Running as user: $(whoami)"

# Remove directory if it exists (clean start)
if [ -d "$TEST_DIR" ]; then
    # echo "🧹 Removing existing test directory..."
    # rm -rf "$TEST_DIR"
    echo "📁 Test directory already exists: $TEST_DIR"
    else
    echo "📁 Creating test directory: $TEST_DIR"
    mkdir -p "$TEST_DIR"
fi



# Check mkdir exit code
if [ $? -eq 0 ]; then
    echo "✅ Directory created successfully"
else
    echo "❌ Failed to create directory (exit code: $?)"
    exit 1
fi

# Verify directory was created
if [ ! -d "$TEST_DIR" ]; then
    echo "❌ Directory doesn't exist after creation"
    exit 1
fi

echo "✅ Test directory confirmed: $TEST_DIR"
echo "📂 Directory permissions:"
ls -ld "$TEST_DIR"

echo ""
echo "🔄 Starting file operations..."

# Create events
echo "1. Creating files..."
touch "$TEST_DIR/created_file.txt"
echo "Hello World" > "$TEST_DIR/new_file.txt"

if [ -f "$TEST_DIR/created_file.txt" ] && [ -f "$TEST_DIR/new_file.txt" ]; then
    echo "✅ Files created successfully"
else
    echo "❌ Failed to create files"
    ls -la "$TEST_DIR"
fi

sleep 2

# Modify events  
echo "2. Modifying files..."
echo "Modified content" >> "$TEST_DIR/new_file.txt"
echo "Timestamp: $(date)" >> "$TEST_DIR/created_file.txt"
echo "✅ Files modified"

sleep 2

# List files to verify
echo "3. Current files in test directory:"
ls -la "$TEST_DIR"

sleep 1

# Delete events
echo "4. Deleting one file..."
rm "$TEST_DIR/created_file.txt"
rm "$TEST_DIR/new_file.txt"


if [ ! -f "$TEST_DIR/created_file.txt" ]; then
    echo "✅ File deleted successfully"
else
    echo "❌ Failed to delete file"
fi

sleep 2

# Create a few more files for batch testing
echo "5. Creating multiple files..."
for i in {1..3}; do
    echo "Content for file $i" > "$TEST_DIR/batch_file_$i.txt"
    echo "📄 Created batch_file_$i.txt"
    sleep 0.5
done

sleep 1

# Delete batch files
echo "6. Deleting batch files..."
rm "$TEST_DIR"/batch_file_*.txt
echo "✅ Batch files deleted"

sleep 1

echo "7. Final directory contents:"
ls -la "$TEST_DIR"

sleep 1

echo "8. Final cleanup..."
rm -rf "$TEST_DIR"

if [ ! -d "$TEST_DIR" ]; then
    echo "✅ Test directory cleaned up"
else
    echo "❌ Failed to clean up test directory"
fi

echo ""
echo "🎉 File watcher test complete!"
echo "📊 Summary: Created files, modified files, deleted files"
echo "🔍 Check your Rust application output for detected events"