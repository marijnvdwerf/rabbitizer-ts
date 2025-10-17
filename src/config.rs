// SPDX-FileCopyrightText: © 2022-2024 Decompollaborate
// SPDX-License-Identifier: MIT

use napi_derive::napi;

/// Global configuration for instruction disassembly and analysis
///
/// Note: Configuration is limited in the current Rust bindings.
/// Full configuration support will be added in a future update.
#[napi]
pub struct Config;

#[napi]
impl Config {
    /// Returns a placeholder message about configuration
    #[napi]
    pub fn info() -> String {
        "Rabbitizer configuration support is limited in the Rust bindings. \
         Full configuration will be added in a future update."
            .to_string()
    }
}
