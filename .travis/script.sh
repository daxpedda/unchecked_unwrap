# display commands in build log
set -x

# variable to track exit code
exit_code=0

# builds
cargo build --verbose || exit_code=1
cargo build --release --verbose || exit_code=1
# tests
cargo test --verbose || exit_code=1
cargo test --release --verbose || exit_code=1
# check if needs update
cargo update --locked --verbose || exit_code=1

# nightly has additional features we want to use
# build documentation
if [ "$TRAVIS_RUST_VERSION" != "nightly" ]; then
	cargo doc --verbose || exit_code=1
else
	cargo rustdoc --verbose -- -Z unstable-options --enable-index-page || exit_code=1
	# run benchmarks
	cargo bench --verbose || exit_code=1
fi

# check if clippy is present in the toolchain - because sometimes it's missing
if rustup component add clippy; then
	# clippy check build and tests
	cargo clippy --all-features --verbose -- -D warnings || exit_code=1
	cargo clippy --all-features --tests --verbose -- -D warnings || exit_code=1

	# clippy check benches, but only in nightly, because they don't work in stable yet
	if [ "$TRAVIS_RUST_VERSION" == "nightly" ]; then
		cargo clippy --all-features --benches --verbose -- -D warnings || exit_code=1
	fi
fi

# check if fmt is present in the toolchain - because sometimes it's missing
if rustup component add rustfmt; then
	# check if any formatting is needed
	cargo fmt --all --verbose -- --check || exit_code=1
fi

# if it's not a PR and we want to deploy, deploy!
if [ "$TRAVIS_PULL_REQUEST" == false ] &&  [ "$TRAVIS_RUST_VERSION" == "nightly" ]; then
	# try to install packages
	cargo install cargo-update || echo "cargo-update already installed"
	RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin --git "https://github.com/xd009642/tarpaulin.git" --branch "develop" || echo "cargo-tarpaulin already installed"
	cargo install cargo-travis --git "https://github.com/daxpedda/cargo-travis.git" --branch "temporary" || echo "cargo-travis already installed"
	# now we just check for updates
	cargo install-update cargo-update
	RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install-update cargo-tarpaulin -g
	cargo install-update cargo-travis -g
	# decrypt deploy key
	echo $github_deploy_key | gpg --passphrase-fd 0 ".travis/github_deploy_key.gpg"
	chmod 600 ".travis/github_deploy_key"
	# add it to the current git shell
	eval "$(ssh-agent -s)"
	ssh-add ".travis/github_deploy_key"
	# upload documentation
	cargo doc-upload --branch $TRAVIS_BRANCH --clobber-index || exit_code=1
	# do some test coverage
	cargo tarpaulin --out Xml --verbose || exit_code=1
	bash <(curl -s https://codecov.io/bash) || exit_code=1
	# do some more test coverage in release mode
	cargo tarpaulin --release --out Xml --verbose || exit_code=1
	bash <(curl -s https://codecov.io/bash) || exit_code=1
fi

# this is just some workaround to pass variables safely
exit_script='exit "$1"'
# pass the exit code to travis
sh -c "$exit_script" -- "$exit_code"
