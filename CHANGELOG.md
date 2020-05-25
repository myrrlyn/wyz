# Changelog <!-- omit in toc -->

All notable changes will be documented in this file.

This document is written according to the [Keep a Changelog][kac] style.

## 0

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
