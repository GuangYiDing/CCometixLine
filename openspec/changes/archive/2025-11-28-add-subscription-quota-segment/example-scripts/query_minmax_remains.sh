#!/usr/bin/env bash
if [ -z "$ANTHROPIC_AUTH_TOKEN" ]; then
  echo "请先设置 ANTHROPIC_AUTH_TOKEN 环境变量"
  exit 1
fi

# ---- 请求接口 ----
resp=$(curl -s --location --request GET \
  "https://www.minimaxi.com/v1/api/openplatform/coding_plan/remains" \
  --header "authorization: Bearer ${ANTHROPIC_AUTH_TOKEN}"
)

# ---- 解析真实字段 ----
start_ms=$(echo "$resp" | jq '.model_remains[0].start_time')
end_ms=$(echo "$resp" | jq '.model_remains[0].end_time')
remains_ms=$(echo "$resp" | jq '.model_remains[0].remains_time')
# 当前时间窗口的配额总量
total=$(echo "$resp" | jq '.model_remains[0].current_interval_total_count')
# 当前时间窗口的剩余可使用量
remaining=$(echo "$resp" | jq '.model_remains[0].current_interval_usage_count')
# 计算已使用量
used=$(($total - $remaining))
model_name=$(echo "$resp" | jq -r '.model_remains[0].model_name')

# ---- 毫秒 → 秒 ----
start=$((start_ms / 1000))
end=$((end_ms / 1000))
remains=$((remains_ms / 1000))

# ---- 兼容 mac / Linux 的时间格式函数 ----
fmt_time() {
  local ts="$1"
  if date -d @"$ts" "+%H:%M" >/dev/null 2>&1; then
    date -d @"$ts" "+%H:%M"      # Linux / GNU date
  else
    date -r "$ts" "+%H:%M"       # macOS / BSD date
  fi
}

start_str=$(fmt_time "$start")
end_str=$(fmt_time "$end")

# ---- 重置倒计时 ----
h=$((remains / 3600))
m=$(((remains % 3600) / 60))
countdown="${h}h${m}m"

# ---- 当前时间 ----
now=$(date "+%H:%M")

# ---- 计算已使用百分比 ----
percentage=$(awk "BEGIN { printf \"%d\", ($used/$total)*100 }")

# ---- 文本进度条（10 格）----
bar_length=10
filled=$((percentage * bar_length / 100))
empty=$((bar_length - filled))

filled_char="█"
empty_char="░"

progress_bar=""
for ((i=0; i<filled; i++)); do
  progress_bar="${progress_bar}${filled_char}"
done
for ((i=0; i<empty; i++)); do
  progress_bar="${progress_bar}${empty_char}"
done

# ---- 颜色：低绿 / 中橙 / 高红 ----
GREEN="\033[32m"
YELLOW="\033[33m"
RED="\033[31m"
RESET="\033[0m"

if [ "$percentage" -lt 60 ]; then
  color="$GREEN"
elif [ "$percentage" -lt 85 ]; then
  color="$YELLOW"
else
  color="$RED"
fi

colored_bar="${color}${progress_bar}${RESET}"
colored_percentage="${color}${percentage}%${RESET}"

# ---- 状态栏文本（彩色） ----
# 示例：💎 ██████ 45% | 📊 2000/4500 | ⏰ 4h50m
text_output="${colored_bar} ${colored_percentage} | ${used}/${total} | 🔄 ${countdown}"

# 也顺带保留一份纯文本（无颜色），方便别的地方复用
plain_text=" ${progress_bar} ${percentage}% | 📊 ${used}/${total} | 🔄 ${countdown}"

# ---- 文本 输出 ----
echo "$plain_text"
