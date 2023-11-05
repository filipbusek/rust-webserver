FROM debian:stable-slim
COPY rust_webserver /root/rust_webserver
RUN mkdir -p /var/www/html
ENTRYPOINT /root/rust_webserver
