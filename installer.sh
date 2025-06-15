#!/bin/bash

set -e

RELEASE_GZIP_URL=https://github.com/OracleKit/nitro-tee-kit/releases/latest/download/release.tar.gz
RELEASE_GZIP_NAME=release.tar.gz

TMPDIR=$(mktemp -d)
cd $TMPDIR

curl -o $RELEASE_GZIP_NAME $RELEASE_GZIP_URL
tar -xvf $RELEASE_GZIP_NAME
./build/install

cd ..
rm -rf $TMPDIR