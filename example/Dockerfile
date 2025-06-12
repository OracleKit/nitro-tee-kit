FROM ntk-ubuntu

RUN apt-get update && apt-get install net-tools curl -y
COPY enclave.sh /root

CMD ["/bin/bash", "/root/enclave.sh"]