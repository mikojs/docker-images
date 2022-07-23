#!/usr/bin/env bash

local dir=$(cd $(dirname $BATS_TEST_FILENAME) >/dev/null 2>&1 && pwd)
local filename=$(basename $BATS_TEST_FILENAME | sed 's/.bats/.sh/g')

BATS_TEST_DIRNAME=$dir
source "$dir/../$filename"
