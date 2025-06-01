FROM ubuntu:latest

RUN apt-get update && apt-get add net-tools curl -y
COPY enclave.sh /home/root
COPY ntk-enclave /home/root
RUN chmod +x /home/root/ntk-enclave

CMD ["/bin/sh", "/home/root/enclave.sh"]