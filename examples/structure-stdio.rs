extern crate chrono;
extern crate nonblock;
extern crate serde_json;

use chrono::offset::Utc;
use nonblock::NonBlockingReader;
use serde_json::Value;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{env, thread};

// Simple proof-of-concept interval-based JSON log structuring for stdio
//
// Run any process, and collect stdout and stderr once per second.
//   And for each interval, take the collected stdout and stderr along with the timestamp,
//   and print it as compact JSON in the form:
//
//   {"time":"2016-04-24T03:04:29.936957804Z","stdout":"output\nline2","stderr":"Some exception occurred"}`
fn main() {
    let mut args = env::args().skip(1);
    let path = args
        .next()
        .expect("Usage: structured-stdio <executable> [args...]");
    let mut cmd = Command::new(&path);
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    for arg in args {
        cmd.arg(arg);
    }
    let mut child = cmd.spawn().expect("Failed to spawn child");
    let stdout = child.stdout.take().expect("Failed to open runner STDOUT");
    let stderr = child.stderr.take().expect("Failed to open runner STDERR");

    let mut nb_stdout =
        NonBlockingReader::from_fd(stdout).expect("Failed to make stdout non-blocking");
    let mut nb_stderr =
        NonBlockingReader::from_fd(stderr).expect("Failed to make stderr non-blocking");

    loop {
        let time = Utc::now();
        let mut stdout_now = String::new();
        let mut stderr_now = String::new();
        let _ = nb_stdout
            .read_available_to_string(&mut stdout_now)
            .expect("Failed to read available stdout");
        let _ = nb_stderr
            .read_available_to_string(&mut stderr_now)
            .expect("Failed to read available stdout");

        // Don't bother printing anything if there was no stdout/stderr
        if !stdout_now.is_empty() || !stderr_now.is_empty() {
            let mut map = serde_json::Map::new();
            map.insert("time".into(), Value::from(time.to_string()));

            if !stdout_now.is_empty() {
                map.insert("stdout".into(), Value::from(stdout_now));
            }
            if !stderr_now.is_empty() {
                map.insert("stderr".into(), Value::from(stderr_now));
            }

            let json = serde_json::ser::to_string_pretty(&map).expect("Failed to serialize object");
            println!("{}", json);
        }

        // Bail if we've reached EOF for both stdin and stdout
        if nb_stdout.is_eof() && nb_stderr.is_eof() {
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
