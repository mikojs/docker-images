# act

You could see more detail [here](https://github.com/nektos/act).

## How to use `act` image

```sh
docker run \
  -it \
  -v $(pwd):/repo \
  -v /var/run/docker.sock:/var/run/docker.sock \
  act --rm
```
