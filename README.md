# Rust Memory Safety

This crate aims to present Rust memory safety features by a set of simple to understand examples.

The essence of Rust in 30 minutes!

---

## Principle

Rust guarantees memory safety by adopting an ownership model.

- values can be either mutable or immutable (assigned once)
- each value has a _single_ owner
- values can be either _moved_ or _copied_ to a new owner

- values can be _borrowed_ and accessed through either
  - arbitrary number of immutable references (`&T`), or
  - a single mutable reference (`&mut T`)

---

## Defined behavior

Rust ensures (well) _defined behavior_ at all times.

- evaluation (execution) either
  - succeeds (values and references always _valid_)
  - fails with a `panic` (which is still _safe_ as it stops _before_ anything _bad_ happens, thus prevents programs to run with _undefined behavior_)

---

## Realization

The Rust compiler will either:

- prove _defined behavior_ at compile time, or
- generate code that at run-time ensures _defined behavior_

---

## Limitations

### unsafe code

Rust allows `unsafe` code, where the user explicitly opt-out of guarantees:

- access hardware using _raw pointers_
- share mutable values between execution contexts
- call other `unsafe` code, e.g., legacy C/C++

### stack overflow

Rust may run into _undefined behavior_ in case of _stack overflow_.

---

## RTIC Lab will fix it

- The RTIC framework ensures safe sharing of mutable resources
- All hardware access boils down to machine generated code based on vendors specifications
- Additional tooling to prove _defined behavior_
  - SymEx, static analysis by symbolic execution
  - Hippomenes RISC-V Real-Time Security, run-time verification
- Our ultimately goal: methods and tools to design (embedded) systems being
  - reliable, robust, safe and secure _by construction_
