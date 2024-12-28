# vcst: VCSrusT - Version Control System (VCS) utils in Rust

Tries to answer generic questions about a VCS repo, without making you think
about the particular flavor of VCS at play.

This repo lives at <https://gitlab.com/jzacsh/vcst>

## Development

In `lib/` just unit tests right now:

```sh
$ cd lib && cargo watch -x test
# ...
```

In `vcst/` just runt he binary:

```sh
$ cd vcst && cargo watch -x run
# ...
```

## Design Goals

**Goal**: answer 101 introspective questions about a repo/directory.

Questions I frequently[^freq] want to answered are now outlined as `VcstQuery`
enum of in the namesaked reference binary (at `./vcst/src/main.rs`).

The goal is to have coverage popular VCS I personally encounter regulalry, like
`git`, `hg`, `jj`.

## Tests

e2e tests are going to be the easiest to maintain and leverage, and I imagine
that will mean simply a temp filesystem.

TODO figure out how to get root-less container setup easily so we can contain
some tests (because I don't want a unit-test's buggy teardown() func to delete
my root directory, for example).

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
