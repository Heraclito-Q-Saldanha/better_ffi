use crate::*;
use std::{fmt, ops};

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct FFIString {
	vec: FFIVec<u8>,
}

unsafe impl SafeFFi for FFIString {}

impl ops::Deref for FFIString {
	type Target = str;
	#[inline]
	fn deref(&self) -> &Self::Target {
		unsafe { std::str::from_utf8_unchecked(&self.vec) }
	}
}

impl ops::DerefMut for FFIString {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { std::str::from_utf8_unchecked_mut(&mut self.vec) }
	}
}

impl fmt::Display for FFIString {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let value: &str = ops::Deref::deref(&self);
		write!(f, "{}", value)
	}
}

impl fmt::Debug for FFIString {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let value: &str = ops::Deref::deref(&self);
		write!(f, "{:?}", value)
	}
}

impl From<String> for FFIString {
	#[inline]
	fn from(value: String) -> Self {
		unsafe { Self::from_utf8_unchecked(value.into_bytes().into()) }
	}
}

impl From<FFIString> for String {
	#[inline]
	fn from(value: FFIString) -> Self {
		unsafe { Self::from_utf8_unchecked(value.into_bytes().into()) }
	}
}

impl From<&str> for FFIString {
	#[inline]
	fn from(value: &str) -> Self {
		String::from(value).into()
	}
}

impl FFIString {
	#[inline]
	#[must_use]
	pub fn as_ptr(&self) -> *const u8 {
		self.vec.as_ptr()
	}
	#[inline]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut u8 {
		self.vec.as_mut_ptr()
	}
	#[inline]
	#[must_use]
	pub fn capacity(&self) -> usize {
		self.vec.capacity()
	}
	#[inline]
	#[must_use]
	pub fn len(&self) -> usize {
		self.vec.len()
	}
	#[inline]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
	#[inline]
	#[must_use]
	pub fn into_bytes(self) -> FFIVec<u8> {
		self.vec
	}
	#[inline]
	#[must_use]
	pub fn as_str(&self) -> &str {
		self
	}
	#[inline]
	#[must_use]
	pub fn as_mut_str(&mut self) -> &mut str {
		self
	}
	#[inline]
	#[must_use]
	pub unsafe fn from_utf8_unchecked(bytes: FFIVec<u8>) -> Self {
		Self { vec: bytes }
	}
	#[inline]
	#[must_use]
	pub unsafe fn from_raw_parts(ptr: *mut u8, len: usize, cap: usize) -> Self {
		Self {
			vec: FFIVec::from_raw_parts(ptr, len, cap),
		}
	}
}
