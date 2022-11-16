FROM ubuntu:22.04

WORKDIR /home/

COPY . .

## update and install some things we should probably have
RUN apt-get update
RUN apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  vim \
  build-essential \
  openssl

## Install rustup and common components
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source "$HOME/.cargo/env"
RUN rustup install nightly
RUN rustup component add rustfmt
RUN rustup component add rustfmt --toolchain nightly
RUN rustup component add clippy 
RUN rustup component add clippy --toolchain nightly

ENV PATH="/root/.cargo/bin:$PATH"