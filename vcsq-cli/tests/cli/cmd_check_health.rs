use assert_cmd::Command;
use predicates::prelude::*;

// TODO: (tests) figure manipulate PATH so only some bins are available, then add tests for all the
// combinations. Maybe:
// - 1. figure out how to manipulate the $PATH assert_cmd::Command makes aavilable to its pid group
// - 2. setup a bin_dir using TestDirs helpers
// - 3. symlink the real system's VCS's into said bin_dir
// - 4. point assert_cmd to ONLY that path (_and_ the local debug build?)

#[test]
fn full_pass() {
    let mut cmd = Command::cargo_bin("vcsq").unwrap();

    let assert = cmd.arg("check-health").assert();
    assert
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains("PASS: check for Git:"))
        .stdout(predicate::str::contains("git version"))
        .stdout(predicate::str::contains("PASS: check for Mercurial:"))
        .stdout(predicate::str::contains(
            "Mercurial Distributed SCM (version",
        ))
        .stdout(predicate::str::contains("PASS: check for Jujutsu:"))
        .stdout(predicate::str::contains("jj "));
}
