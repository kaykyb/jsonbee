use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_encode_json() {
    let mut cmd = Command::cargo_bin("jsonbee").unwrap();
    cmd.arg("encode")
        .write_stdin(r#"{"name":"John","age":30}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("d3:agei30e4:name4:Johne"));
}

#[test]
fn test_decode_bencode() {
    let mut cmd = Command::cargo_bin("jsonbee").unwrap();
    cmd.arg("decode")
        .write_stdin("d4:name4:John3:agei30ee")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"{"age":30,"name":"John"}"#).normalize());
}
