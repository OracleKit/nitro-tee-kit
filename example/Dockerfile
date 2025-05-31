FROM alpine:latest

RUN apk update && apk add net-tools curl
COPY enclave.sh .
COPY ../target/debug/ntk-enclave .

CMD ["./enclave.sh"]