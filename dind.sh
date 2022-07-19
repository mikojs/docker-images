#!/bin/bash

get_work_dir() {
  if [[ $(pwd) =~ ^/project ]]; then
    pwd
  else
    printf "/project"
  fi
}

if [[ -z $BATS_TEST_FILENAME ]]; then
  docker run \
    -it \
    --volumes-from $(cat /etc/hostname) \
    -w $(get_work_dir) \
    $@
fi
