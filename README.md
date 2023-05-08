# ServeDir

A simple web server that serves the contents of a directory.

Simple docker image example:

```dockerfile
FROM alpine:latest

ARG SERVE_DIR_VERSION=0.1.0

RUN apk add --no-cache \
    unzip \
    ca-certificates

ADD https://github.com/jerebtw/ServeDir/releases/download/v${SERVE_DIR_VERSION}/serve_dir-x86_64-unknown-linux-musl.zip /tmp/serve_dir.zip
RUN unzip /tmp/serve_dir.zip -d /app
RUN chmod +x /app/serve_dir

EXPOSE 8080

CMD ["/app/serve_dir", "--dir", "/app/static", "--url", "0.0.0.0:8080"]
```
