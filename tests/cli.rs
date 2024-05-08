use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// test fails with no args
#[test]
fn fails_with_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("double_echo_rust")?;
    cmd.assert().failure();
    Ok(())
}

// test runs with args
#[test]
fn runs_with_some_args() -> TestResult {
    let mut cmd = Command::cargo_bin("double_echo_rust")?;
    cmd.arg("arg").assert().success();
    Ok(())
}

// test prints twice

// test prints twice with seperator

// test fails when seperator is -- or more dashes
