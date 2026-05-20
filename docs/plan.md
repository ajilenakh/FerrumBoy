# Game Boy Emulator TODO

## Project Structure

```txt
gb-rs/
├── Cargo.toml
├── README.md
├── TODO.md
├── roms/
├── test-roms/
├── docs/
├── crates/
│   ├── gb-core/
│   ├── gb-cpu/
│   ├── gb-memory/
│   ├── gb-cartridge/
│   ├── gb-ppu/
│   ├── gb-apu/
│   ├── gb-timer/
│   ├── gb-input/
│   ├── gb-terminal/
│   └── gb-debugger/
└── target/
```

---

# Build Order

## Phase 0 — Workspace Setup

- [X] Create Rust workspace
- [X] Setup Cargo workspace members
- [x] Setup shared linting and formatting
- [X] Setup logging/tracing
- [X] Setup ROM directory
- [X] Setup test ROM directory
- [ ] Setup CI workflow
- [ ] Create base README

---

# Phase 1 — Cartridge System

## crate: gb-cartridge

### Goals

- Load ROM files
- Parse cartridge headers
- Detect cartridge type
- Support ROM ONLY games

### Tasks

- [ ] Implement ROM loader
- [ ] Parse Nintendo logo
- [ ] Parse game title
- [ ] Parse cartridge metadata
- [ ] Implement ROM ONLY mapper
- [ ] Add cartridge tests

---

# Phase 2 — Memory Bus

## crate: gb-memory

### Goals

- Create memory map
- Route reads/writes correctly

### Tasks

- [ ] Create Bus struct
- [ ] Implement read8()
- [ ] Implement write8()
- [ ] Add WRAM support
- [ ] Add HRAM support
- [ ] Add IO register stubs
- [ ] Connect cartridge reads
- [ ] Add memory tests

### Memory Map

```txt
0000-7FFF ROM
8000-9FFF VRAM
A000-BFFF External RAM
C000-DFFF WRAM
E000-FDFF Echo RAM
FE00-FE9F OAM
FF00-FF7F IO Registers
FF80-FFFE HRAM
FFFF Interrupt Enable
```

---

# Phase 3 — CPU Core

## crate: gb-cpu

### Goals

- Execute Game Boy instructions
- Handle registers and flags

### Tasks

- [ ] Create CPU registers
- [ ] Implement flag handling
- [ ] Implement fetch/decode/execute loop
- [ ] Add opcode table
- [ ] Implement load/store instructions
- [ ] Implement arithmetic instructions
- [ ] Implement jump/call instructions
- [ ] Implement stack operations
- [ ] Implement CB-prefixed instructions
- [ ] Implement interrupts
- [ ] Return cycle counts
- [ ] Add CPU tests

### Registers

```txt
AF
BC
DE
HL
SP
PC
```

---

# Phase 4 — Emulator Core

## crate: gb-core

### Goals

- Connect all hardware components

### Tasks

- [ ] Create GameBoy struct
- [ ] Connect CPU + Bus
- [ ] Add clock stepping
- [ ] Add frame stepping
- [ ] Add reset functionality
- [ ] Add boot sequence

### Main Loop

```rust
loop {
    let cycles = cpu.step(&mut bus);

    timer.step(cycles);
    ppu.step(cycles);
    apu.step(cycles);
}
```

---

# Phase 5 — Timer Hardware

## crate: gb-timer

### Goals

- Implement Game Boy timers

### Tasks

- [ ] Implement DIV register
- [ ] Implement TIMA
- [ ] Implement TMA
- [ ] Implement TAC
- [ ] Add timer overflow interrupts
- [ ] Add timer tests

---

# Phase 6 — Interrupt System

## crate: gb-cpu + gb-memory

### Goals

- Handle hardware interrupts

### Tasks

- [ ] Add IF register
- [ ] Add IE register
- [ ] Implement interrupt priorities
- [ ] Implement IME flag
- [ ] Implement interrupt servicing
- [ ] Add interrupt tests

---

# Phase 7 — PPU (Graphics)

## crate: gb-ppu

### Goals

- Render Game Boy graphics

### Tasks

- [ ] Create framebuffer
- [ ] Implement LCD registers
- [ ] Implement scanline timing
- [ ] Implement background rendering
- [ ] Implement tile fetching
- [ ] Implement scrolling
- [ ] Implement sprite rendering
- [ ] Implement window rendering
- [ ] Implement VBlank interrupts
- [ ] Add PPU tests

### Framebuffer

```txt
160 x 144
```

---

# Phase 8 — Terminal Frontend

## crate: gb-terminal

### Goals

- Display framebuffer in terminal

### Tasks

- [ ] Setup crossterm
- [ ] Add terminal renderer
- [ ] Add grayscale rendering
- [ ] Add keyboard input
- [ ] Add FPS counter
- [ ] Add ROM loading CLI
- [ ] Add game loop

### Controls

```txt
Arrow Keys -> D-Pad
Z -> A
X -> B
Enter -> Start
Backspace -> Select
```

---

# Phase 9 — Input System

## crate: gb-input

### Goals

- Handle joypad input

### Tasks

- [ ] Implement JOYP register
- [ ] Add button states
- [ ] Add input interrupts
- [ ] Connect terminal input

---

# Phase 10 — Audio (Optional Early)

## crate: gb-apu

### Goals

- Generate Game Boy audio

### Tasks

- [ ] Stub APU
- [ ] Implement channel 1
- [ ] Implement channel 2
- [ ] Implement wave channel
- [ ] Implement noise channel
- [ ] Mix audio output

---

# Phase 11 — MBC Support

## crate: gb-cartridge

### Goals

- Support bank switching

### Tasks

- [ ] Implement MBC1
- [ ] Implement MBC3
- [ ] Implement RAM banking
- [ ] Implement battery saves
- [ ] Add save file support

---

# Phase 12 — Debugger

## crate: gb-debugger

### Goals

- Add emulator debugging tools

### Tasks

- [ ] Add instruction tracing
- [ ] Add memory viewer
- [ ] Add breakpoint support
- [ ] Add register viewer
- [ ] Add step execution
- [ ] Add disassembler

---

# Phase 13 — Testing

## test-roms/

### CPU Tests

- [ ] Blargg CPU tests
- [ ] Instruction timing tests
- [ ] Interrupt tests

### Graphics Tests

- [ ] PPU timing tests
- [ ] Sprite tests
- [ ] Scanline tests

### Integration Tests

- [ ] Boot ROM tests
- [ ] Tetris boot test
- [ ] Pokemon boot test

---

# Milestones

## Milestone 1

- ROM loads
- CPU executes instructions

## Milestone 2

- Pass CPU test ROMs

## Milestone 3

- Nintendo boot logo appears

## Milestone 4

- First playable game

## Milestone 5

- Stable rendering

## Milestone 6

- Audio support

## Milestone 7

- Save files + debugger

---

# Technical Rules

- [ ] Keep frontend separate from emulator core
- [ ] CPU must NEVER access memory directly
- [ ] Everything runs on cycle timing
- [ ] Avoid circular dependencies
- [ ] Test every opcode
- [ ] Keep framebuffer independent from renderer

---

# Recommended Libraries

## Terminal

- crossterm
- ratatui

## CLI

- clap

## Logging

- tracing

## Bitflags

- bitflags

---

# Future Features

- [ ] SDL frontend
- [ ] WebAssembly frontend
- [ ] Shader rendering
- [ ] Save states
- [ ] Rewind support
- [ ] TAS tools
- [ ] Multiplayer experiments
- [ ] Cheat engine
- [ ] Replay system
