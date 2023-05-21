#!/bin/sh

rm ./fire_db && \
rm /home/pi/NFOS/*.json && \
rm /home/pi/Thumbnails/*.jpg && \
cargo run --release