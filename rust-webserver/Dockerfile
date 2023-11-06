FROM debian:stable-slim
COPY rust-webserver /usr/bin/rust-webserver
RUN mkdir -p /var/www/html
ENTRYPOINT /usr/bin/rust-webserver
