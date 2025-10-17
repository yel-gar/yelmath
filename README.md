# 🧮 yelmath

> A lightweight, **generic** linear algebra library for Rust — vectors and matrices, all in one tidy crate. 🚀

`yelmath` is a small, dependency-light Rust crate that provides **generic** 2, 3 and 4-dimensional
vectors and square matrices. Because every value type implements the [`Scalar`] trait, you can
work with **`f32`**, **`f64`**, or even **integer** types (`i8`...`i128`) from the exact same API.
That makes it a handy foundation for math-heavy hobby projects, prototyping, education, and
small rendering / physics / game logic.

---

## 📑 Contents

- [Features](#-features)
- [Solutions / Design](#%EF%B8%8F-solutions--design)
- [Project Structure](#-project-structure)
- [Usage](#-usage)
- [Running the tests](#%EF%B8%8F-running-the-tests)
- [License](#-license)

---

## ✨ Features

- 🧱 **Generic** over any scalar type — no `f32`-only tunnel vision.
- 📐 **Vectors** in `2D`, `3D`, and `4D` (`Vector2D`, `Vector3D`, `Vector4D`).
- 🟦 **Square matrices** `3×3` and `4×4` behind a single `Matrix` trait.
- 🔢 Core ops: **add**, **sub**, **dot**, **cross**, **scale**, **magnitude**, **invert**, **normalize**.
- 🧭 **Matrix** ops: **determinant**, **minor**, **transpose**, **inverse**, **matrix × vector**.
- 🎯 **Precision comparison** (`precision_eq`) — compare floats within an epsilon instead of exact.
- 🧪 Fully covered by an in-crate test suite. ✅
- ⚡ Only one real dependency: [`num-traits`](https://crates.io/crates/num-traits).

---

## 🛠️ Solutions / Design

The crate is split into small, focused modules:

| Module        | Responsibility                                                        |
| ------------- | --------------------------------------------------------------------- |
| `types.rs`    | The [`Scalar`] trait + impls for `f32/f64/i8…i128`.                   |
| `vectors.rs`  | `Vector2D`, `Vector3D`, `Vector4D` and the shared `Vector<T>` trait.  |
| `matrices.rs` | `Matrix3x3`, `Matrix4x4` and the shared `Matrix<T>` trait.            |
| `util.rs`     | The generic `normalize_float_arr` helper used by vector normalization.|
| `errors.rs`   | `VectorErr` (length mismatch) with `Debug`/`Display`/`Error`.         |
| `lib.rs`      | Module wiring + the `#[cfg(test)]` test module.                       |

### Key ideas

- **Trait-generic vectors** — `Vector2D<T>` etc. are generic over `T: Scalar`, so the same
  implementation serves every numeric type.
- **Float normalization** — `normalize_float_arr` computes the Euclidean norm with a generic
  `sum` + `sqrt`, dividing each element in place; zero-length vectors safely collapse to zero.
- **Matrix inverse via cofactors** — determinant and cofactor-expansion minors are used together
  to build the adjugate, then divided by the determinant (`±minor / det`).
- **Precision equality** — since floats can't be compared exactly, every "equality" API accepts a
  tolerance and checks that each component's absolute difference stays within it.

---

## 📦 Project Structure

```
yelmath/
├── Cargo.toml          # package manifest (edition 2024, single dep: num-traits)
├── Cargo.lock          # resolved dependency versions
├── .gitignore          # ignores /target and .idea
├── README.md           # this file
└── src/
    ├── lib.rs          # module declarations + tests
    ├── types.rs        # Scalar trait + numeric impls
    ├── vectors.rs      # Vector2D / Vector3D / Vector4D
    ├── matrices.rs     # Matrix3x3 / Matrix4x4
    ├── util.rs         # generic normalize helper
    └── errors.rs       # VectorErr type
```

---

## 🚀 Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
yelmath = { path = "./yelmath" }   # for a local checkout
```

You can find more usage examples in the tests in `lib.rs`

---

## ▶️ Running the tests

The crate ships with an extensive in-module test suite covering
construction, arithmetic, normalization, precision equality, determinants, minors,
transposition, inversion, and matrix–vector multiplication.

```bash
cargo test
```
