#!/bin/bash

if [[ ! -d config ]]; then
  printf "The url of the config repository: "
  read url
  git clone $url config
fi

for file in $(ls); do
  if [[ -f $file/Dockerfile ]]; then
    docker build -t $file -f ./$file/Dockerfile .
  fi
done
