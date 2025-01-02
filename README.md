# vcst: VCSrusT - Version Control System (VCS) utils in Rust

`vcst` tries to answer a _small set_ of generic questions about "any" type of
VCS repo, without making you think about the particular flavor of VCS at play.

This repo lives at <https://gitlab.com/jzacsh/vcst>

## Project Status

[![Build Status][gitlab_ci_badge]][gitlab_ci_dash] shows e2e tess' status. I'm
hoping for near-complete coverage.

_tl;dr_ the effort's 80% done, but the functionality's only 30% through. Some
early/core functionality (see "goals" section) is already done (and I should
probably just port my `$PS1` already), an enormous amount of test-infra and
outlining to enable the rest of the APIs, and now just remains some drudge-work
to finish the port.

## Design & Goals

**Goal**: answer 101 introspective questions about a repo/directory.

This very much inspired by Greg's famous `vcprompt` I'd been using for years,
but also APIs I've frequently[^freq] wanted for scripting purposes. Each of
those APIs I wished for is now outlined in this codebase's `VcstQuery` enum of
in the namesaked reference binary (at `./vcst/src/lib.rs`).

The goal is to have coverage for the popular VCS I personally encounter
regulalry, like `git`, `hg`, `jj`, but I tried to make it as biolerplate-free as
possible to add new ones.

### Usage

TODO: (feature) outline installation, and super-basic `$PS1` bash integration.

### Shelling Out

Ultimately this is just like a shell script: it's relying on the CLI of the VCS
actually being in your `$PATH`, and interacting with it as such. I think it'd be
fun to explore calling the VCS's own libraries at some point, and I tried to
write the library with that in mind from the start. _But_ that's a whole nother
ball of wax and very unlikely to happen before major TODOs are completed
(feature completeness, and repo-setup TODOs below, like local/remote ci/cd
steps).

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
$ cd vcst && cargo watch -x build
# ...
# can also be tacked onto the previous command via another '-x build' arg at the
# before the test args, but then you get the issue of too-many-lines-output when
# there's compiler errors you're trying to read.
```

### Tests

e2e tests of the CLI binary, in `vcst/tests/`, are the strategy for the moment;
they covery every API that `lib/` is meant to offer.

Gitlab servers also run this for us on every merge to main, via `.gitlab-ci.yml`
instructions. The results can be seen at: <https://gitlab.com/jzacsh/vcst/-/jobs>

TODO: (test infra) consider either/both:

1. starting to teardown the vcst tests temp directory (see
   `vcst/tests/common/mod.rs` for the tempdir setup funcs that get called for
    every test in `vcst/tests/integration_test.rs`)
2. root-less container setup to easily run our e2e tests (so we can contain any
   potentially buggy teardown(), and not delte our own root directory).

### TODOs

- [x] install jj VCS to $PATH of gitlab ci/cd

  - [ ] move `git --version && hg --version && jj --version` frmo ci/cd yml into
  test harness startup as a sanity-check _anywhere_ the test runs.

- [ ] feature: run a dump of what VCS we find in path, as part of `--version`
      output, or some place of the sort.
- [ ] cleanup all the CLI string handling (the `String::from_utf8` and
      `expect(.*utf8` references) to use `String::from_utf8_lossy`
- [ ] feature: add ["list tracked files" concept][vcsListUsecase]
- [ ] cleanup some of the error enums that aren't being fully utilized (eg: some
  that default to map_err() to `Unknown`-fallbackish variants). This is because
  some of the better alternatives were only added _later_ (eg:
  `RepoLoadError::Stderr`) which could fix some
- [ ] before releasing....
  - [ ] flag-guard `todo!()`/`unimplemented!()` blocks for dev/tests only; eg:
  via `#[cfg(debug_assertions)]`
  - [ ] setup a Github mirror [via gitlab's mechanism][gLabToGhubMirror]
  - [ ] centralize/codify standards I'm trying to follow, so all "preferences"
  are automated:
    - [ ] get local test/build/watch command that will error when clippy isn't
    happy (and document that in the Development instructions above with another
    `-x ...` on the recommended watch line); ie: something that will _error out_
    when either vcst/ or lib/ cause any output from `cargo clippy` (and then
    codify the tip: "maybe just run `cargo clippy --allow-no-vcs --fix`" into readme).
      - [ ] eval options:
        - `cargo fmt --check` seems to be a thing
      - [ ] ci/cd clippy: get gitlab ci to do the above and rport on failures.
    - [ ] get local test/build/watch command that will _report_ coverage status
      - [ ] ci/cd: get gitlab ci to do the above and report on a health-status
      on this. find out what the SLA is for this reporting (do you need to save
      it locally somehwo to have good guarantees? or will it be around for a
      longtime in the gitlab CI pipeline? are their generic solutions for
      updating this sort of history directly into the repo periodically?)
    - [ ] run `cargo doc` (in second tty) and ensure it runs on gitlab ci (use
    `--no-deps --all-features` for that)
  - [ ] get to clippy:pedantic level:
    - [ ] address clippy::pedantic output `cargo clippy --all -- -W
    clippy::pedantic`
    - [ ] roll it into above stages (both doc and ci/cd)

[vcsListUsecase]: https://gitlab.com/jzacsh/dotfiles/-/blob/b166218af42ed43e640fd066a7ff9e0d34a7cea5/bin/lib/hacky-java-rename#L147
[gLabToGhubMirror]: https://docs.gitlab.com/ee/user/project/repository/mirror/push.html#set-up-a-push-mirror-from-gitlab-to-github

[^freq]:
    See the three predecessors/mini-libs that inspired this one, at:
    [`vcsq` of gitlab.com/jzacsh/dotfiles][dotsVcsq] ([ref][dotsVcsq_ref]) and at
    [`vcs.sh` of gitlab.com/jzacsh/yabashlib][yblibVcs] ([ref][yblibVcs_ref]) and
    [gitlab.com/jzacsh/jzach.gitlab.io][wwwVcsts]

[yblibVcs]: https://gitlab.com/jzacsh/yabashlib/-/blob/main/src/vcs.sh
[yblibVcs_ref]: https://gitlab.com/jzacsh/yabashlib/-/blob/dd838fc3b32a66fe2ec95fb85a5e9aa67280fee9/src/vcs.sh
[dotsVcsq]: https://gitlab.com/jzacsh/dotfiles/-/blob/main/bin/lib/vcsq
[dotsVcsq_ref]: https://gitlab.com/jzacsh/dotfiles/-/blob/2543adf4a6d4fcf946d0fda2c70658f72739a250/bin/lib/vcsq
[wwwVcsts]: https://gitlab.com/jzacsh/jzacsh.gitlab.io/-/blob/fix-jj-usage-vcslib-refactoring/src/bin/vcslib.ts?ref_type=heads
[gitlab_ci_badge]: https://gitlab.com/jzacsh/vcst/badges/main/pipeline.svg
[gitlab_ci_dash]: https://gitlab.com/jzacsh/vcst/-/jobs

## License

Apache v2.
