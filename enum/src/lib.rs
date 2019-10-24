/*! Procedural macros for working with `enum`s.
!*/

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
	Fields,
	Ident,
	ItemEnum,
	parse_macro_input,
};

/** Generates code to permit testing which variant of an `enum` is live.

Standard Rust only permits testing enum variants through the `match` expression,
and requires manual implementation of `is_variant(&self) -> bool` test
functions. This can become tedious when working with many `enum` types, or where
an enum has many variants. This macro generates those test functions, and a
discriminant-only sibling enum, from each `enum` declaration to which it is
attached.

```rust
extern crate wyz_enum;
use wyz_enum::discern;

#[discern]
pub enum Example {
    A,
    B(i32),
}

let a = Example::A;
let b = Example::B(5);

assert!(a.is_a());
assert!(b.is_b());
assert_eq!(a.variant(), ExampleVariant::A);
assert_eq!(b.variant(), ExampleVariant::B);
```

The macro produces the following code for any attached enum:

```rust,ignore
# extern crate wyz_enum;
# use wyz_enum::discern;
// #[discern]
enum Name {
    One(i32),
    Two { x: f32, y: f32 },
}

// produced by #[discern]

impl Name {
    fn is_one(&self) -> bool {
        match self {
            Name::One(..) => true,
            _ => false,
        }
    }
    fn is_two(&self) -> bool {
        match self {
            Name::Two{..} => true,
            _ => false,
        }
    }
    fn variant(&self) -> NameVariant {

    }
}

enum NameVariant {
    One,
    Two,
}
```

All generated code uses the same visibility specifier as the source enum.
**/
#[proc_macro_attribute]
pub fn discern(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let item = parse_macro_input!(item as ItemEnum);
	let vis = &item.vis;
	let variants = &item.variants;
	let fn_names = (&variants)
		.into_iter()
		.map(|v| {
			let snake = pascal_to_snake(&v.ident.to_string());
			let prepend = if snake.starts_with("_") { "" } else { "_" };
			Ident::new(&format!("is{}{}", prepend, snake), Span::call_site())
		})
		.collect::<Vec<_>>();
	let var_fields = (&variants)
		.into_iter()
		.map(|v| match v.fields {
			Fields::Unit => quote! {},
			Fields::Unnamed( .. ) => quote! { ( .. ) },
			Fields::Named( .. ) => quote! { { .. } },
		})
		.collect::<Vec<_>>();

	let var_names_orig = (&variants)
		.into_iter()
		.map(|v| &v.ident)
		.collect::<Vec<_>>();
	let var_names_a = (&var_names_orig).into_iter();
	let var_names_b = var_names_a.clone();
	let var_names_c = var_names_a.clone();
	let kind_vars_a = var_names_a.clone();
	let kind_vars_b = var_names_a.clone();
	let kind_name = Ident::new(&format!("{}Variant", item.ident), Span::call_site());

	let generics = &item.generics;
	let (g_impl, g_ty, g_where) = generics.split_for_impl();
	let i_name = &item.ident;
	let out = quote! {
		#item

		impl #g_impl #i_name #g_ty #g_where {
			#(
				/// Tests if `self` is the `#var_names_a` variant.
				#vis fn #fn_names(&self) -> bool {
					match self {
						Self :: #var_names_b #var_fields => true,
						_ => false,
					}
				}
			)*

			/// Produces a sibling enum with all the fields removed, leaving
			/// only the names of each variant.
			///
			/// This is useful for cases where you may need to test against
			/// multiple discriminants, but are not attempting to bind their
			/// fields.
			#vis fn variant(&self) -> #kind_name {
				match self {
					#(
						Self :: #var_names_c #var_fields => #kind_name :: #kind_vars_a ,
					)*
				}
			}
		}

		//  TODO(myrrlyn): Make production of this type depend on macro argument

		/// A sibling enum to `#i_name`, with identical variant names but
		/// without any fields.
		#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
		#vis enum #kind_name {
			#(
				#kind_vars_b ,
			)*
		}
	};
	out.into()
}

/// Translates a `PascalCase` name to `_snake_case`.
fn pascal_to_snake(text: &str) -> String {
	let mut out = String::with_capacity(text.len());
	text.chars().for_each(|c| if c.is_uppercase() {
		out.push('_');
		c.to_lowercase().for_each(|c| out.push(c));
	}
	else {
		out.push(c);
	});
	out
}
