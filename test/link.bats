#!/usr/bin/env bats

setup() {
  load 'test_helper/bats-support/load'
  load 'test_helper/bats-assert/load'
  load 'test_helper/common_setup'

  load_script docker-compose/link.sh
  mkdir -p /storage/config
  STORAGE_NAME=config
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

@test "could link files" {
  echo "test/a
test/b/a
cache-dir:test/c
cache-file:test/d" >> /storage/config/.config
  mkdir -p /storage/config/test/b
  touch /storage/config/test/a
  touch /storage/config/test/b/c

  run link_files
  run ls -a -R /test
  assert_output "/test:
.
..
a
b
c
d

/test/b:
.
..
a"

  run touch /test/c/a
  assert_success

  run touch /test/d/a
  assert_failure

  run ls -a -R /storage
  assert_output "/storage:
.
..
cache
config

/storage/cache:
.
..
test

/storage/cache/test:
.
..
c
d

/storage/cache/test/c:
.
..
a

/storage/config:
.
..
.config
test

/storage/config/test:
.
..
a
b

/storage/config/test/b:
.
..
c"
}
