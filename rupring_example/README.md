# Rupring Example

Simple example template

## Local Run

very easy

```bash
cargo run
```

## Run with Docker

```bash
cd ./rupring_example
sudo docker build -t test .
sudo docker run -p 8080:8000 test
# ...

curl http://localhost:8080
```

## Deploy to AWS Lambda

1. Create an AWS Lambda. The runtime must be either Amazon Linux 2, Amazon Linux 2023.

2. Compile and create an executable file.

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

3. Zip the executable file and upload it to the AWS console.

```bash
zip -j bootstrap.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
# ...and upload it as a file to the AWS console
```
