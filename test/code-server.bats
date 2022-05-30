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
