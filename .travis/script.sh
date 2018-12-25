#!/bin/bash
cargo build --verbose
cargo build --release --verbose
cargo test --verbose
cargo test --release --verbose
cargo update --locked --verbose

# nightly has additional features we want to test and use
if [ "$TRAVIS_RUST_VERSION" != "nightly-2018-12-23" ]; then
	cargo doc --verbose
else
	#cargo rustdoc --verbose -- -Z unstable-options --enable-index-page
	cargo bench --verbose
fi

if rustup component add clippy; then
	cargo clippy --all-features --verbose -- -D warnings
	cargo clippy --all-features --tests --verbose -- -D warnings

	if [ "$TRAVIS_RUST_VERSION" == "nightly-2018-12-23" ]; then
		cargo clippy --all-features --benches --verbose -- -D warnings
	fi
fi

if rustup component add rustfmt; then
	cargo fmt --all --verbose -- --check
fi

if [ "${TRAVIS_PULL_REQUEST}" == false ] &&  [ "${DEPLOY}" == true ]; then
	cargo install cargo-update || echo "cargo-update already installed"
	RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin --git "https://github.com/xd009642/tarpaulin.git" --branch "develop" || echo "cargo-tarpaulin already installed"
	cargo install cargo-travis --git "https://github.com/daxpedda/cargo-travis.git" --branch "badge" || echo "cargo-travis already installed"
	cargo install-update cargo-update
	RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install-update cargo-tarpaulin -g
	cargo install-update cargo-travis -g
	echo $github_deploy_key | gpg --passphrase-fd 0 ".travis/github_deploy_key.gpg"
	chmod 600 ".travis/github_deploy_key"
	eval "$(ssh-agent -s)"
	ssh-add ".travis/github_deploy_key"
	cargo doc-upload --branch $TRAVIS_BRANCH
	cargo tarpaulin --out Xml --verbose
	bash <(curl -s https://codecov.io/bash)
	cargo tarpaulin --release --out Xml --verbose
	bash <(curl -s https://codecov.io/bash)
fi
