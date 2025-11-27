#!/usr/bin/env bash
if [ -z "$ANTHROPIC_AUTH_TOKEN" ]; then
  echo "请先设置 ANTHROPIC_AUTH_TOKEN 环境变量"
  exit 1
fi

# 请求接口
resp=$(curl -s --location --request GET \
  "https://www.minimaxi.com/v1/api/openplatform/coding_plan/remains" \
  --header "authorization: Bearer ${ANTHROPIC_AUTH_TOKEN}"
)

# 解析字段
total=$(echo "$resp" | jq '.model_remains[0].current_interval_total_count')
usage=$(echo "$resp" | jq '.model_remains[0].current_interval_usage_count')

# 计算
percentage=$(awk "BEGIN { printf \"%d\", ($usage/$total)*100 }")
remaining=$(awk "BEGIN { printf \"%d\", $total-$usage }")
totals=$(awk "BEGIN { printf \"%d\", $total }")

# 输出 JSON
cat <<EOF
{
  "percentage": $percentage,
  "remaining": "$remaining",
  "total": "$totals"
}
EOF
