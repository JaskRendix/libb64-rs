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

#[test]
fn parallel_roundtrip_stdin() {
    let mut encode = Command::cargo_bin("base64-cli").unwrap();
    let encoded = encode
        .arg("encode")
        .arg("--parallel")
        .write_stdin("parallel test 123")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let mut decode = Command::cargo_bin("base64-cli").unwrap();
    decode
        .arg("decode")
        .arg("--parallel")
        .write_stdin(encoded)
        .assert()
        .success()
        .stdout("parallel test 123");
}

#[test]
fn decode_check_valid() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("decode")
        .arg("--check")
        .write_stdin("aGVsbG8=")
        .assert()
        .success()
        .stdout("");
}

#[test]
fn decode_check_invalid() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("decode")
        .arg("--check")
        .write_stdin("not_base64!!")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Decode error"));
}

#[test]
fn decode_with_whitespace() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("decode")
        .write_stdin("aG Vs bG 8g d2 9y bG Q=")
        .assert()
        .success()
        .stdout("hello world");
}

#[test]
fn decode_invalid_padding_too_much() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("decode")
        .write_stdin("abcd===")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Decode error"));
}

#[test]
fn decode_invalid_padding_middle() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("decode")
        .write_stdin("ab=cd")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Decode error"));
}

#[test]
fn decode_parallel_invalid() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("decode")
        .arg("--parallel")
        .write_stdin("###")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Decode error"));
}

#[test]
fn encode_large_stdin() {
    let input = "A".repeat(20000);

    let mut cmd = Command::cargo_bin("base64-cli").unwrap();
    cmd.arg("encode")
        .write_stdin(input) // <-- FIXED
        .assert()
        .success()
        .stdout(predicate::str::is_match("^[A-Za-z0-9+/=\\n]+$").unwrap());
}

#[test]
fn encode_wrap_no_trailing_newline() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    let out = cmd
        .arg("encode")
        .arg("--wrap")
        .arg("4")
        .write_stdin("hello")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    assert!(!out.ends_with(b"\n"));
}

#[test]
fn decode_parallel_empty() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("decode")
        .arg("--parallel")
        .write_stdin("")
        .assert()
        .success()
        .stdout("");
}

#[test]
fn encode_binary_data() {
    let mut input = NamedTempFile::new().unwrap();
    input.write_all(&[0, 159, 255, 10, 33]).unwrap();

    let mut output = NamedTempFile::new().unwrap();

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

    assert_eq!(encoded.trim(), "AJ//CiE=");
}

#[test]
fn encode_url_safe_stdin_to_stdout() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("encode")
        .arg("--url-safe")
        .write_stdin("hello world")
        .assert()
        .success()
        .stdout("aGVsbG8gd29ybGQ=");
}

#[test]
fn encode_url_safe_file_to_file() {
    let mut input = NamedTempFile::new().unwrap();
    let mut output = NamedTempFile::new().unwrap();

    write!(input, "hello world").unwrap();

    let mut cmd = Command::cargo_bin("base64-cli").unwrap();
    cmd.arg("encode")
        .arg("--url-safe")
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
fn encode_url_safe_parallel() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("encode")
        .arg("--parallel")
        .arg("--url-safe")
        .write_stdin("parallel urlsafe 123")
        .assert()
        .success()
        .stdout(predicate::str::contains("cGFyYWxsZWwgdXJsc2FmZSAxMjM"));
}

#[test]
fn decode_url_safe_stdin_to_stdout() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    // URL-safe form of "hello world"
    cmd.arg("decode")
        .arg("--url-safe")
        .write_stdin("aGVsbG8gd29ybGQ=")
        .assert()
        .success()
        .stdout("hello world");
}

#[test]
fn decode_url_safe_mixed_alphabet() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    cmd.arg("decode")
        .arg("--url-safe")
        .write_stdin("aGVsbG8gd29ybGQ=") // same as standard
        .assert()
        .success()
        .stdout("hello world");
}

#[test]
fn decode_url_safe_parallel() {
    let mut cmd = Command::cargo_bin("base64-cli").unwrap();

    // URL-safe encoding of "parallel test"
    cmd.arg("decode")
        .arg("--parallel")
        .arg("--url-safe")
        .write_stdin("cGFyYWxsZWwgdGVzdA==")
        .assert()
        .success()
        .stdout("parallel test");
}
