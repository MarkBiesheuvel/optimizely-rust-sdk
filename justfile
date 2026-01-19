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

clippy:
  cargo clippy

dry-run-publish:
  cargo publish --dry-run --allow-dirty -p optimizely

run example:
  cd examples/{{example}} && cargo run

pdf:
  wkhtmltopdf $(find target/doc/optimizely -type f -name '*.html' -printf '%d http://[::1]/optimizely-rust-sdk/%p\n' | sort -n | cut -d' ' -f2- ) target/doc/optimizely-rust-sdk.pdf