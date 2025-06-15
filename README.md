# Nitro TEE Kit
Nitro TEE Kit (NTK) is a runtime environment tooling for Nitro Enclaves that provides features like network egress, ingress, host file system mounts, and many more...

## Features

- Network egress to Host Machine and Internet

## Installation
NTK has 2 components:
- Host Daemon
- Enclave Base Image

Currently, NTK has been tested only on Amazon Linux 2023.

The Host Daemon needs the following dependencies:
- `nftables`
- `iproute2`

Installing Host Daemon:
```bash
curl -L https://github.com/OracleKit/nitro-tee-kit/releases/latest/download/installer.sh | sh
```

Using enclave base image:
```
FROM oraclekitio/ntk-ubuntu:latest

CMD ["ntk-up", "&&", "bash"]
```

- `ntk-ubuntu` is the base image built on ubuntu, with NTK installed.
- `ntk-up` starts the enclave daemon process.

## Example

Refer to [example/](https://github.com/OracleKit/nitro-tee-kit/tree/main/example)

