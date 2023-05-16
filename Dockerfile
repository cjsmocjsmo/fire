FROM rust:buster AS builder

RUN mkdir /root/fire

WORKDIR /root/fire

COPY main.rs .
COPY setup.rs .
COPY Cargo.lock .
COPY Cargo.toml .

RUN mkdir /root/fire/setup

WORKDIR /root/fire/setup

COPY fire_create_dirs.rs .
COPY fire_image.rs .
COPY fire_misc.rs .
COPY fire_mp3_info.rs .
COPY fire_process_movie_images.rs .
COPY fire_process_movies.rs .
COPY fire_process_music_images.rs .
COPY fire_process_music.rs .
COPY fire_process_tvshows.rs .
COPY fire_utils.rs .
COPY fire_walk_dirs.rs .


RUN cargo install --path .

# FROM ubuntu:22.04
FROM debian:buster

RUN \
  apt-get update && \
  apt-get -y dist-upgrade && \
  apt-get -y autoclean && \
  apt-get -y autoremove

COPY --from=builder /root/fire/target/release/fire /usr/local/bin/fire

RUN chmod -R +rwx /root

WORKDIR /root/

RUN \
  mkdir ./movies && \
  mkdir ./tvshows && \
  mkdir ./music && \
  mkdir ./thumbnails && \
  mkdir ./USB1 && \
  mkdir ./USB2 && \ 
  mkdir ./USB3 && \
  mkdir ./USB

RUN \
  chmod -R +rwx ./movies && \
  chmod -R +rwx ./tvshows && \
  chmod -R +rwx ./music && \
  chmod -R +rwx ./thumbnails && \ 
  chmod -R +rwx ./USB1 && \
  chmod -R +rwx ./USB2 && \
  chmod -R +rwx ./USB3 && \
  chmod -R +rwx ./USB4


STOPSIGNAL SIGINT

# CMD ["tail", "-f", "/dev/null"]
CMD ["fire"]
