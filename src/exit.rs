/*! `exit!` macro

The `exit!` macro simplifies exiting with an error code, and optionally printing
an error message prior to exit.

# Examples

This example exits with status `1`.

```rust,should_panic
wyz::exit!();
```

This example exits with status `2`.

```rust,should_panic
wyz::exit!(2);
```

This example exits with status `3`, and uses `eprintln!` to print an error
message before exiting.

```rust,should_panic
wyz::exit!(3, "Error status: {}", "testing");
!*/

#![cfg(feature = "std")]

/// `exit!` macro
#[macro_export]
macro_rules! exit {
	() => {
		$crate::exit!(1);
	};

	( $num:expr ) => {
		::std::process::exit($num);
	};

	( $num:expr, $fmt:expr $( , $arg:expr )* ) => {{
		let _: std::io::Result<()> = {
			let err = std::io::stderr();
			let mut err = err.lock();
			writeln!(err, $fmt $( , $arg )*)?;
			Ok(())
		};
		$crate::exit!($num);
	}};
}
