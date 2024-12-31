# vcst: VCSrusT - Version Control System (VCS) utils in Rust

Tries to answer generic questions about a VCS repo, without making you think
about the particular flavor of VCS at play.

This repo lives at <https://gitlab.com/jzacsh/vcst>

**STATUS**: 80% effort is done; 30% functionality is done. Early days, has some
functionality ported in from the originals (see "design goals" section), and
enormous amount of test-infra and outlining to enable the rest of the APIs, and
now just remains some drudge-work to finish the port.

## Development [![Build Status][gitlab_ci_badge]][gitlab_ci_dash]

Logic in `lib/` and in main (`vcst/`) is covered by e2e tests, so just run them
continuously via:

```sh
$ cd vcst && cargo watch test  --color=always -- --nocapture
# ...
```

And so the binary is always available for manual testing, just keep it built
via (in a separate terminal):

```sh
$ cd vcst && cargo watch -x build
# ...
# can also be tacked onto the previous command via another '-x build' arg at the
# before the test args, but then you get the issue of too-many-lines-output when
# there's compiler errors you're trying to read.
```

Gitlab servers also run this for us on every merge to main, via `.gitlab-ci.yml`
instructions. The results can be seen at: <https://gitlab.com/jzacsh/vcst/-/jobs>

### TODOs

- [ ] ci/cd clippy: get gitlab ci to run clippy (and _error out_ if changes
   presented) in both vcst/ and lib/: `cargo clippy --allow-no-vcs --fix` is the
   run I use and want to be warned if I haven't run.
- [ ] address clippy::pedantic, then roll it into above ci/cd stage:
      `cargo clippy --all -- -W clippy::pedantic`
- [ ] cleanup all the CLI string handling (the `String::from_utf8` and
      `expect(.*utf8` references) to use `String::from_utf8_lossy`
- [ ] cleanup some of the error enums that aren't being fully utilized (eg: some
  that default to map_err() to `Unknown`-fallbackish variants). This is because
  some of the better alternatives were only added _later_ (eg:
  `RepoLoadError::Stderr`) which could fix some

## Design Goals

**Goal**: answer 101 introspective questions about a repo/directory.

Questions I frequently[^freq] want to answered are now outlined as `VcstQuery`
enum of in the namesaked reference binary (at `./vcst/src/main.rs`).

The goal is to have coverage popular VCS I personally encounter regulalry, like
`git`, `hg`, `jj`.

## Tests

e2e tests of the CLI binary, in `vcst/tests/`, are the strategy for the moment;
they covery every API that `lib/` is meant to offer.

TODO consider either/or:

1. starting to teardown the vcst tests temp directory (see
   `vcst/tests/common/mod.rs` for the tempdir setup funcs that get called for
    every test in `vcst/tests/integration_test.rs`)
2. root-less container setup to easily run our e2e tests (so we can contain any
   potentially buggy teardown(), and not delte our own root directory).

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
