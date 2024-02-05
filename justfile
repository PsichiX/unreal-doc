list:
  just --list

format:
  cargo fmt

build:
  cargo build

build-release:
  cargo build --release

test:
  cargo test

example-doc:
  cargo run -- -i ./resources/UnrealDoc.toml
  mdbook serve ./resources/docs --open

clippy:
  cargo clippy

checks:
  just build
  just clippy
  just test
  just test-doc-gen

list-outdated:
  cargo outdated -R -w

update:
  cargo update --aggressive

publish:
  cargo publish --no-verify
