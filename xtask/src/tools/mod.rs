use crate::utilities;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;

pub struct RustVersionMeta {
    pub host: String,
    pub commit_hash: String,
}

pub fn find_rust_version_meta() -> RustVersionMeta {
    let mut host = None;
    let mut commit_hash = None;

    utilities::run_command_and_stream_output(Command::new("rustc").arg("-vV"), |stdout| {
        for line in BufReader::new(stdout).lines() {
            let line = line.unwrap();

            if let Some(host_value) = line.strip_prefix("host: ") {
                host = Some(host_value.to_string());
            } else if let Some(commit_hash_value) = line.strip_prefix("commit-hash: ") {
                commit_hash = Some(commit_hash_value.to_string());
            }
        }
    });

    RustVersionMeta {
        host: host.unwrap(),
        commit_hash: commit_hash.unwrap(),
    }
}

pub fn find_rust_sysroot() -> PathBuf {
    let mut path_buffer = String::from_utf8(utilities::run_command_and_get_output(
        Command::new("rustc").args(["--print", "sysroot"]),
    ))
    .unwrap();

    path_buffer.pop();

    PathBuf::from(path_buffer)
}
