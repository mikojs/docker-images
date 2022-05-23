FROM alpine

RUN apk add \
  git \
  github-cli \
  vim \
  zsh \
  docker

# code-server
RUN apk add nodejs-current
RUN wget https://github.com/cdr/code-server/releases/download/v4.4.0/code-server-4.4.0-linux-amd64.tar.gz && \
  tar x -zf code-server-4.4.0-linux-amd64.tar.gz && \
  rm code-server-4.4.0-linux-amd64.tar.gz
RUN mv code-server-4.4.0-linux-amd64 ~/.local/lib/code-server && \
  sed -i 's/"$ROOT\/lib\/node"/node/g' ~/.local/lib/code-server/bin/code-server && \
  ln -s ~/.local/lib/code-server/bin/code-server ~/.local/bin/code-server

# work directory
WORKDIR /root
