mod option;
mod result;
mod string;
mod vec;

pub use better_ffi_macros::*;
pub use option::*;
pub use result::*;
pub use string::*;
pub use vec::*;

#[cfg(feature = "libloading")]
mod lib {
	use crate::*;
	pub use libloading::Error;
	pub use libloading::Symbol;
	pub struct Library {
		lib: libloading::Library,
	}
	impl Library {
		#[inline]
		pub unsafe fn load(path: &str) -> Result<Self, Error> {
			Ok(Self {
				lib: libloading::Library::new(path)?,
			})
		}
		#[inline]
		pub unsafe fn get<T: SafeFFi>(&self, symbol: &[u8]) -> Result<Symbol<T>, Error> {
			self.lib.get(symbol)
		}
		#[inline]
		pub unsafe fn get_unchecked<T>(&self, symbol: &[u8]) -> Result<Symbol<T>, Error> {
			self.lib.get(symbol)
		}
	}
}

#[cfg(feature = "libloading")]
pub use lib::*;

pub unsafe trait SafeFFi {}

unsafe impl<T: SafeFFi> SafeFFi for &'static T {}
unsafe impl<T: SafeFFi> SafeFFi for &'static mut T {}
unsafe impl<T> SafeFFi for *const T {}
unsafe impl<T> SafeFFi for *mut T {}
unsafe impl SafeFFi for usize {}
unsafe impl SafeFFi for isize {}
unsafe impl SafeFFi for u8 {}
unsafe impl SafeFFi for i8 {}
unsafe impl SafeFFi for u16 {}
unsafe impl SafeFFi for i16 {}
unsafe impl SafeFFi for u32 {}
unsafe impl SafeFFi for i32 {}
unsafe impl SafeFFi for u64 {}
unsafe impl SafeFFi for i64 {}
unsafe impl SafeFFi for f32 {}
unsafe impl SafeFFi for f64 {}
unsafe impl SafeFFi for bool {}

macro_rules! safe_fn {
    () => {
        unsafe impl SafeFFi for extern "C" fn() {}
    };
    ($R:tt) => {
        unsafe impl<$R: SafeFFi> SafeFFi for extern "C" fn() -> $R {}
    };
    ($R:tt, $($G:tt),+) => {
        unsafe impl<$($G: SafeFFi),+> SafeFFi for extern "C" fn($($G),+) {}
        unsafe impl<$R: SafeFFi, $($G: SafeFFi),+> SafeFFi for extern "C" fn($($G),+) -> $R {}
    };
}

safe_fn!();
safe_fn!(R);
safe_fn!(R, A);
safe_fn!(R, A, B);
safe_fn!(R, A, B, C);
safe_fn!(R, A, B, C, D);
safe_fn!(R, A, B, C, D, E);
safe_fn!(R, A, B, C, D, E, F);
safe_fn!(R, A, B, C, D, E, F, G);
safe_fn!(R, A, B, C, D, E, F, G, H);
safe_fn!(R, A, B, C, D, E, F, G, H, I);
safe_fn!(R, A, B, C, D, E, F, G, H, I, J);
safe_fn!(R, A, B, C, D, E, F, G, H, I, J, K);
safe_fn!(R, A, B, C, D, E, F, G, H, I, J, K, L);
safe_fn!(R, A, B, C, D, E, F, G, H, I, J, K, L, N);
safe_fn!(R, A, B, C, D, E, F, G, H, I, J, K, L, N, M);
safe_fn!(R, A, B, C, D, E, F, G, H, I, J, K, L, N, M, O);
safe_fn!(R, A, B, C, D, E, F, G, H, I, J, K, L, N, M, O, P);
