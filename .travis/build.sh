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

# we don't want build to break just because there is something not up to date
if [ $TRAVIS_BRANCH != "release" ]; then
	cargo update --locked --verbose || exit_code=1
fi

# nightly has additional features we want to use
# build documentation
if [ $TRAVIS_RUST_VERSION != "nightly" ]; then
	cargo doc --verbose || exit_code=1
else
	cargo rustdoc --features doc_include --verbose -- -Z unstable-options --enable-index-page || exit_code=1
	# run benchmarks
	cargo bench --verbose || exit_code=1
fi

# check if clippy is present in the toolchain - because sometimes it's missing
if rustup component add clippy; then
	# clippy check build and tests
	cargo clippy --verbose -- -D warnings || exit_code=1
	cargo clippy --tests --verbose -- -D warnings || exit_code=1

	# clippy check benches, but only in nightly, because they don't work in stable yet
	if [ $TRAVIS_RUST_VERSION == "nightly" ]; then
		cargo clippy --benches --verbose -- -D warnings || exit_code=1
	fi
fi

# check if fmt is present in the toolchain - because sometimes it's missing
# also we don't want to break the build just because some formatting is broken
if rustup component add rustfmt && [ $TRAVIS_BRANCH != "release" ]; then
	# check if any formatting is needed
	cargo fmt --all --verbose -- --check || exit_code=1
fi

# if it's not a PR and we want to deploy, deploy!
if [ $TRAVIS_PULL_REQUEST == false ] && [ $TRAVIS_RUST_VERSION == "nightly" ]; then
	# install and update packages
	cargo install cargo-update
	cargo install-update cargo-update
	cargo install cargo-audit
	cargo install-update cargo-audit
	cargo install cargo-travis --git "https://github.com/roblabla/cargo-travis.git"
	cargo install-update cargo-travis --git

	# check rust RustSec db
	cargo audit || exit_code=1

	# decrypt deploy key
	echo $github_deploy_key | gpg --passphrase-fd 0 ".travis/github_deploy_key.gpg"
	chmod 600 ".travis/github_deploy_key"
	# add it to the current git shell
	eval "$(ssh-agent -s)"
	ssh-add ".travis/github_deploy_key"
	# upload documentation
	cargo doc-upload --branch $TRAVIS_BRANCH --clobber-index --path $TRAVIS_BRANCH/doc/ || exit_code=1
fi

# pass the exit code to travis
exit $exit_code
