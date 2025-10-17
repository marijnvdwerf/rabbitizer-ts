#!/usr/bin/env node
// SPDX-FileCopyrightText: © 2022-2024 Decompollaborate
// SPDX-License-Identifier: MIT

/**
 * Basic example of using rabbitizer-ts
 */

import { Instruction, Config, Utils, getVersion } from '../index.js'

console.log('Rabbitizer v' + getVersion())
console.log('')

// Example 1: Basic instruction decoding
console.log('=== Example 1: Basic Disassembly ===')
const instr = new Instruction(0x24010001)
console.log('Instruction: 0x' + (0x24010001).toString(16).toUpperCase())
console.log('Disassembly:', instr.disassemble())
console.log('Opcode:', instr.getOpcode())
console.log('RS:', instr.getRs())
console.log('RT:', instr.getRt())
console.log('')

// Example 2: Instruction analysis
console.log('=== Example 2: Instruction Analysis ===')
const jal = new Instruction(0x0c000000)
console.log('Instruction: jal 0x00000000')
console.log('Is jump?', jal.isJump())
console.log('Is call?', jal.isFunctionCall())
console.log('Is branch?', jal.isBranch())
console.log('')

// Example 3: Load/Store detection
console.log('=== Example 3: Load/Store Instructions ===')
const load = new Instruction(0x8c420000)
const store = new Instruction(0xac420000)
console.log('lw: is_load =', load.doesLoad(), ', is_store =', load.doesStore())
console.log('sw: is_load =', store.doesLoad(), ', is_store =', store.doesStore())
console.log('')

// Example 4: Register utilities
console.log('=== Example 4: Register Names ===')
console.log('Register 4 (O32):', Utils.get_register_name_o32(4))
console.log('Register 4 (numeric):', Utils.get_register_name_numeric(4))
console.log('')

// Example 5: Batch disassembly
console.log('=== Example 5: Batch Disassembly ===')
const instructions = [0x27bdffe0, 0xafbf001c, 0x0c000000, 0x00000000, 0x8fbf001c, 0x27bd0020, 0x03e00008]

let address = 0x80000000
for (const word of instructions) {
  const instr = new Instruction(word, address)
  console.log('0x' + address.toString(16).toUpperCase().padStart(8, '0') + '  ' + instr.disassemble())
  address += 4
}

console.log('')
console.log('Done!')
