# Recipes written down in one place for both CI/CD, and basic
# documentatoin/portable usage (we can expect everyone has make instaled).

dev: build test lint 

watch_test:
	cargo watch \
		--ignore='.vcsq-*/target/' \
		--ignore='.jj/' \
		-s 'make test'

watch_build:
	cargo watch \
		--ignore='.vcsq-*/target/' \
		--ignore='.jj/' \
		-s 'make build' \
		-s 'make lint' \
		-s 'make doc'

all: build test doc lint 

clean:
	cargo clean

clean_all: clean
	$(RM) *.profraw
	$(RM) vcsq-lib/*.profraw
	$(RM) vcsq-cli/*.profraw

# everything above this line is meant for local development usage (ci/cd runs
# things more carefully, in interdependent stages)
##############################################################################

build:
	RUSTFLAGS='-Ddeprecated -Dwarnings' cargo build --workspace --all-targets

test: e2e_test_deps
	RUST_BACKTRACE=full RUSTFLAGS='-Ddeprecated -Dwarnings' cargo test --workspace --locked --all-features --all-targets --verbose -- --nocapture

doc: 
	RUSTFLAGS='-Ddeprecated -Dwarnings' cargo doc --workspace --all-features

lint:
	cargo clippy --workspace --all -- -W clippy::pedantic -Dwarnings -Ddeprecated

# TODO: uncomment --output-path below, delete --text. We
# can do this once we're public. Until then, this will always fail per
# https://github.com/codecov/codecov-action/issues/1671#issuecomment-2486953810
# (and I don't feel like messing with tokens just for the interim).
# - bash <(curl -s https://codecov.io/bash)
#
# TODO: once coverage is working for me locally, maybe:
# LLVM_PROFILE_FILE="target/coverage/prof/%p-%m.profraw"
#
# TODO: figure out what's wrong with coverage; current status:
# - expecting: >70% coverage at _least_
# - seeing: <30% coverage, and the missed lines are _definitely_ executed as
#   part of e2e suite.
# - ways I've tried to run analysis:
#
#   - <19% line-coverage from:
#
#   ```sh
#   $ cargo tarpaulin --release --bins --engine llvm --follow-exec
#   ```
#   - <32% line-coverage from:
#
#   ```sh
#   $ cargo tarpaulin --command test --workspace --release --engine llvm --follow-exec
#   ```
#   - <21% line-coverage from:
#
#   ```sh
#   $ RUSTFLAGS='-Ddeprecated -Dwarnings' cargo llvm-cov --all-features --workspace --summary-only
#   ```
#
# Same as test's steps, but run via code coverage instrumentor:
cov: e2e_test_deps
	RUSTFLAGS='-Ddeprecated -Dwarnings' cargo llvm-cov --all-features --workspace --summary-only # --text # --codecov  --output-path codecov.json

e2e_test_deps: have_vcs_deps

have_vcs_deps: have_vcs_git have_vcs_hg have_vcs_jj

have_vcs_git:
	which git
	git --version

have_vcs_hg:
	which hg
	hg --version

have_vcs_jj:
	which jj
	jj --version

# Yes, we're calling build phony because we using this as a sort of portable
# script, _not_ trying to rely on Make's needs-rebuild heuristics. (for that we
# should use a different tool if we really want one; eg: ).
.PHONY: dev watch_build watch_test all clean clean_all build doc lint test cov e2e_test_deps have_vcs_deps have_vcs_git have_vcs_hg have_vcs_jj
