FROM ubuntu:latest

RUN apt-get update && apt-get install net-tools -y

COPY ./build/ntk-enclave /usr/bin
RUN mkdir /root/.ntk
COPY ./scripts/ /root/.ntk

RUN chmod +x /usr/bin/ntk-enclave

ENTRYPOINT ["bash", "/root/.ntk/scripts/entrypoint.sh"]