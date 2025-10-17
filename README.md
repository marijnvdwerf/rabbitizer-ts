# rabbitizer-ts

MIPS instruction decoder for Node.js and Bun with TypeScript support.

[![Build Status](https://github.com/marijnvdwerf/rabbitizer-ts/actions/workflows/CI.yml/badge.svg)](https://github.com/marijnvdwerf/rabbitizer-ts/actions/workflows/CI.yml)
[![npm version](https://img.shields.io/npm/v/@marijnvdwerf/rabbitizer.svg)](https://www.npmjs.com/package/@marijnvdwerf/rabbitizer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Fast MIPS instruction decoding in JavaScript/TypeScript
- Full TypeScript type definitions with auto-generation
- Support for multiple MIPS architectures:
  - Standard MIPS CPU (I, II, III)
  - N64 RSP (Reality Signal Processor)
  - PlayStation R3000 GTE
  - PlayStation Portable R4000 ALLEGREX
  - PlayStation 2 R5900 (Emotion Engine)
- Zero-copy instruction analysis
- Cross-platform binaries (Linux, macOS, Windows, Android, WASI)
- Works with both Node.js and Bun

## Installation

```bash
npm install @marijnvdwerf/rabbitizer
```

Or with Bun:

```bash
bun add @marijnvdwerf/rabbitizer
```

## Quick Start

```javascript
import { Instruction, Utils } from '@marijnvdwerf/rabbitizer'

// Create an instruction from a 32-bit word
const instr = new Instruction(0x24010001) // addiu $at, $zero, 1

// Disassemble to assembly
console.log(instr.disassemble()) // "addiu $at, $zero, 0x1"

// Check instruction properties
console.log(instr.get_opcode()) // 9
console.log(instr.is_jump())    // false
console.log(instr.does_load())  // false

// Use utility functions
console.log(Utils.get_register_name_o32(4)) // "$a0"
console.log(Utils.sign_extend_immediate(0xffff)) // -1
```

## API Documentation

### `Instruction`

Represents a decoded MIPS instruction.

**Constructor:** `new Instruction(word: number, vram?: number, category?: string)`

**Properties:**
- `word: number` - The instruction word
- `vram: number` - Virtual address
- `category: string` - Instruction category

**Methods:**
- `disassemble(immediateOverride?: string, vram?: number): string`
- `get_opcode(), get_rs(), get_rt(), get_rd(), get_sa(), get_function(), get_immediate(), get_instr_index()` - Bit field getters
- `is_branch(), is_jump(), is_function_call(), is_return(), does_load(), does_store(), is_nop(), is_pseudo(), is_trap()` - Instruction type checks
- `modifies_rt(), modifies_rd(), modifies_rs(), reads_rs(), reads_rt(), reads_rd()` - Register analysis

### `Utils`

```typescript
Utils.sign_extend_immediate(imm: number): number
Utils.swap_endianness(word: number): number
Utils.is_power_of_two(val: number): boolean
Utils.get_register_name_o32(index: number): string
Utils.get_register_name_numeric(index: number): string
```

### `Config`

```typescript
Config.info(): string  // Returns configuration info message
```

## Development

### Prerequisites

- Node.js 12.0.0+
- Rust 1.66.1+
- C compiler

### Build from Source

```bash
npm install
npm run build
npm test
```

## License

MIT

See [LICENSE](./LICENSE) for details.

## References

- [rabbitizer](https://github.com/Decompollaborate/rabbitizer) - Original C library
- [rabbitizer-rs](https://crates.io/crates/rabbitizer) - Rust bindings
