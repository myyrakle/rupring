FROM rust:1.83.0-alpine AS builder
COPY . /home/server 
WORKDIR /home/server
# Do something for ready to build
RUN apk add musl-dev
RUN cargo build --release
RUN mv /home/server/target/release/rupring_example /home/server/bin

FROM alpine:3.21.0 AS deployer
RUN mkdir /app
COPY --from=builder /home/server/bin /app/server
COPY --from=builder /home/server/application.properties /app/application.properties
RUN ls -lah /app
ENTRYPOINT [ "/bin/sh", "-c", "/app/server" ]
EXPOSE 8080