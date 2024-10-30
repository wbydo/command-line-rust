use assert_cmd::Command as AssertCommand;
use std::process::Command;

#[test]
fn runs() {
    {
        let mut cmd = Command::new("ls");
        let res = cmd.output();
        assert!(res.is_ok());
    }
    {
        let mut cmd = AssertCommand::cargo_bin("command_line_rust").unwrap();
        cmd.assert().success();
    }
    {
        let mut cmd = AssertCommand::cargo_bin("command_line_rust").unwrap();
        cmd.assert().success().stdout("Hello, world!!!!!!\n");
    }
}

#[test]
fn true_ok() {
    let mut cmd = AssertCommand::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = AssertCommand::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
