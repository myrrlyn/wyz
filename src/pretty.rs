/*! Prints `Display` instead of `Debug`.

Various APIs in Rust use `Debug` hook to show failure diagnostics. The default
implementation of `Debug` provided by its derive macro prints a source-code
representation of the value, which may not be the most useful text to show to a
user.

This module provides a single type that implements `Debug` by delegating to the
`Display` implementation of the wrapped type. Any type with a `Display`
implementation can wrap in this type, and have its `Display` implementation
called by `Debug` hooks.

All other formatting traits are passed through without redirection.
!*/

use core::fmt::{
	self,
	Binary,
	Debug,
	Display,
	LowerExp,
	LowerHex,
	Octal,
	Pointer,
	UpperExp,
	UpperHex,
	Formatter,
};

/** Redirects `Debug` to `Display`, leaving all other formatters unchanged.

This type wraps any other type, and reïmplements the formatters that the wrapped
type has. If the wrapped type implements `Display`, this wrapper implements both
`Debug` and `Display` as redirects to the wrapped `Display`.

Because Rust does not have negative trait bounds, there is no `Debug`
implementation if the wrapped type has `Debug` but not `Display`.

Bug the language team.
**/
pub struct Pretty<T>(T);

/// Redirect `Debug` to `Display` when wrapping `Display` types.
impl<T: Display> Debug for Pretty<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

/// Forward `Binary` correctly.
impl<T: Binary> Binary for Pretty<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Binary::fmt(&self.0, f)
	}
}

/// Forward `Display` correctly.
impl<T: Display> Display for Pretty<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

/// Forward `LowerExp` correctly.
impl<T: LowerExp> LowerExp for Pretty<T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		LowerExp::fmt(&self.0, f)
	}
}

/// Forward `LowerHex` correctly.
impl<T: LowerHex> LowerHex for Pretty<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		LowerHex::fmt(&self.0, f)
	}
}

/// Forward `Octal` correctly.
impl<T: Octal> Octal for Pretty<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Octal::fmt(&self.0, f)
	}
}

/// Forward `Pointer` correctly.
impl<T: Pointer> Pointer for Pretty<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Pointer::fmt(&self.0, f)
	}
}

/// Forward `UpperExp` correctly.
impl<T: UpperExp> UpperExp for Pretty<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		UpperExp::fmt(&self.0, f)
	}
}

/// Forward `UpperHex` correctly.
impl<T: UpperHex> UpperHex for Pretty<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		UpperHex::fmt(&self.0, f)
	}
}

/// Allow any type to wrap itself in `Pretty`.
impl<T: Sized> From<T> for Pretty<T> {
	fn from(val: T) -> Self {
		Self(val)
	}
}

/// Shorthand conversion to wrap any value in a `Pretty`.
pub trait Prettify: Sized {
	/// Wraps `self` in a `Pretty`.
	fn prettify(self) -> Pretty<Self> {
		self.into()
	}
}
