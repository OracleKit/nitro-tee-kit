FROM ubuntu:latest

RUN apt-get update && apt-get install net-tools curl -y

COPY enclave.sh /root
COPY ntk-enclave /root
COPY ./service/ntkenclave.service /etc/systemd/system/

RUN chmod +x /root/ntk-enclave
RUN systemctl enable ntkenclave
RUN systemctl start ntkenclave

CMD ["/bin/sh", "/root/enclave.sh"]