# Pingora gRPC-Web

This repository is an example on how to setup pingora proxy to upgrade requests from gRPC-Web to native gRPC.

## Server

```bash
cd server

cargo run
```

## Proxy

```bash
cd proxy

cargo run
```

## Client

### Native gRPC

```bash
grpcurl \
  -plaintext \
  -d '{}' \
  -import-path protos \
  -proto protos/helloworld.proto \
  -vv \
  localhost:6193 \
  helloworld.Greeter/SayHello
```

### gRPC-Web

Open [index.html](https://github.com/davihsg/) on your browser, set the url to `http://localhost:6193` (proxy url), then click on send request.

![Image](https://github.com/user-attachments/assets/a2461114-5111-4647-a50d-d795044cc1fd)
