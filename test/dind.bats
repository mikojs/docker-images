#!/usr/bin/env bats

setup() {
  load 'test_helper/bats-support/load'
  load 'test_helper/bats-assert/load'
  load 'test_helper/common_setup'

  DIND_FOLDER=$BATS_TEST_DIRNAME/plugins_folder
  mkdir $BATS_TEST_DIRNAME/plugins_folder
}

teardown() {
  rm -rf $BATS_TEST_DIRNAME/plugins_folder
}

@test "load plugins" {
  printf "echo test" > $BATS_TEST_DIRNAME/plugins_folder/a.sh

  run load_plugins
  assert_output "test"
}
