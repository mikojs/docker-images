#!/bin/sh

if [ $# -eq 0 ]; then
  files=$(ls)
else
  files=$@
fi

if [ ! -d config ]; then
  mkdir config
fi

print_sign() {
  i=1
  while [ $i -le $2 ]; do
    printf $1
    i=$((i + 1))
  done
}

for file in $files; do
  if [ -f $file/Dockerfile ]; then
    if [ ! -d config/$file ]; then
      mkdir config/$file
    fi

    printf "┌"
    print_sign "─" $((${#file} + 10 ))
    printf "┐\n"

    echo "│     $file     │"

    printf "└"
    print_sign "─" $((${#file} + 10 ))
    printf "┘\n"

    docker build -t $file -f ./$file/Dockerfile .
  fi
done
