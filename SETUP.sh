#!/bin/sh

rm ./fire_db && \
rm /home/pi/nfos/*.json && \
rm /home/pi/thumbnails/*.jpg && \
git pull && \
cargo run --release