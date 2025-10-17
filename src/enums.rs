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

/// Memory access type enumeration
#[napi]
pub enum AccessType {
    Invalid,
    Byte,
    Short,
    Word,
    Doubleword,
    Quadword,
    Float,
    Doublefloat,
    WordLeft,
    WordRight,
    Doublewordleft,
    Doublewordright,
}

/// Instruction suffix type enumeration
#[napi]
pub enum InstrSuffix {
    None,
    R5900Xyzw,
}

/// Instruction ID type enumeration
#[napi]
pub enum InstrIdType {
    AllInvalid,
    CpuInvalid,
    CpuNormal,
    CpuSpecial,
    CpuRegimm,
    CpuCop0,
    CpuCop0Bc0,
    CpuCop0Tlb,
    CpuCop1,
    CpuCop1Bc1,
    CpuCop1Fpus,
    CpuCop1Fpud,
    CpuCop1Fpuw,
    CpuCop1Fpul,
    CpuCop2,
    RspInvalid,
    RspNormal,
    RspNormalLwc2,
    RspNormalSwc2,
    RspSpecial,
    RspRegimm,
    RspCop0,
    RspCop1,
    RspCop2,
    RspCop2Vu,
    R3000gteInvalid,
    R3000gteNormal,
    R3000gteSpecial,
    R3000gteRegimm,
    R3000gteCop0,
    R3000gteCop1,
    R3000gteCop2,
    R3000gteCop2Gte,
    R4000allegrexInvalid,
    R4000allegrexNormal,
    R4000allegrexSpecial,
    R4000allegrexSpecialRs,
    R4000allegrexSpecialSa,
    R4000allegrexRegimm,
    R4000allegrexSpecial2,
    R4000allegrexSpecial3,
    R4000allegrexSpecial3Bshfl,
    R4000allegrexCop0,
    R4000allegrexCop0Bc0,
    R4000allegrexCop0Tlb,
    R4000allegrexCop1,
    R4000allegrexCop1Bc1,
    R4000allegrexCop1Fpus,
    R4000allegrexCop1Fpuw,
    R4000allegrexCop2,
    R4000allegrexCop2Bc2,
    R4000allegrexCop2Mfhc2,
    R4000allegrexCop2Mfhc2P,
    R4000allegrexCop2Mfhc2PS,
    R4000allegrexCop2Mthc2,
    R4000allegrexVfpu0,
    R4000allegrexVfpu1,
    R4000allegrexVfpu3,
    R4000allegrexVfpu4,
    R4000allegrexVfpu4Fmt0,
    R4000allegrexVfpu4Fmt0Fmt0,
    R4000allegrexVfpu4Fmt0Fmt2,
    R4000allegrexVfpu4Fmt0Fmt3,
    R4000allegrexVfpu4Fmt0Rnd,
    R4000allegrexVfpu4Fmt0Cvtflt,
    R4000allegrexVfpu4Fmt0Cvtint,
    R4000allegrexVfpu4Fmt0Fmt8,
    R4000allegrexVfpu4Fmt0Fmt9,
    R4000allegrexVfpu4Fmt0Control,
    R4000allegrexVfpu4Fmt0Color,
    R4000allegrexVfpu4Fmt0Cst,
    R4000allegrexVfpu4Fmt2,
    R4000allegrexVfpu4Fmt2Cndmove,
    R4000allegrexVfpu5,
    R4000allegrexVfpu6,
    R4000allegrexVfpu6Fmt7,
    R4000allegrexVfpu6Fmt7Fmt0,
    R4000allegrexVfpu7,
    R4000allegrexQuadlr,
    R5900Invalid,
    R5900Normal,
    R5900Special,
    R5900Regimm,
    R5900Cop0,
    R5900Cop0Tlb,
    R5900Cop1,
    R5900Cop1Fpus,
    R5900Cop2,
    R5900Cop2Nohighbit,
    R5900Cop2Bc2,
    R5900Cop2Special1,
    R5900Cop2Special2,
    R5900Cop2Viwr,
    R5900Mmi,
    R5900Mmi0,
    R5900Mmi1,
    R5900Mmi2,
    R5900Mmi3,
    R5900MmiPmfhl,
    R5900MmiPmthl,
}

/// Operand type enumeration
#[napi]
pub enum OperandType {
    AllInvalid,
    CpuRs,
    CpuRt,
    CpuRd,
    CpuSa,
    CpuZero,
    CpuCop0d,
    CpuFs,
    CpuFt,
    CpuFd,
    CpuCop1cs,
    CpuCop2t,
    CpuCop2cd,
    CpuOp,
    CpuHint,
    CpuCode,
    CpuCodeLower,
    CpuCopraw,
    CpuLabel,
    CpuImmediate,
    CpuBranchTargetLabel,
    CpuImmediateBase,
    CpuMaybeRdRs,
    RspRs,
    RspRt,
    RspRd,
    RspCop0d,
    RspCop2t,
    RspCop2cd,
    RspVs,
    RspVt,
    RspVd,
    RspHint,
    RspVtElementhigh,
    RspVtElementlow,
    RspVdDe,
    RspVsIndex,
    RspOffsetRs,
    RspImmediateBase,
    RspMaybeRdRs,
    R3000gteGbg,
    R3000gteSf,
    R3000gteMx,
    R3000gteV,
    R3000gteCv,
    R3000gteLm,
    R4000allegrexSVs,
    R4000allegrexSVt,
    R4000allegrexSVd,
    R4000allegrexSVtImm,
    R4000allegrexSVdImm,
    R4000allegrexPVs,
    R4000allegrexPVt,
    R4000allegrexPVd,
    R4000allegrexTVs,
    R4000allegrexTVt,
    R4000allegrexTVd,
    R4000allegrexQVs,
    R4000allegrexQVt,
    R4000allegrexQVd,
    R4000allegrexQVtImm,
    R4000allegrexMpVs,
    R4000allegrexMpVt,
    R4000allegrexMpVd,
    R4000allegrexMpVsTranspose,
    R4000allegrexMtVs,
    R4000allegrexMtVt,
    R4000allegrexMtVd,
    R4000allegrexMtVsTranspose,
    R4000allegrexMqVs,
    R4000allegrexMqVt,
    R4000allegrexMqVd,
    R4000allegrexMqVsTranspose,
    R4000allegrexCop2cs,
    R4000allegrexCop2cd,
    R4000allegrexPos,
    R4000allegrexSize,
    R4000allegrexSizePlusPos,
    R4000allegrexImm3,
    R4000allegrexOffset14Base,
    R4000allegrexOffset14BaseMaybeWb,
    R4000allegrexVcmpCond,
    R4000allegrexVcmpCondSMaybeVsMaybeVt,
    R4000allegrexVcmpCondPMaybeVsMaybeVt,
    R4000allegrexVcmpCondTMaybeVsMaybeVt,
    R4000allegrexVcmpCondQMaybeVsMaybeVt,
    R4000allegrexVconstant,
    R4000allegrexPowerOfTwo,
    R4000allegrexVfpuCcBit,
    R4000allegrexBn,
    R4000allegrexInt16,
    R4000allegrexFloat16,
    R4000allegrexPVrotCode,
    R4000allegrexTVrotCode,
    R4000allegrexQVrotCode,
    R4000allegrexRpx,
    R4000allegrexRpy,
    R4000allegrexRpz,
    R4000allegrexRpw,
    R4000allegrexWpx,
    R4000allegrexWpy,
    R4000allegrexWpz,
    R4000allegrexWpw,
    R5900I,
    R5900Q,
    R5900R,
    R5900Acc,
    R5900AccXyzw,
    R5900Vfs,
    R5900Vft,
    R5900Vfd,
    R5900VfsXyzw,
    R5900VftXyzw,
    R5900VfdXyzw,
    R5900Vfsn,
    R5900Vftn,
    R5900Vfdn,
    R5900Vfsl,
    R5900Vftl,
    R5900Vfdl,
    R5900Vfsm,
    R5900Vftm,
    R5900Vfdm,
    R5900Vis,
    R5900Vit,
    R5900Vid,
    R5900VisPredecr,
    R5900VitPredecr,
    R5900VidPredecr,
    R5900VisPostincr,
    R5900VitPostincr,
    R5900VidPostincr,
    R5900VisParenthesis,
    R5900Immediate5,
    R5900Immediate15,
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

impl Into<rabbitizer::AccessType> for AccessType {
    fn into(self) -> rabbitizer::AccessType {
        match self {
            AccessType::Invalid => rabbitizer::AccessType::INVALID,
            AccessType::Byte => rabbitizer::AccessType::BYTE,
            AccessType::Short => rabbitizer::AccessType::SHORT,
            AccessType::Word => rabbitizer::AccessType::WORD,
            AccessType::Doubleword => rabbitizer::AccessType::DOUBLEWORD,
            AccessType::Quadword => rabbitizer::AccessType::QUADWORD,
            AccessType::Float => rabbitizer::AccessType::FLOAT,
            AccessType::Doublefloat => rabbitizer::AccessType::DOUBLEFLOAT,
            AccessType::WordLeft => rabbitizer::AccessType::WORD_LEFT,
            AccessType::WordRight => rabbitizer::AccessType::WORD_RIGHT,
            AccessType::Doublewordleft => rabbitizer::AccessType::DOUBLEWORD_LEFT,
            AccessType::Doublewordright => rabbitizer::AccessType::DOUBLEWORD_RIGHT,
        }
    }
}

impl From<rabbitizer::AccessType> for AccessType {
    fn from(val: rabbitizer::AccessType) -> Self {
        match val {
            rabbitizer::AccessType::INVALID => AccessType::Invalid,
            rabbitizer::AccessType::BYTE => AccessType::Byte,
            rabbitizer::AccessType::SHORT => AccessType::Short,
            rabbitizer::AccessType::WORD => AccessType::Word,
            rabbitizer::AccessType::DOUBLEWORD => AccessType::Doubleword,
            rabbitizer::AccessType::QUADWORD => AccessType::Quadword,
            rabbitizer::AccessType::FLOAT => AccessType::Float,
            rabbitizer::AccessType::DOUBLEFLOAT => AccessType::Doublefloat,
            rabbitizer::AccessType::WORD_LEFT => AccessType::WordLeft,
            rabbitizer::AccessType::WORD_RIGHT => AccessType::WordRight,
            rabbitizer::AccessType::DOUBLEWORD_LEFT => AccessType::Doublewordleft,
            rabbitizer::AccessType::DOUBLEWORD_RIGHT => AccessType::Doublewordright,
            rabbitizer::AccessType::MAX => AccessType::Invalid,
        }
    }
}

impl Into<rabbitizer::InstrSuffix> for InstrSuffix {
    fn into(self) -> rabbitizer::InstrSuffix {
        match self {
            InstrSuffix::None => rabbitizer::InstrSuffix::ALL_NONE,
            InstrSuffix::R5900Xyzw => rabbitizer::InstrSuffix::R5900_xyzw,
        }
    }
}

impl From<rabbitizer::InstrSuffix> for InstrSuffix {
    fn from(val: rabbitizer::InstrSuffix) -> Self {
        match val {
            rabbitizer::InstrSuffix::ALL_NONE => InstrSuffix::None,
            rabbitizer::InstrSuffix::R5900_xyzw => InstrSuffix::R5900Xyzw,
            rabbitizer::InstrSuffix::ALL_MAX => InstrSuffix::None,
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

// Note: InstrIdType and OperandType are large enums with 100+ variants
// They are exposed as enums for type checking but work with u32 values in practice
