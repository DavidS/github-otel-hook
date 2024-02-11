FROM mcr.microsoft.com/devcontainers/rust:dev-1-bookworm

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld postgresql-client \
   && apt-get autoremove -y && apt-get clean -y \
   && curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
   && cargo binstall -y --force --only-signed cargo-watch

# use `cargo watch -x check -x test -x "run --package service --bin service"` to run tests and check code before running the service
