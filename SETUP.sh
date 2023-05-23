#!/bin/sh

rm /home/pi/fire/fire_db && \
rm /home/pi/nfos/*.json && \
rm /home/pi/thumbnails/*.jpg && \
git pull && \
cargo run --release