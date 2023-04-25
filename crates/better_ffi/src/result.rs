use crate::*;

#[must_use]
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum FFIResult<T: SafeFFi, E: SafeFFi> {
	Ok(T),
	Err(E),
}

unsafe impl<T: SafeFFi, E: SafeFFi> SafeFFi for FFIResult<T, E> {}

impl<T: SafeFFi, E: SafeFFi> From<Result<T, E>> for FFIResult<T, E> {
	#[inline]
	fn from(value: Result<T, E>) -> Self {
		match value {
			Result::Ok(t) => Self::Ok(t),
			Result::Err(e) => Self::Err(e),
		}
	}
}

impl<T: SafeFFi, E: SafeFFi> From<FFIResult<T, E>> for Result<T, E> {
	#[inline]
	fn from(value: FFIResult<T, E>) -> Self {
		match value {
			FFIResult::Ok(t) => Self::Ok(t),
			FFIResult::Err(e) => Self::Err(e),
		}
	}
}

impl<T: SafeFFi, E: SafeFFi> FFIResult<T, E> {
	#[inline]
	#[must_use]
	pub fn unwrap(self) -> T {
		match self {
			Self::Ok(t) => t,
			Self::Err(_) => panic!(),
		}
	}
	#[inline]
	#[must_use]
	pub fn expect(self, msg: &str) -> T {
		match self {
			Self::Ok(t) => t,
			Self::Err(_) => panic!("{msg}"),
		}
	}
	#[inline]
	#[must_use]
	pub const fn is_ok(&self) -> bool {
		matches!(self, FFIResult::Ok(_))
	}
	#[inline]
	#[must_use]
	pub const fn is_err(&self) -> bool {
		matches!(self, FFIResult::Err(_))
	}
}
