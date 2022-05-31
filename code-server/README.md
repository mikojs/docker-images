# code-server

You could see more detail [here](https://github.com/coder/code-server). This image would generate a random password. You could send it to the Slack channel or just show it in the terminal.

## How to use `code-server` image

```sh
docker run \
  -it \
  -p 8080:8080 \
  code-server --bind-addr 0.0.0.0:8080
```
