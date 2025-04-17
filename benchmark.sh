#!/bin/bash

# Create a markdown report file
REPORT_FILE="benchmark_report.md"
echo "# Compression Benchmark Report" > $REPORT_FILE
echo -e "\n## Test Results\n" >> $REPORT_FILE

# Function to get file size in bytes
get_file_size() {
    stat -c %s "$1" 2>/dev/null
}

# Function to run docker command and capture time
run_docker_cmd() {
    { time -p "$@" > /dev/null; } 2>&1
}

# Test file
TEST_FILE="test.txt"
COMPRESSED_FILE="test.rle.compressed"
DECOMPRESSED_FILE="test.decompressed"

# Get current user and group IDs
USER_ID=$(id -u)
GROUP_ID=$(id -g)

# Original file size
ORIGINAL_SIZE=$(get_file_size "$TEST_FILE")
if [ -z "$ORIGINAL_SIZE" ]; then
    echo "Error: Test file not found"
    exit 1
fi

# Cleanup any existing files
rm -f "$COMPRESSED_FILE" "$DECOMPRESSED_FILE"

# Compression test
echo "### Compression Test" >> $REPORT_FILE
COMPRESS_TIME=$(run_docker_cmd docker run --rm -v "$(pwd)":/data --workdir /data --user $USER_ID:$GROUP_ID rust-compressor rle compress "$TEST_FILE")
sleep 1  # Give the filesystem a moment to sync

COMPRESSED_SIZE=$(get_file_size "$COMPRESSED_FILE")
if [ -z "$COMPRESSED_SIZE" ]; then
    echo "Error: Compression failed"
    exit 1
fi
COMPRESSION_RATIO=$(echo "scale=2; $COMPRESSED_SIZE / $ORIGINAL_SIZE * 100" | bc)

echo "- Original file size: $ORIGINAL_SIZE bytes" >> $REPORT_FILE
echo "- Compressed file size: $COMPRESSED_SIZE bytes" >> $REPORT_FILE
echo "- Compression ratio: $COMPRESSION_RATIO%" >> $REPORT_FILE
echo "- Compression time: $(echo "$COMPRESS_TIME" | grep real | awk '{print $2}') seconds" >> $REPORT_FILE

# Decompression test
echo -e "\n### Decompression Test" >> $REPORT_FILE
DECOMPRESS_TIME=$(run_docker_cmd docker run --rm -v "$(pwd)":/data --workdir /data --user $USER_ID:$GROUP_ID rust-compressor rle decompress "$COMPRESSED_FILE")
sleep 1  # Give the filesystem a moment to sync

DECOMPRESSED_SIZE=$(get_file_size "$DECOMPRESSED_FILE")
if [ -z "$DECOMPRESSED_SIZE" ]; then
    echo "Error: Decompression failed"
    exit 1
fi

echo "- Decompression time: $(echo "$DECOMPRESS_TIME" | grep real | awk '{print $2}') seconds" >> $REPORT_FILE
echo "- Decompressed file size: $DECOMPRESSED_SIZE bytes" >> $REPORT_FILE

# Verify integrity
if cmp -s "$TEST_FILE" "$DECOMPRESSED_FILE"; then
    echo "- Integrity check: ✅ Files match" >> $REPORT_FILE
else
    echo "- Integrity check: ❌ Files do not match" >> $REPORT_FILE
fi

# Cleanup
rm -f "$COMPRESSED_FILE" "$DECOMPRESSED_FILE"

echo -e "\nBenchmark report generated in $REPORT_FILE" 