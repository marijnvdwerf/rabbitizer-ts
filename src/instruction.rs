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

    // Bit field getters
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

    // Examination methods - these check what kind of instruction this is

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
}
