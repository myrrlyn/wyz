################################################################################
#                                   Justfile                                   #
#                                                                              #
# Set of routines to execute for development work.                             #
################################################################################

# Run the benchmarks. Currently, this requires the nightly compiler series.
bench:
	cargo +nightly bench

# Build the project, after checking that it is valid.
build: check
	cargo build --all-features

# Runs the checker and linter.
check: format
	cargo check --all-features
	cargo clippy --all-features

# Destroys build artifacts.
clean:
	cargo clean

# Development workflow.
dev: format check doc test

# Documents the project, after checking that it is valid.
doc: check
	cargo doc --document-private-items --all-features

format:
	cargo +nightly fmt

# Runs a Justfile recipe on every change to the workspace.
loop action:
	watchexec -- "just {{action}}"

# Runs the project under the Miri interpreter. This is currently nightly-only.
miri:
	cargo +nightly miri test

# Prepares the project for package deployment.
#
# This allows uncommitted VCS files, as a convenience for development.
package: test doc
	cargo package --allow-dirty

# Publishes the project to crates.io.
#
# This repackages the project and fails on a dirty VCS checkout.
publish: test doc
	cargo package # no --allow-dirty this time
	cargo publish

# Runs the test suite.
test: build
	cargo test --all-features
