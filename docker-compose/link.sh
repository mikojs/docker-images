#/bin/sh

if [ ! -d /storage/config ]; then
  if [ -z $CONFIG_URL ]; then
    echo "could not get the url of the config repository"
    exit 1
  fi

  git clone $CONFIG_URL /storage/config
fi

files=$(find /storage/config -type f | \
  grep -v .git/ | \
  grep -v .gitignore | \
  grep -v .swp | \
  sed "s/\/storage\/config//")

for file in $files; do
  if [ ! -d $(dirname $file) ]; then
    mkdir -p $(dirname $file)
  fi

  ln -s "/storage/config$file" $file
done
