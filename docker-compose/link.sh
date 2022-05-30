#/bin/sh

clone_config() {
  if [[ -z $CONFIG_URL ]]; then
    echo "could not get the url of the config repository"
    exit 1
  fi

  git clone $CONFIG_URL /storage/config
}

link_files() {
  for cache_path in $(cat /storage/config/.$STORAGE_NAME); do
    cache_type=$(echo $cache_path | grep cache | sed 's/:.*//')
    cache_file=$(echo $cache_path | sed 's/.*://' | sed 's/^/\//')

    if [[ ! -d $(dirname $cache_file) ]]; then
      mkdir -p $(dirname $cache_file)
    fi

    if [[ "$cache_type" =~ ^cache-* ]]; then
      if [[ ! -d $(dirname "/storage/cache$cache_file") ]]; then
        mkdir -p $(dirname "/storage/cache$cache_file")
      fi

      if [[ $cache_type == cache-dir ]] && [[ ! -d "/storage/cache$cache_file" ]]; then
        mkdir "/storage/cache$cache_file"
      elif [[ $cache_type == cache-file ]] && [[ ! -f "/storage/cache$cache_file" ]]; then
        touch "/storage/cache$cache_file"
      fi
    fi

    if [[ ! -L $cache_file ]]; then
      if [[ -f $cache_file ]] || [[ -d $cache_file ]]; then
        rm -rf $cache_file
      fi

      if [[ -z $cache_type ]]; then
        ln -s "/storage/config$cache_file" $cache_file
      else
        ln -s "/storage/cache$cache_file" $cache_file
      fi
    fi
  done
}

if [[ -z $BATS_TEST_FILENAME ]]; then
  if [[ ! -z $INIT_STORAGE ]]; then
    apk add \
      git \
      openssh
    clone_config

    if [[ -d /root/.ssh ]] && [[ -d /storage/config/root/.ssh ]]; then
      cp /root/.ssh/* /storage/config/root/.ssh
    fi
  fi

  if [[ -z $STORAGE_NAME ]]; then
    exit 0
  fi

  line_files
fi
