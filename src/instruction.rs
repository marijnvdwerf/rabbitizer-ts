// SPDX-FileCopyrightText: © 2022-2024 Decompollaborate
// SPDX-License-Identifier: MIT

use napi_derive::napi;
use rabbitizer::Instruction as RustInstruction;
use rabbitizer::InstrCategory;

#[napi]
pub struct Instruction {
    inner: RustInstruction,
}

#[napi]
impl Instruction {
    /// Creates a new Instruction from a 32-bit word
    ///
    /// # Arguments
    /// * `word` - The 32-bit instruction word
    /// * `vram` - The virtual address (optional, defaults to 0)
    /// * `category` - The instruction category as a string: "cpu", "rsp", "r3000gte", "r4000allegrex", "r5900" (optional)
    #[napi(constructor)]
    pub fn new(word: u32, vram: Option<u32>, category: Option<String>) -> napi::Result<Self> {
        let vram = vram.unwrap_or(0);
        let category_enum = match category.as_deref().unwrap_or("cpu") {
            "rsp" => InstrCategory::RSP,
            "r3000gte" => InstrCategory::R3000GTE,
            "r4000allegrex" => InstrCategory::R4000ALLEGREX,
            "r5900" => InstrCategory::R5900,
            "cpu" | _ => InstrCategory::CPU,
        };

        Ok(Instruction {
            inner: RustInstruction::new(word, vram, category_enum),
        })
    }

    /// Disassembles the instruction to assembly string
    #[napi]
    pub fn disassemble(&self, immediate_override: Option<String>, vram: Option<i32>) -> String {
        let vram = vram.unwrap_or(self.inner.vram as i32);
        self.inner.disassemble(immediate_override.as_deref(), vram)
    }

    /// Gets the instruction word
    #[napi(getter)]
    pub fn word(&self) -> u32 {
        self.inner.raw()
    }

    /// Gets the virtual address (vram)
    #[napi(getter)]
    pub fn vram(&self) -> u32 {
        self.inner.vram
    }

    /// Gets the instruction category as a string
    #[napi(getter)]
    pub fn category(&self) -> String {
        match self.inner.category {
            InstrCategory::CPU => "cpu".to_string(),
            InstrCategory::RSP => "rsp".to_string(),
            InstrCategory::R3000GTE => "r3000gte".to_string(),
            InstrCategory::R4000ALLEGREX => "r4000allegrex".to_string(),
            InstrCategory::R5900 => "r5900".to_string(),
            InstrCategory::MAX => "cpu".to_string(),
        }
    }

    // ==================== Bit field getters ====================

    #[napi]
    pub fn get_opcode(&self) -> u32 {
        self.inner.get_opcode()
    }

    #[napi]
    pub fn get_rs(&self) -> u32 {
        self.inner.get_rs()
    }

    #[napi]
    pub fn get_rt(&self) -> u32 {
        self.inner.get_rt()
    }

    #[napi]
    pub fn get_rd(&self) -> u32 {
        self.inner.get_rd()
    }

    #[napi]
    pub fn get_sa(&self) -> u32 {
        self.inner.get_sa()
    }

    #[napi]
    pub fn get_function(&self) -> u32 {
        self.inner.get_function()
    }

    #[napi]
    pub fn get_immediate(&self) -> u32 {
        self.inner.get_immediate() as u32
    }

    #[napi]
    pub fn get_instr_index(&self) -> u32 {
        self.inner.get_instr_index()
    }

    #[napi]
    pub fn get_code(&self) -> u32 {
        self.inner.get_code()
    }

    #[napi]
    pub fn get_code_upper(&self) -> u32 {
        self.inner.get_code_upper()
    }

    #[napi]
    pub fn get_code_lower(&self) -> u32 {
        self.inner.get_code_lower()
    }

    #[napi]
    pub fn get_copraw(&self) -> u32 {
        self.inner.get_copraw()
    }

    #[napi]
    pub fn get_cop0d(&self) -> u32 {
        self.inner.get_cop0d()
    }

    #[napi]
    pub fn get_fs(&self) -> u32 {
        self.inner.get_fs()
    }

    #[napi]
    pub fn get_ft(&self) -> u32 {
        self.inner.get_ft()
    }

    #[napi]
    pub fn get_fd(&self) -> u32 {
        self.inner.get_fd()
    }

    #[napi]
    pub fn get_cop1cs(&self) -> u32 {
        self.inner.get_cop1cs()
    }

    #[napi]
    pub fn get_cop2t(&self) -> u32 {
        self.inner.get_cop2t()
    }

    // ==================== Raw value accessors ====================

    #[napi]
    pub fn raw(&self) -> u32 {
        self.inner.raw()
    }

    #[napi]
    pub fn processed_immediate(&self) -> i32 {
        self.inner.processed_immediate()
    }

    #[napi]
    pub fn instr_index_as_vram(&self) -> u32 {
        self.inner.instr_index_as_vram()
    }

    #[napi]
    pub fn branch_offset(&self) -> i32 {
        self.inner.branch_offset()
    }

    #[napi]
    pub fn branch_offset_generic(&self) -> i32 {
        self.inner.branch_offset_generic()
    }

    #[napi]
    pub fn branch_vram_generic(&self) -> u32 {
        self.inner.branch_vram_generic()
    }

    #[napi]
    pub fn destination_gpr(&self) -> Option<u32> {
        self.inner.destination_gpr()
    }

    #[napi]
    pub fn outputs_to_gpr_zero(&self) -> bool {
        self.inner.outputs_to_gpr_zero()
    }

    #[napi]
    pub fn opcode_name(&self) -> String {
        self.inner.opcode_name().to_string()
    }

    #[napi]
    pub fn instr_id_type_name(&self) -> String {
        self.inner.instr_id_type_name().to_string()
    }

    // ==================== Control flow analysis ====================

    #[napi]
    pub fn is_branch(&self) -> bool {
        self.inner.is_branch()
    }

    #[napi]
    pub fn is_branch_likely(&self) -> bool {
        self.inner.is_branch_likely()
    }

    #[napi]
    pub fn is_unconditional_branch(&self) -> bool {
        self.inner.is_unconditional_branch()
    }

    #[napi]
    pub fn is_jump(&self) -> bool {
        self.inner.is_jump()
    }

    #[napi]
    pub fn is_jump_with_address(&self) -> bool {
        self.inner.is_jump_with_address()
    }

    #[napi]
    pub fn is_function_call(&self) -> bool {
        self.inner.is_function_call()
    }

    #[napi]
    pub fn is_return(&self) -> bool {
        self.inner.is_return()
    }

    #[napi]
    pub fn is_jumptable_jump(&self) -> bool {
        self.inner.is_jumptable_jump()
    }

    #[napi]
    pub fn has_delay_slot(&self) -> bool {
        self.inner.has_delay_slot()
    }

    // ==================== Memory and load/store ====================

    #[napi]
    pub fn does_load(&self) -> bool {
        self.inner.does_load()
    }

    #[napi]
    pub fn does_store(&self) -> bool {
        self.inner.does_store()
    }

    #[napi]
    pub fn does_dereference(&self) -> bool {
        self.inner.does_dereference()
    }

    #[napi]
    pub fn does_link(&self) -> bool {
        self.inner.does_link()
    }

    #[napi]
    pub fn access_type(&self) -> u32 {
        match self.inner.access_type() {
            rabbitizer::AccessType::INVALID => 0,
            rabbitizer::AccessType::BYTE => 1,
            rabbitizer::AccessType::SHORT => 2,
            rabbitizer::AccessType::WORD => 3,
            rabbitizer::AccessType::DOUBLEWORD => 4,
            rabbitizer::AccessType::QUADWORD => 5,
            rabbitizer::AccessType::FLOAT => 6,
            rabbitizer::AccessType::DOUBLEFLOAT => 7,
            rabbitizer::AccessType::WORD_LEFT => 8,
            rabbitizer::AccessType::WORD_RIGHT => 9,
            rabbitizer::AccessType::DOUBLEWORD_LEFT => 10,
            rabbitizer::AccessType::DOUBLEWORD_RIGHT => 11,
            rabbitizer::AccessType::MAX => 0,
        }
    }

    #[napi]
    pub fn does_unsigned_memory_access(&self) -> bool {
        self.inner.does_unsigned_memory_access()
    }

    // ==================== Register modification analysis ====================

    #[napi]
    pub fn modifies_rt(&self) -> bool {
        self.inner.modifies_rt()
    }

    #[napi]
    pub fn modifies_rd(&self) -> bool {
        self.inner.modifies_rd()
    }

    #[napi]
    pub fn modifies_rs(&self) -> bool {
        self.inner.modifies_rs()
    }

    #[napi]
    pub fn reads_rs(&self) -> bool {
        self.inner.reads_rs()
    }

    #[napi]
    pub fn reads_rt(&self) -> bool {
        self.inner.reads_rt()
    }

    #[napi]
    pub fn reads_rd(&self) -> bool {
        self.inner.reads_rd()
    }

    #[napi]
    pub fn reads_hi(&self) -> bool {
        self.inner.reads_hi()
    }

    #[napi]
    pub fn reads_lo(&self) -> bool {
        self.inner.reads_lo()
    }

    #[napi]
    pub fn modifies_hi(&self) -> bool {
        self.inner.modifies_hi()
    }

    #[napi]
    pub fn modifies_lo(&self) -> bool {
        self.inner.modifies_lo()
    }

    // ==================== Floating point register analysis ====================

    #[napi]
    pub fn modifies_fs(&self) -> bool {
        self.inner.modifies_fs()
    }

    #[napi]
    pub fn modifies_ft(&self) -> bool {
        self.inner.modifies_ft()
    }

    #[napi]
    pub fn modifies_fd(&self) -> bool {
        self.inner.modifies_fd()
    }

    #[napi]
    pub fn reads_fs(&self) -> bool {
        self.inner.reads_fs()
    }

    #[napi]
    pub fn reads_ft(&self) -> bool {
        self.inner.reads_ft()
    }

    #[napi]
    pub fn reads_fd(&self) -> bool {
        self.inner.reads_fd()
    }

    // ==================== Instruction classification ====================

    #[napi]
    pub fn is_nop(&self) -> bool {
        self.inner.is_nop()
    }

    #[napi]
    pub fn maybe_is_move(&self) -> bool {
        self.inner.maybe_is_move()
    }

    #[napi]
    pub fn is_pseudo(&self) -> bool {
        self.inner.is_pseudo()
    }

    #[napi]
    pub fn is_trap(&self) -> bool {
        self.inner.is_trap()
    }

    #[napi]
    pub fn is_float(&self) -> bool {
        self.inner.is_float()
    }

    #[napi]
    pub fn is_double(&self) -> bool {
        self.inner.is_double()
    }

    #[napi]
    pub fn is_unsigned(&self) -> bool {
        self.inner.is_unsigned()
    }

    #[napi]
    pub fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }

    #[napi]
    pub fn is_likely_handwritten(&self) -> bool {
        self.inner.is_likely_handwritten()
    }

    #[napi]
    pub fn not_emitted_by_compilers(&self) -> bool {
        self.inner.not_emitted_by_compilers()
    }

    #[napi]
    pub fn can_be_hi(&self) -> bool {
        self.inner.can_be_hi()
    }

    #[napi]
    pub fn can_be_lo(&self) -> bool {
        self.inner.can_be_lo()
    }

    // ==================== Instruction comparison ====================

    #[napi]
    pub fn same_opcode(&self, other: &Instruction) -> bool {
        self.inner.same_opcode(&other.inner)
    }

    #[napi]
    pub fn same_opcode_but_different_arguments(&self, other: &Instruction) -> bool {
        self.inner.same_opcode_but_different_arguments(&other.inner)
    }

    // ==================== Operand analysis ====================

    #[napi]
    pub fn has_operand(&self, operand: u32) -> bool {
        // operand is u32 index into OperandType enum
        // For now, we'll pass through - users need to use raw values
        unsafe {
            let op: rabbitizer::OperandType = std::mem::transmute(operand);
            self.inner.has_operand(op)
        }
    }

    #[napi]
    pub fn has_operand_alias(&self, operand: u32) -> bool {
        unsafe {
            let op: rabbitizer::OperandType = std::mem::transmute(operand);
            self.inner.has_operand_alias(op)
        }
    }

    // ==================== Suffix and descriptor ====================

    #[napi]
    pub fn instr_suffix(&self) -> u32 {
        match self.inner.instr_suffix() {
            rabbitizer::InstrSuffix::ALL_NONE => 0,
            rabbitizer::InstrSuffix::R5900_xyzw => 1,
            rabbitizer::InstrSuffix::ALL_MAX => 0,
        }
    }
}
