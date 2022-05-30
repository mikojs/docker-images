#!/usr/bin/env bats

setup() {
  load 'test_helper/bats-support/load'
  load 'test_helper/bats-assert/load'
  load 'test_helper/common_setup'
  
  load_script docker-compose/link.sh
  mkdir -p /storage/config
}

teardown() {
  rm -rf /storage/config
}

@test "could not clone config with empty CONFIG_URL" {
  rm -rf /storage/config

  run clone_config
  assert_failure
  assert_output "could not get the url of the config repository"
}

@test "skip to clone config when config folder exists" {
  clone_config

  run ls -a /storage/config
  assert_output ".
.."
}
