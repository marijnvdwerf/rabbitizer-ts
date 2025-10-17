// SPDX-FileCopyrightText: © 2022-2024 Decompollaborate
// SPDX-License-Identifier: MIT

#![allow(non_snake_case)]

use napi_derive::napi;

mod instruction;
mod enums;
mod config;
mod utils;

pub use instruction::Instruction;
pub use enums::*;
pub use config::*;
pub use utils::*;

#[napi]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[napi(object)]
pub struct VersionInfo {
    pub version: String,
    pub name: String,
}

#[napi]
pub fn get_version_info() -> VersionInfo {
    VersionInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: "rabbitizer".to_string(),
    }
}
