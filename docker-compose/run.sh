#!/bin/bash

if [ ! -f ./.env ]; then
  for env_name in NGROK_AUTHTOKEN SLACK_BOT_TOKEN SLACK_BOT_CHANNEL; do
    echo "$env_name=$(printenv $env_name)" >> .env
  done
fi

docker-compose -f ./run.yml up -d
