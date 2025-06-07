FROM ubuntu:latest

RUN apt-get update && apt-get install net-tools curl -y
COPY enclave.sh /root
COPY ntk-enclave /root
RUN chmod +x /root/ntk-enclave

CMD ["/bin/sh", "/root/enclave.sh"]