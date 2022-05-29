#/bin/sh

if [ ! -d /storage/config ]; then
  if [ -z $CONFIG_URL ]; then
    echo "could not get the url of the config repository"
    exit 1
  fi

  git clone $CONFIG_URL /storage/config
fi

for file in $(cat /storage/config/.$STORAGE_NAME); do
  if [ ! -d $(dirname $file) ]; then
    mkdir -p $(dirname $file)
  fi

  ln -s "/storage/config$file" $file
done
