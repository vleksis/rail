# RailPL

```
  ____       _ _ ____  _
 |  _ \ __ _(_) |  _ \| |
 | |_) / _` | | | |_) | |
 |  _ < (_| | | |  __/| |___
 |_| \_\__,_|_|_|_|   |_____|
```

**RailPL** is a small programming language experiment in Rust.

---

## What you get

- A complete compiler pipeline ending in a bytecode program representation
- A VM that executes that representation

---

## Build

```bash
cargo build
cargo test
```

## Run (CLI is evolving)

```bash
cargo run --release
```

---

## Language snapshot

Syntax is still in flux, but the project currently aims for a familiar imperative feel:

```rail
{
  12345 * 54321;
  0x101u64 + 0x11u64;
  2 + 2 == 4;
}
```

---

## VM tracing

There is an optional `trace_vm` Cargo feature for VM execution tracing.

---

## Features / TODO

Legend: ✅ implemented · 🧪 in progress / partial · ⬜ planned

### Compiler pipeline & diagnostics

| Feature                                                 | Status |
| ------------------------------------------------------- | :----: |
| Compile source → bytecode `Program`                     |   ✅   |
| Validation-only mode (`check`)                          |   ⬜   |
| Rich diagnostics with spans (lexer/parser/typechecker)  |   🧪   |
| Pretty error rendering (caret highlights, notes, helps) |   ⬜   |
| Source → AST dump (`ast` subcommand)                    |   ⬜   |
| Bytecode verifier (stack height, jumps, const indices)  |   ⬜   |
| Constant folding                                        |   ⬜   |
| Peephole optimizer                                      |   ⬜   |

### VM, runtime & serialization

| Feature                            | Status |
| ---------------------------------- | :----: |
| Execute bytecode `Program` in a VM |   ✅   |
| Deterministic single-step mode     |   🧪   |
| VM execution trace                 |   🧪   |
| Disassembler                       |   🧪   |
| Dump / serialize `Program`         |   ✅   |
| JSON dump mode                     |   ⬜   |
| GC / refcounting                   |   ⬜   |

### Language surface

| Feature                                                    | Status |
| ---------------------------------------------------------- | :----: |
| Expression precedence + operators                          |   🧪   |
| Numeric literals: decimal / `0b` / `0o` / `0x` + postfixes |   🧪   |
| Boolean literals and boolean ops                           |   🧪   |
| Blocks + statement sequencing                              |   🧪   |
| Variables (`let`)                                          |   ⬜   |
| Assignment                                                 |   ⬜   |
| Lexical scoping                                            |   ⬜   |
| `if` / `else`                                              |   ⬜   |
| `while` / `for`                                            |   ⬜   |
| Functions (declaration, call, return)                      |   ⬜   |
| Recursion                                                  |   ⬜   |
| Closures (non-capturing → capturing)                       |   ⬜   |
| Strings                                                    |   ⬜   |
| Arrays                                                     |   ⬜   |
| Structs                                                    |   ⬜   |
| Enums / tagged unions                                      |   ⬜   |
| Pattern matching                                           |   ⬜   |

### Types

| Feature                                           | Status |
| ------------------------------------------------- | :----: |
| Primitive types + type checking                   |   🧪   |
| Type inference (locals / returns)                 |   ⬜   |
| Exhaustive type errors (“expected/found” + spans) |   ⬜   |

### Tooling, testing & project hygiene

| Feature                                                      | Status |
| ------------------------------------------------------------ | :----: |
| Stable CLI (`run`, `dump`, `disasm`, `check`)                |   ⬜   |
| REPL                                                         |   ⬜   |
| Formatter / pretty-printer                                   |   ⬜   |
| Golden tests (source → tokens/AST/bytecode/output snapshots) |   ⬜   |
| Parser fuzzing / crash minimization                          |   ⬜   |
| CI: fmt + clippy + feature matrix                            |   ⬜   |

---

## License

Licensed under both of Apache 2.0 license or MIT license.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
