FROM alpine

RUN apk add \
  git \
  github-cli \
  vim \
  zsh \
  npm \
  docker

# npm packages
RUN npm install -g commitizen cz-conventional-changelog
RUN echo '{ "path": "cz-conventional-changelog" }' > ~/.czrc

# code-server
RUN wget https://github.com/cdr/code-server/releases/download/v4.4.0/code-server-4.4.0-linux-amd64.tar.gz && \
  tar x -zf code-server-4.4.0-linux-amd64.tar.gz && \
  rm code-server-4.4.0-linux-amd64.tar.gz
RUN mkdir ~/.local ~/.local/lib ~/.local/bin && \
  mv code-server-4.4.0-linux-amd64 ~/.local/lib/code-server && \
  sed -i 's/"$ROOT\/lib\/node"/node/g' ~/.local/lib/code-server/bin/code-server && \
  ln -s ~/.local/lib/code-server/bin/code-server ~/.local/bin/code-server

# work directory
ENV PATH="/root/.local/bin:$PATH"
WORKDIR /root
