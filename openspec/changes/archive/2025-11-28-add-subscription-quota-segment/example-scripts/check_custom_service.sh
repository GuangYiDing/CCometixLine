#!/bin/bash
# Example script template for custom service quota
# Adapt this for your specific service

# Your script should output ONE of the following formats:

# 1. TEXT format (simple number):
# 85

# 2. JSON format:
# {
#   "percentage": 85,
#   "remaining": "150 requests"
# }

# 3. Key-Value format:
# percentage=85
# remaining=150 requests

# Instructions:
# 1. Replace this script with your actual service check
# 2. Update your configuration with:
#    - script_path: "/path/to/this/script.sh"
#    - output_format: "text", "json", or "key_value"
# 3. Make the script executable: chmod +x /path/to/this/script.sh

# Example implementation for a hypothetical API:
# API_RESPONSE=$(curl -s "https://api.yourservice.com/quota?key=$API_KEY")
# echo "$API_RESPONSE" | jq -r '.usage_percentage'
