#!/usr/bin/env bash

DIR=$(cd "$(dirname "$0")"; pwd)
set -ex
cd $DIR

wget http://enumer.org/public-stun.txt -O stun.txt

