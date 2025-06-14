FROM ubuntu:latest

RUN apt-get update && apt-get install net-tools -y

COPY ./ntk-enclave /usr/bin
COPY ./ntk-up /usr/bin
RUN chmod +x /usr/bin/ntk-enclave && chmod +x /usr/bin/ntk-up

RUN chmod +x /usr/bin/ntk-enclave
