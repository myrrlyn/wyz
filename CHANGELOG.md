# Changelog <!-- omit in toc -->

All notable changes will be documented in this file.

This document is written according to the [Keep a Changelog][kac] style.

## 0

### 0.6.0

Strip the `comu` module (migrated to `funty 3.0`) and the `wm` module (an idle
sketch I will never seriously use).

### 0.5.0

Added the `Bidi` iterator adapter, which applies a `.rev()` on construction if a
given condition is true.

Added the `RangeExt` trait for making some operations on
`<R: RangeBounds<usize>>` easier.

Added a `FmtList` type (and `.fmt_list()` method) which allows anything that can
be borrowed as an iterator to render itself conveniently.

Added more pointer methods to `Address`, and created a system for working with
references as well as pointers.

### 0.4.0

Add the `comu` module containing the type-system mutability tracking extracted
from `bitvec`.

### 0.3.0

Added a background garbage disposal system in the `wm` module, under the
`garbage` feature. It is accessed by importing the `wm::BgDropExt` trait and
using its `.bg_drop()` method on a value.

The disposal system manages a single worker thread which receives any type and
runs the appropriate destructor for it. Once initialized, the system remains in
operation until explicitly shut down by the client program; once shut down, all
future deferred-drop objects are destroyed in their local thread as normal.

This system allows programs to opt in to faster immediate behavior when a value
goes out of scope, with minimal system and per-value cost.

Removed `tap`, `pipe`, and `conv`. They have been promoted to the [`tap`] crate.

### 0.2.0

Added `conv::TryConv` for fallible directed conversion.

Added `fmt` module, which supplies behavior to forward any formatting trait to
`Debug`.

Removed `pretty` module in favor of `fmt`.

### 0.1.1

Fix typos.

### 0.1.0

Initial release, featuring:

- `conv`
- `exit`
- `pipe`
- `pretty`
- `tap`

[`tap`]: https://crates.io/crates/tap
