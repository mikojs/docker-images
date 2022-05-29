FROM node:lts-alpine

# install packages
RUN apk add \
  git \
  github-cli \
  vim \
  zsh \
  docker \
  docker-compose \
  # other packages
  curl \
  openssh \
  openssl \
  tree \
  # code-server packages
  alpine-sdk \
  bash \
  libstdc++ \
  libc6-compat
RUN npm config set python python3

# npm packages
RUN yarn global add \
  commitizen \
  cz-conventional-changelog \
  code-server

# zsh
RUN curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh | \
  sh

# commitizen
RUN echo '{ "path": "cz-conventional-changelog" }' > ~/.czrc

# code-server
RUN mkdir -p /root/.local/share/code-server/logs
COPY ./code-server.sh /code-server.sh

# run
ENV PATH="/root/.local/bin:$PATH"
WORKDIR /root
ENTRYPOINT ["/code-server.sh"]
