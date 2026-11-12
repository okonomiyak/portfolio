#!/usr/bin/env bash
# dx serve を起動しつつ、styles/main.scss を一定間隔で「保存」(touch) する
# 使い方: ./dev.sh [dx serve の引数...]
#         INTERVAL=3 ./dev.sh   (間隔を秒で変更。デフォルト20秒)

# どこから実行してもプロジェクトルート(このファイルの1つ上)で動かす
cd "$(dirname "$0")/.." || exit 1

FILE="styles/main.scss"
INTERVAL="${INTERVAL:-20}"

touch_loop() {
  local tmp
  tmp="$(mktemp)"
  while true; do
    sleep "$INTERVAL"
    cp "$FILE" "$tmp" && cat "$tmp" > "$FILE"
  done
}

touch_loop &
LOOP_PID=$!
trap 'kill "$LOOP_PID" 2>/dev/null' EXIT INT TERM

dx serve "$@"