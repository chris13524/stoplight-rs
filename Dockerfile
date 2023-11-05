FROM debian:bullseye
RUN apt update && apt install -y ca-certificates
WORKDIR /
COPY stoplight-rs stoplight-rs
CMD ["/stoplight-rs"]
