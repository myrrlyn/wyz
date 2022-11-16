<div style="text-align: center;" align="center">

# `wyz`

## myrrlyn’s wyzyrdly library <!-- omit in toc -->

[![Latest Version][version_img]][crate_link]
[![MSRV][msrv_img]][crate_link]
[![License][license_img]][license_file]

[![Documentation][docs_img]][docs_link]
[![Crate Downloads][downloads_img]][crate_link]

</div>

I have developed a collection of utility and convenience Rust modules that are
useful to me, and may be useful to you also.

This crate is a collection of largely-independent small modules. I do not
currently offer features to disable modules independently of each other, but
their compilation cost is small enough to essentially not matter.

## Modules <!-- omit in toc -->

1. [`bidi`](#bidi)
1. [`exit`](#exit)
1. [`fmt`](#fmt)
1. [`range`](#range)

## `bidi`

This provides an extension trait for `DoubleEndedIterator` with a method,
`.bidi(cond: bool)`, that sets whether the iterator operates in forward or
reverse by the runtime condition. When the condition is `true`, forward
iteration (with `.next()`, `.nth()`) forwards to the equivalent reverse
methods (`.next_back()`, `.nth_back()`) and vice-versa; when the condition is
`false`, iteration behaves normally.

This only checks the condition upon initial creation; it is otherwise
branchless.

## `exit`

This is a macro that calls `std::process::exit`. It can return a status code,
and also print a message to `stderr`.

```rust
use wyz::exit::exit;

exit!();
exit!(2);
exit!(3, "This is a {} message", "failure");
```

The default call is `std::process::exit(1)`; a call may provide its own exit
code and, in addition, a set of arguments to pass directly to `eprintln!`. The
error message is not guaranteed to be emitted, as `stderr` may be closed at time
of `exit!`.

## `fmt`

Rust uses the `Debug` trait for automatic printing events in several parts of
the standard library. This module provides wrapper types which forward their
`Debug` implementation to a specified other formatting trait. It also implements
extension methods on all types that have format trait implementations to wrap
them in the corresponding shim type.

```rust
use wyz::fmt::FmtForward as _;

let val = 6;
let addr = &val as *const i32;
println!("{:?}", addr.fmt_pointer());
```

This snippet uses the `Debug` format template, but will print the `Pointer`
implementation of `*const i32`.

This is useful for fitting your values into an error-handling framework that
only uses `Debug`, such as the `fn main() -> Result` program layout.

In addition to forwarding each of the scalar traits, this also provides a
`.fmt_list()` that formats any type `T where &T: IntoIterator` as a list. The
list-formatting adapter itself implements all of the scalar formatting traits,
and can also be wrapped in any of the forwarding guards so that it can be sent
to a `Debug` sink:

```rust
use wyz::fmt::FmtForward as _;

let seq = 0 .. 4;
assert_eq!(
  format!("{:02b}", seq.fmt_list()),
  "[00, 01, 10, 11]",
);
assert_eq!(
  format!(
    "{:?}",
    seq
      .map(|x| (x + 1) * 10)
      .fmt_list()
      .fmt_lower_hex(),
  ),
  "[a, 14, 1e, 28]",
);
```

## `range`

This provides an extension trait, `RangeExt`, on `RangeBounds`. It is currently
only used with `R: RangeBounds<usize>`, again because it is an MVP for bitvec’s
use rather than a project in its own right. It normalizes arbitrary ranges into
the `Range` concrete type. PRs welcome!

[crate_link]: https://crates.io/crates/wyz "Crate Link"
[docs_link]: https://docs.rs/wyz/latest/wyz "Documentation"
[docs_img]: https://img.shields.io/docsrs/wyz/latest.svg?style=for-the-badge "Documentation Display"
[downloads_img]: https://img.shields.io/crates/dv/wyz.svg?style=for-the-badge "Crate Downloads"
[license_file]: https://github.com/bitvecto-rs/wyz/blob/master/LICENSE.txt "License File"
[license_img]: https://img.shields.io/crates/l/wyz.svg?style=for-the-badge "License Display"
[msrv_img]: https://img.shields.io/badge/MSRV-1.50-f46623?style=for-the-badge&logo=rust "Minimum Supported Rust Version: 1.50"
[version_img]: https://img.shields.io/crates/v/wyz?color=f46623&style=for-the-badge "wyz version badge"
