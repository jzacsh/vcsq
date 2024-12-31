# vcst: VCSrusT - Version Control System (VCS) utils in Rust

Tries to answer generic questions about a VCS repo, without making you think
about the particular flavor of VCS at play.

This repo lives at <https://gitlab.com/jzacsh/vcst>

**STATUS**: early days, mostly boilerplate/setup, with only some functionality
ported in from the originals (see "design goals" section).

## Development

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

## License

Apache v2.
