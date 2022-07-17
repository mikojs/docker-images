#!/usr/bin/env bats

setup() {
  load 'test_helper/bats-support/load'
  load 'test_helper/bats-assert/load'
  load 'test_helper/common_setup'
}

@test "check volume name" {
  run check_volume_name
  assert_failure
}

@test "the current directory is under the product directory" {
  pwd() {
    printf "/project/a/b/c"
  }

  run get_work_dir
  assert_output "/project/a/b/c"
}

@test "the current directory isn't under the product directory" {
  pwd() {
    printf "/root/a/b/c"
  }

  run get_work_dir
  assert_output "/project"
}
