#!/bin/sh

generate_password() {
  local password
  
  if [[ -z "$PASSWORD" ]]; then
    password=$(openssl rand -base64 45)
  else
    password=$PASSWORD
  fi

  echo $password
}

code_server() {
  if [[ ! -z "$SLACK_BOT_TOKEN" ]] && [[ ! -z "$SLACK_BOT_CHANNEL" ]]; then
    result=$(curl \
      -X POST \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $SLACK_BOT_TOKEN" \
      -d "{\"channel\": \"$SLACK_BOT_CHANNEL\", \"text\": \"$password\" }" \
      https://slack.com/api/chat.postMessage)

    if [[ ! -z $(echo $result | grep -v "\"ok\":true") ]]; then
      echo "could not send the password to the slack channel"
      exit 1
    fi
  else
    echo "code-server: $password"
  fi

  PASSWORD=$password code-server
}
