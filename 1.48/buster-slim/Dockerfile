FROM debian:buster-slim AS base

ENV DEBIAN_FRONTEND=noninteractive
RUN set -eux; \
  apt-get update -qq; \
  apt-get install -qq --no-install-recommends \
  # build-essential without dpkg-dev (see https://packages.debian.org/stable/devel/build-essential)
  g++ \
  gcc \
  libc6-dev \
  make \
  #
  ca-certificates \
  ; \
  rm -Rf /var/lib/apt/lists/*

FROM base AS builder

ENV RUST_VERSION=1.48.0
ENV RUSTUP_VERSION=1.23.1
# see https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/x86_64-unknown-linux-gnu/rustup-init.sha256
ENV RUSTUP_SHA256=ed7773edaf1d289656bdec2aacad12413b38ad0193fff54b2231f5140a4b07c5

ENV CARGO_HOME /usr/local/cargo
ENV RUSTUP_HOME /usr/local/rustup
ENV PATH $CARGO_HOME/bin:$PATH

ENV DEBIAN_FRONTEND=noninteractive
RUN set -eux; \
  apt-get update -qq; \
  apt-get install -qq --no-install-recommends \
  curl \
  ; \
  # rust
  curl -Lso /tmp/rustup-init https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/x86_64-unknown-linux-gnu/rustup-init; \
  echo "${RUSTUP_SHA256} /tmp/rustup-init" | sha256sum -c -; \
  chmod +x /tmp/rustup-init; \
  ./tmp/rustup-init -y --default-toolchain ${RUST_VERSION} --default-host x86_64-unknown-linux-gnu --no-modify-path --profile minimal; \
  rm -rf /tmp/*; \
  chmod -R a+w ${CARGO_HOME} ${RUSTUP_HOME}; \
  # cleanup
  apt-get purge --auto-remove -y curl; \
  apt-get clean; \
  rm -Rf /var/lib/apt/lists/*

WORKDIR ${CARGO_HOME}/bin

FROM builder

ARG BUILD_DATE
ARG VCS_REF
ARG VCS_URL

# metadata
LABEL maintainer="Alexander Wolff <wolffaxn@gmail.com>" \
  org.label-schema.build-date=${BUILD_DATE} \
  org.label-schema.docker.dockerfile="/Dockerfile" \
  org.label-schema.vcs-ref=${VCS_REF} \
  org.label-schema.vcs-url=${VCS_URL}
