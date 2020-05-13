#!/usr/bin/env zsh

CARGO_INCREMENTAL='0'
RUST_FLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
RUST_DOC_FLAGS="-Cpanic=abort"
RUST_VERSION="${LANGUAGES_RUST_VERSION:-1.42.0}"

while getopts "hr:" opt; do
  case $opt in
    r) RUST_VERSION=$OPTARG ;;
    h) echo "Usage: ${0:t} [-r"
  esac
done

RUST_IMAGE="rust:$RUST_VERSION"

function run_rust_container_command {
  local command="$1"
  local entrypoint="$2"

  get_container_volume 'rust-cargo-cache'
  get_container_volume 'rustup-cache'
  local args=(\
    "--rm" \
    "--user" "$(id -u):$(id -g)" \
    "-v" "rust-cargo-cache:/usr/local/cargo" \
    "-v" "rustup-cache:/usr/local/rustup" \
    "-v" "'$PWD:/app/zephrys'" \
    "-w" "/app/zephrys" \
    "-e" "CARGO_INCREMENTAL=\"$CARGO_INCREMENTAL\"" \
    "-e" "RUSTFLAGS=\"$RUST_FLAGS\"" \
    "-e" "RUSTDOCFLAGS=\"$RUST_DOC_FLAGS\"" \
  )
  if test ! -z $entrypoint; then args+=("--entrypoint=$entrypoint"); fi
  args+=("$RUST_IMAGE")
  if test -n $command; then args+=("$command"); fi

  echo $args | xargs -L1 docker run
}

function get_container_volume {
  docker volume create $1 > /dev/null
}

function main {
  echo "ZephyRS CI :: Running tests and generating a coverage report"
  docker pull $RUST_IMAGE

  run_rust_container_command 'env'
  run_rust_container_command 'rustup install nightly'
  run_rust_container_command "bash -c \"cargo +nightly check\""
  run_rust_container_command "bash -c \"cargo +nightly test -- --nocapture\""
  grcov ./target/debug \
    -o ./coverage.json \
    -t coveralls \
    -s . \
    --llvm \
    --branch \
    --ignore-not-existing \
    --parallel \
    --log stdout \
    --service-job-id $(git rev-parse HEAD) \
    --service-name github-actions
}

main
