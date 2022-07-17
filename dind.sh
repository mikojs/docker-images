#!/bin/bash

check_volume_name() {
  if [[ -z $VOLUME_NAME ]]; then
    printf "couldn't find the volume name"
    exit 1
  fi
}

get_work_dir() {
  if [[ $(pwd) =~ ^/project ]]; then
    pwd
  else
    printf "/project"
  fi
}

if [[ -z $BATS_TEST_FILENAME ]]; then
  check_volume_name

  docker run \
    -it \
    -v $VOLUME_NAME:/project \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -e VOLUME_NAME=$VOLUME_NAME \
    -w $(get_work_dir) \
    $@
fi
