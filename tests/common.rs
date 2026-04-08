#![cfg(test)]

use std::fs::File;
use std::path::{Path, PathBuf};

use serde::{
    self, Deserialize
};
use serde_json;
use brawl_api::Client;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct TestConfig {
    pub key: String,

    pub tags: TCTags,

    #[serde(default)]
    pub rankings_matrix: TCRankingsMatrix,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct TCTags {
    pub player: String,

    pub club: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Default)]
pub struct TCRankingsMatrix {
    pub country_sample_size: Option<usize>,
    pub country_sample_seed: Option<usize>,
    pub cursor_variant_count: Option<usize>,
    pub cursor_variant_seed: Option<usize>,
}

fn test_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    paths.push(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/test_config.json"));
    paths.push(PathBuf::from("tests/test_config.json"));
    paths.push(PathBuf::from("test_config.json"));

    paths
}

/// Opens test configuration.
///
/// Returns an `Err` when file opening/parsing fails for all candidate locations.
pub fn open_test_config() -> std::result::Result<TestConfig, String> {
    let mut attempted = Vec::new();
    let mut last_err = String::new();

    for path in test_config_paths() {
        attempted.push(path.display().to_string());

        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                last_err = e.to_string();
                continue;
            }
        };

        return serde_json::from_reader(file)
            .map_err(|e| {
                format!(
                    "Failed to parse {} (see tests/README.md): {}",
                    path.display(),
                    e
                )
            });
    }

    Err(format!(
        "Failed to open test_config.json (see tests/README.md). Tried: {}. Last error: {}",
        attempted.join(", "),
        last_err
    ))
}

/// Opens test configuration.
///
/// # Panics
///
/// Panics if opening was not possible, or parsing it failed.
pub fn open_test_config_panic() -> TestConfig {
    open_test_config().unwrap_or_else(|e| panic!("{}", e))
}

/// Creates a client with the test_config data.
#[allow(dead_code)]
pub fn create_test_client() -> Client {
    let config: TestConfig = open_test_config_panic();
    Client::new(&config.key)
}