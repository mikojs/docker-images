FROM node:lts-alpine

# install packages
RUN apk add \
  curl \
  openssl \
  python3 \
  alpine-sdk \
  bash \
  libstdc++ \
  libc6-compat
RUN npm config set python python3
RUN yarn global add \
  node-pty \
  code-server

# code-server
RUN mkdir -p /root/.local/share/code-server/logs
COPY ./code-server.sh /code-server.sh

# run
ENTRYPOINT ["/code-server.sh"]
