// Copyright 2024. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::{sync::LazyLock, time::SystemTime};

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tokio::sync::RwLock;

use super::trait_config::{ConfigContentImpl, ConfigImpl};

static INSTANCE: LazyLock<RwLock<TestConfig>> = LazyLock::new(|| RwLock::new(TestConfig::new()));
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(default)]
#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
struct NotFullConfigContent {
    created_at: SystemTime,
    some_test_string: String,
}

impl Default for NotFullConfigContent {
    fn default() -> Self {
        Self {
            created_at: SystemTime::now(),
            some_test_string: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(default)]
#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
struct TestConfigContent {
    was_config_migrated: bool,
    created_at: SystemTime,
    some_test_string: String,
    some_test_bool: bool,
    some_test_int: i32,
}

impl Default for TestConfigContent {
    fn default() -> Self {
        Self {
            was_config_migrated: false,
            created_at: SystemTime::now(),
            some_test_string: "".to_string(),
            some_test_bool: false,
            some_test_int: 0,
        }
    }
}

impl ConfigContentImpl for TestConfigContent {}

struct TestConfig {
    content: TestConfigContent,
    app_handle: RwLock<Option<AppHandle>>,
}

impl ConfigImpl for TestConfig {
    type Config = TestConfigContent;

    fn current() -> &'static RwLock<Self> {
        &INSTANCE
    }

    fn new() -> Self {
        Self {
            content: TestConfigContent::default(),
            app_handle: RwLock::new(None),
        }
    }

    async fn _get_app_handle(&self) -> Option<AppHandle> {
        self.app_handle.read().await.clone()
    }

    async fn load_app_handle(&mut self, app_handle: AppHandle) {
        *self.app_handle.write().await = Some(app_handle);
    }

    fn _get_name() -> String {
        "config_test".to_string()
    }

    fn _get_content(&self) -> &Self::Config {
        &self.content
    }

    fn _get_content_mut(&mut self) -> &mut Self::Config {
        &mut self.content
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    // use std::fs;

    fn clear_config_file() {
        if TestConfig::_get_config_path().exists() {
            std::fs::remove_file(TestConfig::_get_config_path()).unwrap();
        }
    }

    fn before_each() {
        clear_config_file();
    }

    #[tokio::test]
    async fn test_saving_to_file() {
        let config = TestConfig::current().read().await;
        before_each();

        TestConfig::_save_config(config._get_content().clone()).unwrap();

        assert!(TestConfig::_get_config_path().exists());
    }

    // TODO: Bartosz: Fix this. Intermittant failures. Likely race conditions and deadlocks.
    // #[tokio::test]
    // async fn test_loading_from_file() {
    //     let config = TestConfig::current().read().await;
    //     before_each();

    //     TestConfig::_save_config(config._get_content().clone()).unwrap();

    //     let loaded_config = TestConfig::_load_config().unwrap();
    //     assert_eq!(config._get_content(), &loaded_config);
    // }

    // TODO: Bartosz: Fix this. It locks up for long periods
    // #[tokio::test]
    // async fn test_update_field() {
    //     let config = TestConfig::current().read().await;
    //     before_each();

    //     let initial_value = *config._get_content().some_test_bool();
    //     TestConfig::update_field(TestConfigContent::set_some_test_bool, !initial_value)
    //         .await
    //         .unwrap();

    //     assert_eq!(!initial_value, *config._get_content().some_test_bool());
    //     assert_eq!(
    //         !initial_value,
    //         *TestConfig::_load_config().unwrap().some_test_bool()
    //     );
    // }

    // #[tokio::test]
    // async fn test_if_loading_with_missing_files_is_handled() {
    //     before_each();

    //     let not_full_config = NotFullConfigContent {
    //         created_at: SystemTime::now(),
    //         some_test_string: "test".to_string(),
    //     };

    //     let not_full_config_serialized = serde_json::to_string_pretty(&not_full_config).unwrap();
    //     fs::write(TestConfig::_get_config_path(), not_full_config_serialized).unwrap();

    //     let loaded_config = TestConfig::_load_config().unwrap();

    //     assert_eq!(
    //         loaded_config.some_test_string,
    //         not_full_config.some_test_string
    //     );
    //     assert_eq!(loaded_config.created_at, not_full_config.created_at);
    //     assert_eq!(
    //         loaded_config.some_test_bool,
    //         TestConfigContent::default().some_test_bool
    //     );
    //     assert_eq!(
    //         loaded_config.some_test_int,
    //         TestConfigContent::default().some_test_int
    //     );
    // }
}
