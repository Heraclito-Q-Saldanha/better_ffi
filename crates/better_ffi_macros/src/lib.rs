use quote::quote;

#[proc_macro_attribute]
pub fn safe_ffi(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let data = syn::parse_macro_input!(item as syn::Item);
	match data {
		syn::Item::Fn(data) => parse_fn(data),
		syn::Item::Struct(data) => parse_struct(data),
		syn::Item::Enum(data) => parse_enum(data),
		_ => panic!(),
	}
	.into()
}

fn parse_fn(data: syn::ItemFn) -> proc_macro2::TokenStream {
	let mut asserts = quote!();
	if let syn::ReturnType::Type(_, ty) = &data.sig.output {
		asserts = quote!(
			#asserts
			const _: fn() = || {
				fn assert_impl<T: ?Sized + better_ffi::SafeFFi>(){}
				assert_impl::<#ty>();
			};
		);
	}
	for arg in &data.sig.inputs {
		if let syn::FnArg::Typed(syn::PatType { ty, .. }) = arg {
			asserts = quote!(
				#asserts
				const _: fn() = || {
					fn assert_impl<T: ?Sized + better_ffi::SafeFFi>(){}
					assert_impl::<#ty>();
				};
			);
		} else {
			panic!();
		}
	}

	quote!(
		#asserts
		#[no_mangle]
		extern "C" #data
	)
}

fn parse_struct(data: syn::ItemStruct) -> proc_macro2::TokenStream {
	let mut asserts = quote!();
	let ident = &data.ident;
	for syn::Field { ty, .. } in &data.fields {
		asserts = quote!(
			#asserts
			const _: fn() = || {
				fn assert_impl<T: ?Sized + better_ffi::SafeFFi>(){}
				assert_impl::<#ty>();
			};
		);
	}
	quote!(
		#asserts
		#[repr(C)]
		#data
		unsafe impl better_ffi::SafeFFi for #ident{}
	)
}

fn parse_enum(data: syn::ItemEnum) -> proc_macro2::TokenStream {
	let mut asserts = quote!();
	let ident = &data.ident;
	for syn::Variant { fields, .. } in &data.variants {
		match fields {
			syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
				for syn::Field { ty, .. } in named {
					asserts = quote!(
						#asserts
						const _: fn() = || {
							fn assert_impl<T: ?Sized + better_ffi::SafeFFi>(){}
							assert_impl::<#ty>();
						};
					);
				}
			}
			syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
				for syn::Field { ty, .. } in unnamed {
					asserts = quote!(
						#asserts
						const _: fn() = || {
							fn assert_impl<T: ?Sized + better_ffi::SafeFFi>(){}
							assert_impl::<#ty>();
						};
					);
				}
			}
			_ => {}
		}
	}
	quote!(
		#asserts
		#[repr(C)]
		#data
		unsafe impl better_ffi::SafeFFi for #ident{}
	)
}
