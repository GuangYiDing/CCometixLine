#!/bin/bash
# Example script to check AWS service quota
# This script returns usage percentage in JSON format

# In production, you would use AWS CLI to get service quotas
# Example with AWS CLI:
# aws service-quotas get-service-quota --service-code ec2 --quota-code L-0263D53A --query 'Quota.Value' --output text

# For demonstration, return JSON format
# This includes both percentage and a secondary text field

# Generate a more deterministic example (could use actual AWS API in production)
percentage=65
remaining=$(echo "scale=1; 1000 - (1000 * $percentage / 100)" | bc)

cat <<EOF
{
  "percentage": $percentage,
  "remaining": "${remaining} units"
}
EOF
