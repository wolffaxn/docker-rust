# docker-rust

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![GitHub last commit (branch)](https://img.shields.io/github/last-commit/wolffaxn/docker-rust/main.svg)](https://github.com/wolffaxn/docker-rust)

> Docker image with rust installed.

# Setup

You'll need `docker` on your machine, as well as the `git` command.

Start a shell and clone this repository.

```
git clone https://github.com/wolffaxn/docker-rust.git
cd docker-rust
```

# Build and run all images using the docker command

## Set environment variables

To enable BuildKit build set the DOCKER_BUILDKIT=1 environment variable when invoking the docker build command.

```
export DOCKER_BUILDKIT=1
```

## Build (using docker command)

To build the `buster-slim` image, run the following command.

```
docker build . -t wolffaxn/rust:1.47.0-buster-slim 1.47/buster-slim
```

## Build (using Makefile)

Run this command to build all docker images.

```
make
```

## Run (using docker command)

Run this command to show `cargo` version.

```
❯ docker run --rm -it wolffaxn/rust:1.47.0-buster-slim cargo --version
cargo 1.47.0 (f3c7e066a 2020-08-28)
```

Run this command to show `rustc` version.

```
❯ docker run --rm -it wolffaxn/rust:1.47.0-buster-slim rustc --version
rustc 1.47.0 (18bf6b4f0 2020-10-07)
```

Run this command to show `rustup` version.

```
❯ docker run --rm -it wolffaxn/rust:1.47.0-buster-slim rustup --version
rustup 1.22.1 (b01adbbc3 2020-07-08)
```

## License

Released under the MIT License. See [license](LICENSE.md) for details.
