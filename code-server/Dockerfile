FROM alpine

RUN apk add \
  git \
  github-cli \
  vim \
  zsh \
  nodejs \
  npm \
  docker

# npm packages
RUN npm install -g commitizen cz-conventional-changelog
RUN echo '{ "path": "cz-conventional-changelog" }' > ~/.czrc

# code-server
# FIXME: https://github.com/coder/code-server/issues/5184, use: npm install -g code-server
RUN apk add gcompat
RUN wget https://github.com/cdr/code-server/releases/download/v4.4.0/code-server-4.4.0-linux-amd64.tar.gz && \
  tar x -zf code-server-4.4.0-linux-amd64.tar.gz && \
  rm code-server-4.4.0-linux-amd64.tar.gz
RUN mkdir ~/.local ~/.local/lib ~/.local/bin && \
  mv code-server-4.4.0-linux-amd64 ~/.local/lib/code-server && \
  sed -i 's/"$ROOT\/lib\/node"/node/g' ~/.local/lib/code-server/bin/code-server && \
  ln -s ~/.local/lib/code-server/bin/code-server ~/.local/bin/code-server

# copy config
COPY ./config/code-server /root/.config/code-server

# run code-server
ENV PATH="/root/.local/bin:$PATH"
WORKDIR /root
ENTRYPOINT ["code-server"]
