FROM alpine:latest

RUN apk update && apk add net-tools curl
COPY enclave.sh .
COPY ntk-enclave .

CMD ["./enclave.sh"]