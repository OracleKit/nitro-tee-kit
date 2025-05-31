FROM alpine:latest

COPY enclave.sh .
COPY ../target/debug/ntk-enclave .

CMD ["./enclave.sh"]