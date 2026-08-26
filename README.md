# Nyx

A small, readable language in the spirit of Python 3.6 — implemented from scratch in Rust.

> **Status: pre-alpha.** Nothing runs yet. This repository is at stage 0 of the roadmap below.

## What this is

A learning project. The goal is not a language anyone should adopt; it is to implement one end to
end — lexer, parser, tree-walking evaluator, and eventually a bytecode VM.

The language is a subset of Python 3.6: the era before type annotations became idiomatic.
Indentation-delimited blocks, dynamic typing, no `async`.

```python
def fizzbuzz(n):
    out = []
    for i in range(1, n + 1):
        if i % 15 == 0:
            out.append("fizzbuzz")
        elif i % 3 == 0:
            out.append("fizz")
        elif i % 5 == 0:
            out.append("buzz")
        else:
            out.append(str(i))
    return out

print("\n".join(fizzbuzz(20)))
```

## Non-goals

CPython compatibility. Features are cut when their cost buys little understanding:

- **Type annotations** — the point is the language as it read before them.
- **Metaclasses, descriptors, multiple inheritance** — single inheritance only.
- **Generators and `yield`** — cheap in CPython, disproportionately expensive in a tree-walker.
- **`async` / `await`, threads, the C API, the import system.**
- **Tabs for indentation** — a hard error.

Dunder dispatch is limited to `__init__`, `__str__`, `__eq__`, `__len__`, `__getitem__`.

## Roadmap

Each stage is a working interpreter, not a layer.

- [ ] **0 — Vertical slice.** Integers, Pratt parser, tree-walking evaluation, a REPL.
- [ ] **1 — Indentation.** Lexer-synthesized `NEWLINE` / `INDENT` / `DEDENT`; implicit line joining.
- [ ] **2 — Control flow.** `if` / `elif` / `else`, `while`, `for`, `break`, `continue`, `return`.
- [ ] **3 — Functions.** Closures, LEGB scoping, `global` and `nonlocal`.
- [ ] **4 — Data types.** `list`, `dict`, `tuple`, `str`, truthiness, slicing, comprehensions.
- [ ] **5 — Classes.** Single inheritance, bound methods, the fixed dunder set.
- [ ] **6 — Exceptions.** `try` / `except` / `finally` / `raise`, tracebacks.
- [ ] **7 — Bytecode VM.** A second backend over the same AST, kept output-identical to the
      tree-walker, which becomes its reference oracle.

## Building

```sh
cargo run              # REPL
cargo run -- main.nyx  # run a file
cargo test
```

## Prior art

- [Crafting Interpreters](https://craftinginterpreters.com/) — the tree-walker-then-VM arc this
  roadmap follows.
- [Writing an Interpreter in Go](https://interpreterbook.com/) — Monkey; Pratt parsing.
- [RustPython](https://github.com/RustPython/RustPython) — a full CPython in Rust.
- [starlark-rust](https://github.com/facebook/starlark-rust) — a reduced Python dialect in Rust.
  Cut for determinism rather than readability, but a useful record of what can be left out.

## License

MIT