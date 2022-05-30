#!/usr/bin/env bash

load_script() {
  source "$(cd $(dirname $BATS_TEST_FILENAME) >/dev/null 2>&1 && pwd)/../$1"
}
