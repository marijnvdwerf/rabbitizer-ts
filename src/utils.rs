// SPDX-FileCopyrightText: © 2022-2024 Decompollaborate
// SPDX-License-Identifier: MIT

use napi_derive::napi;

/// Utility functions for working with MIPS instructions
#[napi]
pub struct Utils;

#[napi]
impl Utils {
    /// Convert a sign-extended 16-bit immediate to a 32-bit signed integer
    #[napi(js_name = "sign_extend_immediate")]
    pub fn sign_extend_immediate(imm: u32) -> i32 {
        (imm as i16) as i32
    }

    /// Swap the endianness of a 32-bit word
    #[napi(js_name = "swap_endianness")]
    pub fn swap_endianness(word: u32) -> u32 {
        u32::from_le_bytes(word.to_be_bytes())
    }

    /// Check if a value is a power of 2
    #[napi(js_name = "is_power_of_two")]
    pub fn is_power_of_two(val: u32) -> bool {
        val != 0 && (val & (val - 1)) == 0
    }

    /// Get the name of a MIPS register by its O32 ABI index
    #[napi(js_name = "get_register_name_o32")]
    pub fn get_register_name_o32(reg_index: u32) -> String {
        match reg_index {
            0 => "$zero".to_string(),
            1 => "$at".to_string(),
            2 => "$v0".to_string(),
            3 => "$v1".to_string(),
            4 => "$a0".to_string(),
            5 => "$a1".to_string(),
            6 => "$a2".to_string(),
            7 => "$a3".to_string(),
            8 => "$t0".to_string(),
            9 => "$t1".to_string(),
            10 => "$t2".to_string(),
            11 => "$t3".to_string(),
            12 => "$t4".to_string(),
            13 => "$t5".to_string(),
            14 => "$t6".to_string(),
            15 => "$t7".to_string(),
            16 => "$s0".to_string(),
            17 => "$s1".to_string(),
            18 => "$s2".to_string(),
            19 => "$s3".to_string(),
            20 => "$s4".to_string(),
            21 => "$s5".to_string(),
            22 => "$s6".to_string(),
            23 => "$s7".to_string(),
            24 => "$t8".to_string(),
            25 => "$t9".to_string(),
            26 => "$k0".to_string(),
            27 => "$k1".to_string(),
            28 => "$gp".to_string(),
            29 => "$sp".to_string(),
            30 => "$fp".to_string(),
            31 => "$ra".to_string(),
            _ => format!("$invalid_{}", reg_index),
        }
    }

    /// Get the numeric name of a MIPS register by its index
    #[napi(js_name = "get_register_name_numeric")]
    pub fn get_register_name_numeric(reg_index: u32) -> String {
        if reg_index < 32 {
            format!("${}", reg_index)
        } else {
            format!("$invalid_{}", reg_index)
        }
    }
}
