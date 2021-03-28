/*! Trait-level `co`nst/`mu`table tracking.

This module provides a system of marker types that can be used to encode write
permissions into type parameters rather than duplicate structures.
!*/

use core::{
	cmp,
	convert::TryFrom,
	fmt::{
		self,
		Debug,
		Display,
		Formatter,
		Pointer,
	},
	hash::{
		Hash,
		Hasher,
	},
	ptr::NonNull,
};

use tap::Pipe;

/// A basic `const` marker.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Const;

/// A basic `mut` marker.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Mut;

/// A frozen wrapper over some other `Mutability` marker.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Frozen<Inner>
where Inner: Mutability
{
	inner: Inner,
}

/** Generalized mutability permissions.

This trait enables referent structures to be generic over the write permissions
of their referent data. As an example, the standard library defines `*const T`
and `*mut T` as two duplicate type families, that cannot share any logic at all.

An equivalent library implementation might be `Ptr<T, M: Mutability>`, where
shared logic can be placed in an `impl<T, M> Ptr<T, M>` block, but unique logic
(such as freezing a `Mut` pointer, or unfreezing a `Frozen<Mut>`) can be placed
in specialized `impl<T> Ptr<T, Mut>` blocks.
**/
pub trait Mutability: 'static + Copy + Sized + seal::Sealed {
	/// Marks whether this type contains mutability permissions within it.
	///
	/// This is `false` for `Const` and `true` for `Mut`. `Frozen` wrappers
	/// atop either of these types inherit their interior marker.
	const CONTAINS_MUTABILITY: bool = false;

	/// Counts the layers of `Frozen<>` wrapping around a base `Const` or `Mut`.
	const PEANO_NUMBER: usize = 0;

	/// Allow instances to be constructed generically.
	const SELF: Self;

	/// One of `*const` or `*mut`.
	const RENDER: &'static str;

	/// Freeze this type, wrapping it in a `const` marker that may later be
	/// removed to thaw it.
	fn freeze(self) -> Frozen<Self> {
		Frozen { inner: self }
	}

	/// Thaw a previously-frozen type, removing its `Frozen` marker and
	/// restoring it to `Self`.
	///
	/// [`PEANO_NUMBER`]: Self::PEANO_NUMBER
	fn thaw(Frozen { inner }: Frozen<Self>) -> Self {
		inner
	}
}

impl Mutability for Const {
	const RENDER: &'static str = "*const";
	const SELF: Self = Self;
}

impl seal::Sealed for Const {
}

impl<Inner> Mutability for Frozen<Inner>
where Inner: Mutability + Sized
{
	const CONTAINS_MUTABILITY: bool = Inner::CONTAINS_MUTABILITY;
	const PEANO_NUMBER: usize = 1 + Inner::PEANO_NUMBER;
	const RENDER: &'static str = Inner::RENDER;
	const SELF: Self = Self { inner: Inner::SELF };
}

impl<Inner> seal::Sealed for Frozen<Inner> where Inner: Mutability + Sized
{
}

impl Mutability for Mut {
	const CONTAINS_MUTABILITY: bool = true;
	const RENDER: &'static str = "*mut";
	const SELF: Self = Self;
}

impl seal::Sealed for Mut {
}

/** A generic non-null pointer with type-system mutability tracking.

# Type Parameters

- `M`: The mutability permissions of the source pointer.
- `T`: The referent type of the source pointer.
**/
pub struct Address<M, T>
where M: Mutability
{
	/// The address value.
	inner: NonNull<T>,
	/// The mutability permissions.
	comu: M,
}

impl<M, T> Address<M, T>
where M: Mutability
{
	/// The dangling pointer.
	pub const DANGLING: Self = Self {
		inner: NonNull::dangling(),
		comu: M::SELF,
	};

	/// Constructs a new `Address` over some pointer value.
	///
	/// You are responsible for selecting the correct `Mutability` marker.
	pub fn new(addr: NonNull<T>) -> Self {
		Self {
			inner: addr,
			comu: M::SELF,
		}
	}

	/// Freezes the `Address` so that it is read-only.
	#[inline(always)]
	pub fn freeze(self) -> Address<Frozen<M>, T> {
		let Self { inner, comu } = self;
		Address {
			inner,
			comu: comu.freeze(),
		}
	}

	/// Thaws the `Address` to its original mutability permission.
	#[inline(always)]
	pub fn thaw(addr: Address<Frozen<M>, T>) -> Self {
		let Address { inner, comu } = addr;
		Self {
			inner,
			comu: Mutability::thaw(comu),
		}
	}

	/// Removes the `Address` type marker, returning the original pointer.
	#[inline(always)]
	pub fn into_inner(self) -> NonNull<T> {
		self.inner
	}

	/// Applies `<*T>::offset`.
	///
	/// # Panics
	///
	/// This panics if the result of applying the offset is the null pointer.
	#[inline]
	pub unsafe fn offset(mut self, count: isize) -> Self {
		self.inner = self
			.inner
			.as_ptr()
			.offset(count)
			.pipe(NonNull::new)
			.unwrap();
		self
	}

	/// Applies `<*T>::wrapping_offset`.
	///
	/// # Panics
	///
	/// This panics if the result of applying the offset is the null pointer.
	#[inline]
	pub fn wrapping_offset(mut self, count: isize) -> Self {
		self.inner = self
			.inner
			.as_ptr()
			.wrapping_offset(count)
			.pipe(NonNull::new)
			.unwrap();
		self
	}

	/// Gets the address as a read-only pointer.
	#[inline(always)]
	pub fn to_const(self) -> *const T {
		self.inner.as_ptr() as *const T
	}

	/// Changes the referent type of the pointer.
	#[inline(always)]
	pub fn cast<U>(self) -> Address<M, U> {
		let Self { inner, comu } = self;
		Address {
			inner: inner.cast::<U>(),
			comu,
		}
	}
}

impl<T> Address<Const, T> {
	/// Force an `Address<Const>` to be `Address<Mut>`.
	///
	/// # Safety
	///
	/// You should only call this on addresses you know to have been created
	/// with `Mut`able permissions and previously removed by [`Address::immut`].
	///
	/// You should prefer using [`Address::freeze`] for temporary, trackable,
	/// immutability constraints instead.
	#[inline(always)]
	pub unsafe fn assert_mut(self) -> Address<Mut, T> {
		Address {
			inner: self.inner,
			..Address::DANGLING
		}
	}
}

impl<T> Address<Mut, T> {
	/// Gets the address as a write-capable pointer.
	#[inline(always)]
	#[allow(clippy::clippy::wrong_self_convention)]
	pub fn to_mut(self) -> *mut T {
		self.inner.as_ptr()
	}

	/// Permanently converts an `Address<Mut>` into an `Address<Const>`.
	#[inline(always)]
	pub fn immut(self) -> Address<Const, T> {
		Address {
			inner: self.inner,
			..Address::DANGLING
		}
	}
}

impl<M, T> Clone for Address<M, T>
where M: Mutability
{
	#[inline(always)]
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> TryFrom<*const T> for Address<Const, T> {
	type Error = NullPtrError;

	#[inline(always)]
	fn try_from(elem: *const T) -> Result<Self, Self::Error> {
		NonNull::new(elem as *mut T)
			.ok_or(NullPtrError)
			.map(|inner| Self { inner, comu: Const })
	}
}

impl<T> From<&T> for Address<Const, T> {
	#[inline(always)]
	fn from(elem: &T) -> Self {
		Self {
			inner: unsafe { NonNull::new_unchecked(elem as *const T as *mut T) },
			comu: Const,
		}
	}
}

impl<T> TryFrom<*mut T> for Address<Mut, T> {
	type Error = NullPtrError;

	#[inline(always)]
	fn try_from(elem: *mut T) -> Result<Self, Self::Error> {
		NonNull::new(elem)
			.ok_or(NullPtrError)
			.map(|inner| Self { inner, comu: Mut })
	}
}

impl<T> From<&mut T> for Address<Mut, T> {
	#[inline(always)]
	fn from(elem: &mut T) -> Self {
		Self {
			inner: elem.into(),
			comu: Mut,
		}
	}
}

impl<M, T> Eq for Address<M, T> where M: Mutability
{
}

impl<M1, M2, T1, T2> PartialEq<Address<M2, T2>> for Address<M1, T1>
where
	M1: Mutability,
	M2: Mutability,
{
	#[inline]
	fn eq(&self, other: &Address<M2, T2>) -> bool {
		self.inner.as_ptr() as usize == other.inner.as_ptr() as usize
	}
}

impl<M, T> Ord for Address<M, T>
where M: Mutability
{
	#[inline]
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.partial_cmp(&other)
			.expect("Addresses have a total ordering")
	}
}

impl<M1, M2, T1, T2> PartialOrd<Address<M2, T2>> for Address<M1, T1>
where
	M1: Mutability,
	M2: Mutability,
{
	#[inline]
	fn partial_cmp(&self, other: &Address<M2, T2>) -> Option<cmp::Ordering> {
		(self.inner.as_ptr() as usize)
			.partial_cmp(&(other.inner.as_ptr() as usize))
	}
}

impl<M, T> Debug for Address<M, T>
where M: Mutability
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
		Debug::fmt(&self.to_const(), fmt)
	}
}

impl<M, T> Pointer for Address<M, T>
where M: Mutability
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
		Pointer::fmt(&self.to_const(), fmt)
	}
}

impl<M, T> Hash for Address<M, T>
where M: Mutability
{
	#[inline(always)]
	fn hash<H>(&self, state: &mut H)
	where H: Hasher {
		self.inner.hash(state)
	}
}

impl<M, T> Copy for Address<M, T> where M: Mutability
{
}

/// [`Address`] cannot be constructed over null pointers.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NullPtrError;

impl Display for NullPtrError {
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
		write!(fmt, "wyz::Address cannot contain a null pointer")
	}
}

#[cfg(feature = "std")]
impl std::error::Error for NullPtrError {
}

#[doc(hidden)]
mod seal {
	#[doc(hidden)]
	pub trait Sealed {}
}
