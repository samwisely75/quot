#!/bin/bash

# Quick test script for release binaries
# This script demonstrates the functionality of quot

set -e

echo "🧪 Testing quot functionality..."
echo

# Test data
TEST_INPUT="Hello \"World\"
Line 2 with\ttab
And a backslash: \\
End"

echo "📝 Test input:"
echo "$TEST_INPUT"
echo

echo "🔸 Testing double quotes (default):"
echo "$TEST_INPUT" | ./quot

echo
echo "🔸 Testing single quotes:"
echo "$TEST_INPUT" | ./quot --single

echo
echo "🔸 Testing raw strings:"
echo "$TEST_INPUT" | ./quot --raw

echo
echo "✅ All tests completed!"
