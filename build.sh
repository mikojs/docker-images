#!/bin/sh

if [ ! -d config ]; then
  mkdir config
fi

for file in $(ls); do
  if [ -f $file/Dockerfile ]; then
    if [ ! -d config/$file ]; then
      mkdir config/$file
    fi

    echo "[$file] start"
    docker build -t $file -f ./$file/Dockerfile .
    echo "[$file] end"
  fi
done
