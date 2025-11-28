#!/bin/bash
# Example script to check GitHub API rate limit
# This script returns usage percentage in key-value format

# GitHub API example (requires GITHUB_TOKEN):
# curl -s -H "Authorization: token $GITHUB_TOKEN" \
#   https://api.github.com/rate_limit | \
#   jq -r '"rate_limit=\((.rate.remaining * 100) / .rate.limit)"'

# For demonstration, return key-value format
# In production, parse actual API response

echo "percentage=78"
