// SPDX-FileCopyrightText: © 2022-2024 Decompollaborate
// SPDX-License-Identifier: MIT

import { test } from 'node:test'
import assert from 'node:assert/strict'
import {
  Instruction,
  InstrCategory,
  Config,
  Abi,
  GprO32,
  Utils,
  getVersion,
  getVersionInfo,
} from '../index.js'

test('Instruction creation', (t) => {
  const instr = new Instruction(0x8d4a7e18, 0x80000000)
  assert.equal(instr.word, 0x8d4a7e18)
  assert.equal(instr.vram, 0x80000000)
  assert.equal(instr.category, 'cpu')
})

test('Instruction disassembly', (t) => {
  const instr = new Instruction(0x24010001)
  const disassembly = instr.disassemble()
  assert(disassembly.includes('addiu'))
})

test('Instruction bit fields', (t) => {
  const instr = new Instruction(0x8c420000)
  assert.equal(instr.getOpcode(), 35)
  assert.equal(instr.getRs(), 2)
  assert.equal(instr.getRt(), 2)
  assert.equal(instr.getImmediate(), 0)
})

test('Load instruction detection', (t) => {
  const lw = new Instruction(0x8c420000)
  assert(lw.doesLoad())
  assert(!lw.doesStore())
})

test('Store instruction detection', (t) => {
  const sw = new Instruction(0xac420000)
  assert(sw.doesStore())
  assert(!sw.doesLoad())
})

test('Jump instruction detection', (t) => {
  const jal = new Instruction(0x0c000000)
  assert(jal.isJump())
  assert(jal.isFunctionCall())
})

test('Branch instruction detection', (t) => {
  const beq = new Instruction(0x10220000)
  assert(beq.isBranch())
  assert(!beq.isJump())
})

test('NOP instruction detection', (t) => {
  const nop = new Instruction(0x00000000)
  assert(nop.isNop())
})

test('Return instruction detection', (t) => {
  const jr = new Instruction(0x03e00008)
  assert(jr.isReturn())
})

test('Register modification detection', (t) => {
  const jalr = new Instruction(0x00a0f809)
  assert(jalr.modifiesRd())
})

test('Register read detection', (t) => {
  const lw = new Instruction(0x8c420000)
  assert(lw.readsRs())
})

test('Different instruction categories', (t) => {
  const cpuInstr = new Instruction(0x24010001, 0, 'cpu')
  assert.equal(cpuInstr.category, 'cpu')

  const rspInstr = new Instruction(0x24010001, 0, 'rsp')
  assert.equal(rspInstr.category, 'rsp')

  const psx = new Instruction(0x24010001, 0, 'r3000gte')
  assert.equal(psx.category, 'r3000gte')
})

test('Configuration info', (t) => {
  const info = Config.info()
  assert(typeof info === 'string')
})

test('Utility function: sign_extend_immediate', (t) => {
  const extended = Utils.sign_extend_immediate(0xffff)
  assert.equal(extended, -1)

  assert.equal(Utils.sign_extend_immediate(0x0001), 1)
})

test('Utility function: register names', (t) => {
  const o32Name = Utils.get_register_name_o32(4)
  assert.equal(o32Name, '$a0')

  const numericName = Utils.get_register_name_numeric(4)
  assert.equal(numericName, '$4')
})

test('Utility function: power of 2', (t) => {
  assert(Utils.is_power_of_two(1))
  assert(Utils.is_power_of_two(2))
  assert(Utils.is_power_of_two(16))
  assert(!Utils.is_power_of_two(3))
  assert(!Utils.is_power_of_two(0))
})

test('Move instruction detection', (t) => {
  const move = new Instruction(0x00000821)
  assert(move.maybeIsMove())
})

test('Pseudo-instruction detection', (t) => {
  const nop = new Instruction(0x00000000)
  assert(typeof nop.isPseudo() === 'boolean')
})

test('Vram and address handling', (t) => {
  const instr = new Instruction(0x24010001, 0x80000004)
  assert.equal(instr.vram, 0x80000004)

  const dis1 = instr.disassemble()
  const dis2 = instr.disassemble(undefined, 0x80000008)
  assert(typeof dis1 === 'string')
  assert(typeof dis2 === 'string')
})

test('Version functions', (t) => {
  const version = getVersion()
  assert.equal(version, '1.14.3')

  const info = getVersionInfo()
  assert.equal(info.version, '1.14.3')
  assert.equal(info.name, 'rabbitizer')
})
