# vcst: VCSrusT

Tries to answer generic questions about a VCS repo, without making you think
about the particular flavor of VCS at play.

This repo lives at https://gitlab.com/jzacsh/vcst

## Development

In `lib/` just unit tests right now:

```sh
$ cargo watch -x test
```

## Design Goals

**Goal**: answer 101 introspective questions about a repo/directory.

Questions I frequently[^freq] want to answer;

1. is dir `foo/` a VCS repo?
   - if so, of which type?
1. given dir `foo/` a where is this repo's root?
1. is this repo dirty?
1. what unique ID (commit/rev) can you give me to refer to the repo's current
   state?
1. what unique bookmark (branch, tag) can you give me to refer to the repo's current?
1. dirty filepaths
1. HEAD's touched files
   - "touched" means "since last commit"
1. union of the last two
1. HEAD's touched as opposed to "last bookmark"

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
    https://gitlab.com/jzacsh/dotfiles and at
    https://gitlab.com/jzacsh/yabashlib and
    https://gitlab.com/jzacsh/jzach.gitlab.io

## License

Apache v2.
