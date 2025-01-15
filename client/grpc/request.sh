ADDRESS="localhost:6193"

grpcurl \
  -plaintext \
  -d '{}' \
  -import-path ../../protos \
  -proto ../../protos/helloworld.proto \
  -vv \
  $ADDRESS \
  helloworld.Greeter/SayHello
