// SPDX-FileCopyrightText: © 2022-2024 Decompollaborate
// SPDX-License-Identifier: MIT

use napi_derive::napi;

/// Instruction category enumeration
#[napi]
pub enum InstrCategory {
    /// Standard MIPS CPU instructions
    CPU,
    /// N64 RSP (Reality Signal Processor) instructions
    RSP,
    /// PlayStation R3000 GTE (Graphics Transformation Engine)
    R3000GTE,
    /// PlayStation Portable R4000 ALLEGREX instructions
    R4000ALLEGREX,
    /// PlayStation 2 R5900 (Emotion Engine) instructions
    R5900,
}

impl Into<rabbitizer::InstrCategory> for InstrCategory {
    fn into(self) -> rabbitizer::InstrCategory {
        match self {
            InstrCategory::CPU => rabbitizer::InstrCategory::CPU,
            InstrCategory::RSP => rabbitizer::InstrCategory::RSP,
            InstrCategory::R3000GTE => rabbitizer::InstrCategory::R3000GTE,
            InstrCategory::R4000ALLEGREX => rabbitizer::InstrCategory::R4000ALLEGREX,
            InstrCategory::R5900 => rabbitizer::InstrCategory::R5900,
        }
    }
}

impl From<rabbitizer::InstrCategory> for InstrCategory {
    fn from(cat: rabbitizer::InstrCategory) -> Self {
        match cat {
            rabbitizer::InstrCategory::CPU => InstrCategory::CPU,
            rabbitizer::InstrCategory::RSP => InstrCategory::RSP,
            rabbitizer::InstrCategory::R3000GTE => InstrCategory::R3000GTE,
            rabbitizer::InstrCategory::R4000ALLEGREX => InstrCategory::R4000ALLEGREX,
            rabbitizer::InstrCategory::R5900 => InstrCategory::R5900,
            rabbitizer::InstrCategory::MAX => InstrCategory::CPU,
        }
    }
}

/// ABI (Application Binary Interface) register naming
#[napi]
pub enum Abi {
    /// Numeric register names (e.g., $0, $1, $2)
    NUMERIC,
    /// O32 ABI register names
    O32,
    /// N32 ABI register names
    N32,
    /// N64 ABI register names
    N64,
}

impl Into<rabbitizer::Abi> for Abi {
    fn into(self) -> rabbitizer::Abi {
        match self {
            Abi::NUMERIC => rabbitizer::Abi::NUMERIC,
            Abi::O32 => rabbitizer::Abi::O32,
            Abi::N32 => rabbitizer::Abi::N32,
            Abi::N64 => rabbitizer::Abi::N64,
        }
    }
}

impl From<rabbitizer::Abi> for Abi {
    fn from(abi: rabbitizer::Abi) -> Self {
        match abi {
            rabbitizer::Abi::NUMERIC => Abi::NUMERIC,
            rabbitizer::Abi::O32 => Abi::O32,
            rabbitizer::Abi::N32 => Abi::N32,
            rabbitizer::Abi::N64 => Abi::N64,
            rabbitizer::Abi::MAX => Abi::NUMERIC,
        }
    }
}

/// CPU register ABI enumeration (O32 convention)
#[napi]
pub enum GprO32 {
    Zero,
    At,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    Gp,
    Sp,
    Fp,
    Ra,
}

impl From<u32> for GprO32 {
    fn from(val: u32) -> Self {
        match val {
            0 => GprO32::Zero,
            1 => GprO32::At,
            2 => GprO32::V0,
            3 => GprO32::V1,
            4 => GprO32::A0,
            5 => GprO32::A1,
            6 => GprO32::A2,
            7 => GprO32::A3,
            8 => GprO32::T0,
            9 => GprO32::T1,
            10 => GprO32::T2,
            11 => GprO32::T3,
            12 => GprO32::T4,
            13 => GprO32::T5,
            14 => GprO32::T6,
            15 => GprO32::T7,
            16 => GprO32::S0,
            17 => GprO32::S1,
            18 => GprO32::S2,
            19 => GprO32::S3,
            20 => GprO32::S4,
            21 => GprO32::S5,
            22 => GprO32::S6,
            23 => GprO32::S7,
            24 => GprO32::T8,
            25 => GprO32::T9,
            26 => GprO32::K0,
            27 => GprO32::K1,
            28 => GprO32::Gp,
            29 => GprO32::Sp,
            30 => GprO32::Fp,
            31 => GprO32::Ra,
            _ => GprO32::Zero,
        }
    }
}
