use crate::*;

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum FFIOption<T: SafeFFi> {
	Some(T),
	None,
}

unsafe impl<T: SafeFFi> SafeFFi for FFIOption<T> {}

impl<T: SafeFFi> From<Option<T>> for FFIOption<T> {
	#[inline]
	fn from(value: Option<T>) -> Self {
		match value {
			Option::Some(t) => Self::Some(t),
			Option::None => Self::None,
		}
	}
}

impl<T: SafeFFi> From<FFIOption<T>> for Option<T> {
	#[inline]
	fn from(value: FFIOption<T>) -> Self {
		match value {
			FFIOption::Some(t) => Self::Some(t),
			FFIOption::None => Self::None,
		}
	}
}

impl<T: SafeFFi> FFIOption<T> {
	#[inline]
	#[must_use]
	pub fn unwrap(self) -> T {
		match self {
			Self::Some(t) => t,
			Self::None => panic!(),
		}
	}
	#[inline]
	#[must_use]
	pub fn expect(self, msg: &str) -> T {
		match self {
			Self::Some(t) => t,
			Self::None => panic!("{msg}"),
		}
	}
	#[inline]
	#[must_use]
	pub const fn is_some(&self) -> bool {
		matches!(self, FFIOption::Some(_))
	}
	#[inline]
	#[must_use]
	pub const fn is_none(&self) -> bool {
		matches!(self, FFIOption::None)
	}
}
