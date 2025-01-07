# vcsq: VCSquery - Version Control System (VCS) Querying CLI

`vcsq` tries to answer a _small set_ of generic questions about "any" type of
VCS repo, without making you think about the particular flavor of VCS at play.

This repo lives at <https://gitlab.com/jzacsh/vcsq>

## Project Status

[![Build Status][gitlab_ci_badge]][gitlab_ci_dash] shows e2e tests' status.

## Design & Goals

**Goal**: answer _basic_ introspective questions about a repo/directory.

This very much inspired by the use-case of the popular [`vcprompt`
CLI][vcprompt] I've been using for years via my `$PS1`, but also by APIs I've
frequently[^freq] wanted for scripting purposes. Each of those APIs I wished for
is now outlined in this codebase's `VcstQuery` enum of in the namesaked
reference binary (at `./vcst/src/lib.rs`).

The goal is to have coverage for the popular VCS I personally encounter
regulalry, like `git`, `hg`, `jj`, but I tried to make it as biolerplate-free as
possible to add new ones. So **contributions of new VCS coverage** is welcome,
just chat with me about it first to avoid any mis-spent time.

### Usage

TODO: (feature) outline installation, and super-basic `$PS1` bash integration.

### Shelling Out

Ultimately this is just like a shell script: it's relying on the CLI of the VCS
actually being in your `$PATH`, and interacting with it as such. I think it'd be
_fun_ to explore calling the VCS's own libraries at some point, and I tried to
write the library with that in mind from the start. _But_ that's probably as or
less important than tying up some loose ends I stlil have (TODOs below like a
few more features I wnat, some rust/cargo questions I'm unclear on, and a few
techdebt TODOs).

## Development

The codebase is a library (`./lib/`) and its dependent: a CLI at `./vcst/`.

Since logic in `lib/` is designed for its only client (`vcst/`), that client's e2e
tests are _the_ test coverage for this entire codebase, so local development
just involves producing a debug binary and making sure you haven't broken tests:

```sh
$ cd vcst && cargo watch test  --color=always -- --nocapture
# ...
```

In a second terminal I ensure the binary is being continuously rebuilt:

```sh
$ cd vcst && cargo watch \
  -x build \
  -x 'clippy --all -- -W clippy::pedantic -Dwarnings -Ddeprecated' \
  -s 'cd ../lib && cargo clippy --all -- -W clippy::pedantic -Dwarnings -Ddeprecated' \
  -x 'doc --all-features' \
  -s 'cd ../lib && cargo doc --all-features'

# ...
# can also be tacked onto the previous command via another '-x build' arg at the
# before the test args, but then you get the issue of too-many-lines-output when
# there's compiler errors you're trying to read.
```

### Tests

e2e tests of the CLI binary, in `vcst/tests/`, are the strategy for the moment;
they covery every API that `lib/` is meant to offer.

Gitlab servers also runs this suite on every merge to main, via `.gitlab-ci.yml`
instructions. The results can be seen at:
<https://gitlab.com/jzacsh/vcsq/-/jobs>

#### Test Coverage

`cargo-llvm-cov` is used to instrument the e2e tests, and the results are simply
dumped as text (for now), which can be read in the ci/cd output. To understand
the output, according to [llvm-cov][manLlvmCovDesc]:

> The basic content of an .gcov output file is a copy of the source file with an
> **execution count and line number prepended to every line**. The execution
> count is shown as - if a line does not contain any executable code. If a line
> contains code but that code was never executed, the count is displayed as
> `#####`

So to see untested lines, just `^F` for " |0" in the output.

[manLlvmCovDesc]: https://manpages.debian.org/bookworm/llvm/llvm-cov.1.en.html#GCOV_COMMAND

### TODOs

- [ ] cleanup/finish rename of objects to vcsq (from old vcst) (and also just
  stop prefixing any internal objects with the liberary's name; idk why the heck
  I did that).
- [ ] **finish tail-end of feature-set**: see lines in vcst/src/lib.rs disabled
  in release
  - try `cargo build --release` to turn these back off
  - in the meantime: dbug build via: `cargo run --` to run, `cargo build`
  - `grep -C 1 -rnE '\b(todo|unimplemented|panic|expect)!' {lib,vcst}/src` to
  hunt down tasks build (because they just `todo!()`, hidden via
  `#[cfg(debug_assertions)]`)
- [ ] techdebt/rust question: merge the two folders lib/ and vcst/ into root?
  any downsides one way or the other? maybe one makes crates.io usage harder? I
  guess separated they have clearer deps-attribution?
- [ ] setup a Github mirror [via gitlab's mechanism][gLabToGhubMirror]
  have one crate when published).
- [ ] techdebt/rust-question: cleanup some of the error enums that aren't being
  fully utilized (eg: some that default to map_err() to `Unknown`-fallbackish
  variants). This is because some of the better alternatives were only added
  _later_ (eg: `RepoLoadError::Stderr`) which could fix some
- [ ] techdbt: make it easier to develop by setting up a nix flake that installs
  all the deps (then cleanup the gitlab CI to use that flow too).

[^freq]:
    See the three predecessors/mini-libs that inspired this one, at:
    [`vcsq` of gitlab.com/jzacsh/dotfiles][dotsVcsq] ([ref][dotsVcsq_ref]) and at
    [`vcs.sh` of gitlab.com/jzacsh/yabashlib][yblibVcs] ([ref][yblibVcs_ref]) and
    [gitlab.com/jzacsh/jzach.gitlab.io][wwwVcsts]

[gLabToGhubMirror]: https://docs.gitlab.com/ee/user/project/repository/mirror/push.html#set-up-a-push-mirror-from-gitlab-to-github
[yblibVcs]: https://gitlab.com/jzacsh/yabashlib/-/blob/main/src/vcs.sh
[yblibVcs_ref]: https://gitlab.com/jzacsh/yabashlib/-/blob/dd838fc3b32a66fe2ec95fb85a5e9aa67280fee9/src/vcs.sh
[dotsVcsq]: https://gitlab.com/jzacsh/dotfiles/-/blob/main/bin/lib/vcsq
[dotsVcsq_ref]: https://gitlab.com/jzacsh/dotfiles/-/blob/2543adf4a6d4fcf946d0fda2c70658f72739a250/bin/lib/vcsq
[wwwVcsts]: https://gitlab.com/jzacsh/jzacsh.gitlab.io/-/blob/fix-jj-usage-vcslib-refactoring/src/bin/vcslib.ts?ref_type=heads
[gitlab_ci_badge]: https://gitlab.com/jzacsh/vcsq/badges/main/pipeline.svg
[gitlab_ci_dash]: https://gitlab.com/jzacsh/vcsq/-/jobs
[vcprompt]: http://vc.gerg.ca/hg/vcprompt

## License

Apache v2.
