use assert_cmd::Command;
use predicates::prelude::*;
use std::io::{Read, Write};
use tempfile::NamedTempFile;

#[test]
fn encode_stdin_to_stdout() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("encode")
        .write_stdin("hello world")
        .assert()
        .success()
        .stdout(predicate::str::contains("aGVsbG8gd29ybGQ="));
}

#[test]
fn decode_stdin_to_stdout() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("decode")
        .write_stdin("aGVsbG8gd29ybGQ=")
        .assert()
        .success()
        .stdout("hello world");
}

#[test]
fn encode_file_to_file() {
    let mut input = NamedTempFile::new().unwrap();
    let mut output = NamedTempFile::new().unwrap();

    write!(input, "hello world").unwrap();

    let mut cmd = Command::cargo_bin("base64-cli").unwrap();
    cmd.arg("encode")
        .arg("--input")
        .arg(input.path())
        .arg("--output")
        .arg(output.path())
        .assert()
        .success();

    let mut encoded = String::new();
    output.read_to_string(&mut encoded).unwrap();

    assert_eq!(encoded.trim(), "aGVsbG8gd29ybGQ=");
}

#[test]
fn decode_file_to_file() {
    let mut input = NamedTempFile::new().unwrap();
    let mut output = NamedTempFile::new().unwrap();

    write!(input, "aGVsbG8gd29ybGQ=").unwrap();

    let mut cmd = Command::cargo_bin("base64-cli").unwrap();
    cmd.arg("decode")
        .arg("--input")
        .arg(input.path())
        .arg("--output")
        .arg(output.path())
        .assert()
        .success();

    let mut decoded = String::new();
    output.read_to_string(&mut decoded).unwrap();

    assert_eq!(decoded, "hello world");
}

#[test]
fn encode_with_wrap() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("encode")
        .arg("--wrap")
        .arg("4")
        .write_stdin("hello world")
        .assert()
        .success()
        .stdout(predicate::str::contains("aGVs\nbG8g\nd29y\nbGQ="));
}
#[test]
fn encode_empty_input() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("encode")
        .write_stdin("")
        .assert()
        .success()
        .stdout("");
}
#[test]
fn decode_empty_input() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("decode")
        .write_stdin("")
        .assert()
        .success()
        .stdout("");
}
#[test]
fn decode_invalid_input() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("decode")
        .write_stdin("### not base64 ###")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Decode error"));
}
#[test]
fn encode_wrap_zero_is_ignored() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("encode")
        .arg("--wrap")
        .arg("0")
        .write_stdin("hello world")
        .assert()
        .success()
        .stdout("aGVsbG8gd29ybGQ=");
}
#[test]
fn encode_wrap_large_value() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("encode")
        .arg("--wrap")
        .arg("999")
        .write_stdin("hello world")
        .assert()
        .success()
        .stdout("aGVsbG8gd29ybGQ=");
}
#[test]
fn roundtrip_stdin() {
    let mut encode = Command::cargo_bin("base64-cli").unwrap();
    let encoded = encode
        .arg("encode")
        .write_stdin("The quick brown fox")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let mut decode = Command::cargo_bin("base64-cli").unwrap();
    decode
        .arg("decode")
        .write_stdin(encoded)
        .assert()
        .success()
        .stdout("The quick brown fox");
}
