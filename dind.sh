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
    -v $VOLUME_NAME:/project \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -e VOLUME_NAME=$VOLUME_NAME \
    -w $(get_work_dir) \
    $@
fi
