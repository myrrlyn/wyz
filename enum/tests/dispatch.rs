/*! Test that the `dispatch` attribute correctly generates code.

This test checks that enum variants are correctly handled in the codegen macro,
and that all the expected code is generated.
!*/

extern crate wyz_enum;

use wyz_enum::discern;

#[discern]
pub enum Example<'a, T: ?Sized>
where T: std::fmt::Debug {
	A,
	B(i32),
	C { x: f32, y: f32 },
	D(&'a T),
}

#[test]
fn discern() {
	let a: Example<'static, [u8]> = Example::A;
	let b: Example<'static, [u8]> = Example::B(5);
	let c: Example<'static, [u8]> = Example::C { x: 1.0, y: -2.0 };
	let d: Example<'static, [u8]> = Example::D(&[1, 2, 3]);

	assert!(a.is_a());
	assert!(b.is_b());
	assert!(c.is_c());
	assert!(d.is_d());

	assert_eq!(a.variant(), ExampleVariant::A);
	assert_eq!(b.variant(), ExampleVariant::B);
	assert_eq!(c.variant(), ExampleVariant::C);
	assert_eq!(d.variant(), ExampleVariant::D);
}
