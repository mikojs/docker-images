FROM node:16-alpine

# install packages
RUN apk add \
  # code-server
  curl \
  openssl \
  python3 \
  alpine-sdk \
  bash \
  libstdc++ \
  libc6-compat \
  # docker
  docker \
  docker-compose
RUN npm config set python python3
RUN npm install \
  --unsafe-perm=true \
  -g \
  yazl \
  yauzl \
  @microsoft/1ds-core-js \
  minimist \
  vscode-regexpp \
  vscode-proxy-agent \
  @vscode/ripgrep \
  xterm-headless \
  spdlog \
  code-server
RUN cd /usr/local/lib/node_modules/code-server/lib/vscode && \
  npm i -f

# code-server
RUN mkdir -p /root/.local/share/code-server/logs

# bin
COPY ./bin/entrypoint /bin/entrypoint

# volumes
VOLUME ["/root/work"]

# run
ENTRYPOINT ["entrypoint", "code-server /root/work"]
