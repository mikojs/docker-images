#!/bin/bash

remove_stopped_container() {
  local stopped_containers=($(docker ps -aq -f status=exited | tr '\n' ' '))

  if [[ ${#stopped_containers[@]} != 0 ]]; then
    docker rm ${stopped_containers[*]}
  fi
}

remove_none_images() {
  local none_images=($(docker images -q -f dangling=true | tr '\n' ' '))

  if [[ ${#none_images[@]} != 0 ]]; then
    docker rmi ${none_images[*]}
  fi
}

run_container() {
  local work_dir

  if [[ $(pwd) =~ ^/project ]]; then
    work_dir=$(pwd)
  else
    work_dir="/project"
  fi

  docker run \
    -it \
    --volumes-from $(cat /etc/hostname) \
    -w $work_dir \
    $@
}
