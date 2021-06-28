// Copyright 2018-2021 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod cargo_contract;

use psutil::process::processes;
use serde_json;
use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    process::Command,
};

/// Returns the full path to the ink! example directory for `example`.
pub fn example_path(example: &str) -> PathBuf {
    let examples_path = std::env::var("INK_EXAMPLES_PATH")
        .expect("env variable INK_EXAMPLES_PATH must be set");
    let path = PathBuf::from(examples_path);
    path.join(example)
}

/// Extracts the `source.hash` field from the contract bundle.
pub fn extract_hash_from_contract_bundle(path: &PathBuf) -> String {
    let file =
        File::open(path).expect(&format!("Contract file at {:?} does not exist", path));
    let reader = BufReader::new(file);
    let json: serde_json::Value =
        serde_json::from_reader(reader).expect("JSON is not well-formatted");
    json.get("source")
        .expect("Unable to get 'source' field from contract JSON")
        .get("hash")
        .expect("Unable to get 'hash' field from contract JSON")
        .to_string()
        .trim_matches('"')
        .to_string()
}

/// Asserts that the `canvas` process is running.
pub fn assert_canvas_node_running() {
    let processes = processes().expect("can't get processes");
    let canvas_node_running = processes
        .into_iter()
        .filter_map(|pr| pr.ok())
        .map(|p| p.cmdline())
        .filter_map(|cmdline| cmdline.ok())
        .filter_map(|opt| opt)
        .any(|str| str.contains("canvas ") || str.contains("canvas-rand-extension "));
    assert!(
        canvas_node_running,
        "ERROR: The canvas node is not running!"
    );
}

/// Returns true if the `canvas-node` log under `/tmp/canvas.log` contains `msg`.
pub fn canvas_log_contains(msg: &str) -> bool {
    let output = Command::new("grep")
        .arg("-q")
        .arg(msg)
        .arg("/tmp/canvas.log")
        .spawn()
        .map_err(|err| format!("ERROR while executing `grep` with {:?}: {:?}", msg, err))
        .expect("failed to execute process")
        .wait_with_output()
        .expect("failed to receive output");
    output.status.success()
}
