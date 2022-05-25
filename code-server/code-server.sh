#!/bin/sh

password=$(openssl rand -base64 45)
result=$(curl \
  -X POST \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $SLACK_BOT_TOKEN" \
  -d "{\"channel\": \"$SLACK_BOT_CHANNEL\", \"text\": \"$password\" }" \
  https://slack.com/api/chat.postMessage)

if [ ! -z $(echo $result | grep -v "\"ok\":true") ]; then
  exit 1
fi

PASSWORD=$password code-server
