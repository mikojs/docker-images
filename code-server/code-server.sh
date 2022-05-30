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

send_slack_or_echo() {
  if [[ ! -z "$SLACK_BOT_TOKEN" ]] && [[ ! -z "$SLACK_BOT_CHANNEL" ]]; then
    result=$(curl \
      -X POST \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $SLACK_BOT_TOKEN" \
      -d "{\"channel\": \"$SLACK_BOT_CHANNEL\", \"text\": \"$1\" }" \
      https://slack.com/api/chat.postMessage)

    if [[ ! -z $(echo $result | grep -v "\"ok\":true") ]]; then
      echo "could not send the password to the slack channel"
      exit 1
    fi
  else
    echo "code-server: $1"
  fi
}

if [[ -z $BATS_TEST_FILENAME ]]; then
  local password=$(generate_password)

  send_slack_or_echo $password
  PASSWORD=$password code-server
fi
