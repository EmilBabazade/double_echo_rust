use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn fails_with_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("double_echo_rust")?;
    cmd.assert().failure();
    Ok(())
}

#[test]
fn runs_with_some_args() -> TestResult {
    let mut cmd = Command::cargo_bin("double_echo_rust")?;
    cmd.arg("arg").assert().success();
    Ok(())
}

fn run(args: &[&str], expected_result: String) -> TestResult {
    Command::cargo_bin("double_echo_rust")?
        .args(args)
        .assert()
        .success()
        .stdout(expected_result);
    Ok(())
}

#[test]
fn prints_twice() -> TestResult {
    run(&["foo", "baz"], String::from("foo baz\nfoo baz"))
}

#[test]
fn prints_twice_with_seperator() -> TestResult {
    run(
        &["foo", "baz", "-s", "*******"],
        String::from("foo baz\n*******\nfoo baz"),
    )
}

#[test]
fn fails_when_seperator_is_2_dashes_or_more() -> TestResult {
    Command::cargo_bin("double_echo_rust")?
        .args(["efefef", "-s", "---"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Found argument '--' "));
    Ok(())
}
