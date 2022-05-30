#!/usr/bin/env bats

setup() {
  load 'test_helper/bats-support/load'
  load 'test_helper/bats-assert/load'
  load 'test_helper/common_setup'
  
  load_script code-server/code-server.sh
}

@test "generate random password" {
  run generate_password
  refute_output 123456
}

@test "generate custom password" {
  PASSWORD=123456

  run generate_password
  assert_output 123456
}

@test "send to slack" {
  SLACK_BOT_TOKEN="test"
  SLACK_BOT_CHANNEL="test"
  curl() {
    echo "\"ok\":true"
  }

  run send_slack_or_echo 123456
  refute_output "code-server: 123456"
}

@test "echo password" {
  SLACK_BOT_TOKEN=""
  SLACK_BOT_CHANNEL=""
  curl() {
    exit 1
  }

  run send_slack_or_echo 123456
  assert_output "code-server: 123456"
}
