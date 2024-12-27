docs:
  cargo doc -p optimizely --all-features --no-deps

release-test:
  cargo test --all-features --release

quick-test:
  cargo test --all-features --lib -- --nocapture

test:
  just release-test
  just docs

fmt:
  cargo fmt --all

dry-run-publish:
  cargo publish --dry-run --allow-dirty -p optimizely

run example:
  cd examples/{{example}} && cargo run
