#!/bin/sh

mprotoc --package -l rust -n probius-mproto -o crates/ proto/probius.mproto
