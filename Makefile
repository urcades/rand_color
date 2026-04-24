.PHONY: release-dry-run release-publish check demo-build demo-serve

check:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	cargo test --workspace --all-features
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps

release-dry-run:
	./scripts/release.sh --dry-run

release-publish:
	./scripts/release.sh

demo-build:
	./demo/build.sh

demo-serve:
	./demo/serve.sh
