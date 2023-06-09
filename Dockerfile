FROM rust:buster AS builder

RUN \
  mkdir /root/fire && \
  mkdir /root/fire/src && \
  mkdir /root/fire/src/setup


WORKDIR /root/fire

COPY Cargo.lock .
COPY Cargo.toml .

WORKDIR /root/fire/src

COPY src/main.rs .
COPY src/setup.rs .
 
WORKDIR /root/fire/src/setup

COPY src/setup/fire_create_dirs.rs .
COPY src/setup/fire_image.rs .
COPY src/setup/fire_misc.rs .
COPY src/setup/fire_mp3_info.rs .
COPY src/setup/fire_process_movie_images.rs .
COPY src/setup/fire_process_movies.rs .
COPY src/setup/fire_process_music_images.rs .
COPY src/setup/fire_process_music.rs .
COPY src/setup/fire_process_tvshows.rs .
COPY src/setup/fire_utils.rs .
COPY src/setup/fire_walk_dirs.rs .

WORKDIR /root/fire

RUN cargo install --path .

# FROM ubuntu:22.04
FROM debian:buster

RUN \
  apt-get update && \
  apt-get -y dist-upgrade && \
  apt-get -y autoclean && \
  apt-get -y autoremove

COPY --from=builder /root/fire/target/release/firecore /usr/local/bin/firecore

RUN chmod -R +rwx /root

WORKDIR /root/

RUN \
  mkdir ./fire && \
  mkdir ./fire/nfos && \
  mkdir ./fire/thumbnails && \
  mkdir ./movies && \
  mkdir ./tvshows && \
  mkdir ./music && \
  mkdir ./USB1 && \
  mkdir ./USB2 && \ 
  mkdir ./USB3 && \
  mkdir ./USB4

RUN \
  chmod -R +rwx ./fire && \
  chmod -R +rwx ./fire/nfos && \ 
  chmod -R +rwx ./fire/thumbnails && \ 
  chmod -R +rwx ./movies && \
  chmod -R +rwx ./tvshows && \
  chmod -R +rwx ./music && \
  chmod -R +rwx ./USB1 && \
  chmod -R +rwx ./USB2 && \
  chmod -R +rwx ./USB3 && \
  chmod -R +rwx ./USB4


STOPSIGNAL SIGINT

# CMD ["tail", "-f", "/dev/null"]
CMD ["/usr/local/bin/firecore"]
