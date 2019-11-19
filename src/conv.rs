/*! Directed Type Conversion

This module provides sibling traits to the `std::convert` module. The standard
library puts the type parameter in the trait declaration, which makes those
traits generic and suitable for constraint clauses and function calls, but not
usable in indeterminate method-call positions. These traits put the type
parameter in the function declaration, making the trait non-generic and allowing
the function to be called in method-call position without ambiguity.
!*/

/** Directed Type Conversion

This trait is an accessory to [`From`] and [`Into`]. It works by moving the
destination type from the trait name (`Into<Target>::into`) into the method name
(`Conv::conv::<Target>`). This change makes `Into<_>` the correct trait to use
in trait bounds and `.conv::<_>` the correct method to use in expressions.

A `conv::<T>` method is automatically available whenever an `Into<T>`
implementation exists for a type. `Into<T>` is most commonly implemented by
taking advantage of the reflexive blanket implentation using `From`, but can
also be manually implemented as desired.

`.into()` cannot be used in intermediate expressions, because it is impossible
for the compiler’s type engine to select a unique `Into<T>` implementation. This
means that expressions like `v.into().use()` will never compile. Users can
replace `.into()` with `.conv::<Dest>()` in order to inform the compiler of the
type of the expression after the conversion, and make compilation succeed.

`Conv` cannot be used in trait bounds, because the trait itself is not generic.
All `Sized` types implement `Conv` by default, so specifying that a type must be
`Conv` adds no information to the solver.

# Examples

## Conversion as methods

Conversion with `.into()` will fail to compile, even with the type annotation:

```rust,ignore
let s: String = "static".into().clone();
//              ^^^^^^^^^^^^^^^ cannot infer type for `T`
// note: type must be known at this point
```

while the equivalent code with `.conv::<_>` does compile:

```rust
# use wyz::conv::Conv;
let s = "static".conv::<String>().clone();
```

## Conversion as traits

Bounding a type with `Conv` will not compile, because the trait itself gives no
information:

```rust,ignore
# use wyz::conv::Conv;
fn lift<T: Conv>(src: T) -> String {
  src.conv::<String>().clone()
//    ^^^^ the trait `From<T>` is not implemented for `String`
// help: consider adding a `where String: From<T>` bound
// note: required because of the requirements on the impl of `Into<String>` for `T`
}
```

This can be fixed by adding the clause `where String: From<T>`, or by using the
bound `Into`:

```rust
# use wyz::conv::Conv;
fn lift<T: Into<String>>(src: T) -> String {
  src.conv::<String>().clone()
}
```

The `Into<T>` trait bound makes available both the `Into::<T>::into` method and
the `Conv::conv::<T>` method.

[`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
[`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
**/
pub trait Conv: Sized {
	/// Converts `self` into a target type.
	///
	/// This method runs `<Self as Into<T>>::into` on `self` to produce the
	/// desired output. The only difference between using `Conv::conv` and
	/// `Into::into` is where the target type is placed in the name; `.conv()`
	/// can be used in intermediate positions of an expression, while `.into()`
	/// cannot.
	///
	/// # Examples
	///
	/// ```rust
	/// use wyz::conv::Conv;
	///
	/// let t = "hello".conv::<String>();
	/// ```
	fn conv<T: Sized>(self) -> T where Self: Into<T> {
		<Self as Into<T>>::into(self)
	}
}

impl<T: Sized> Conv for T {}
