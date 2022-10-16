FROM docker

# bin
COPY ./bin/entrypoint /bin/entrypoint
COPY ./bin/ddocker /bin/ddocker

# run
ENTRYPOINT ["entrypoint", "docker"]
