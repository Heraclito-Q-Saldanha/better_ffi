use crate::*;
use std::{fmt, ops};

#[repr(C)]
#[derive(PartialEq, Eq)]
pub struct FFIVec<T: SafeFFi> {
	ptr: *mut T,
	len: usize,
	cap: usize,
}

unsafe impl<T: SafeFFi> SafeFFi for FFIVec<T> {}

impl<T: SafeFFi> Drop for FFIVec<T> {
	#[inline]
	fn drop(&mut self) {
		drop(unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) });
	}
}

impl<T: SafeFFi + fmt::Debug> fmt::Debug for FFIVec<T> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let value: &[T] = ops::Deref::deref(&self);
		write!(f, "{:?}", value)
	}
}

impl<T: SafeFFi> ops::Deref for FFIVec<T> {
	type Target = [T];
	#[inline]
	fn deref(&self) -> &Self::Target {
		unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
	}
}

impl<T: SafeFFi> ops::DerefMut for FFIVec<T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
	}
}

impl<T: SafeFFi> From<Vec<T>> for FFIVec<T> {
	#[inline]
	fn from(mut value: Vec<T>) -> Self {
		let ptr = value.as_mut_ptr();
		let len = value.len();
		let cap = value.capacity();
		std::mem::forget(value);
		FFIVec { ptr, len, cap }
	}
}

impl<T: SafeFFi> From<FFIVec<T>> for Vec<T> {
	#[inline]
	fn from(value: FFIVec<T>) -> Self {
		let ptr = value.ptr;
		let len = value.len;
		let cap = value.cap;
		std::mem::forget(value);
		unsafe { Vec::from_raw_parts(ptr, len, cap) }
	}
}

impl<T: SafeFFi + Clone> From<&[T]> for FFIVec<T> {
	#[inline]
	fn from(value: &[T]) -> Self {
		Vec::from(value).into()
	}
}

impl<T: SafeFFi> FFIVec<T> {
	#[inline]
	#[must_use]
	pub fn as_ptr(&self) -> *const T {
		self.ptr
	}
	#[inline]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.ptr
	}
	#[inline]
	#[must_use]
	pub fn capacity(&self) -> usize {
		self.cap
	}
	#[inline]
	#[must_use]
	pub fn len(&self) -> usize {
		self.len
	}
	#[inline]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
	#[inline]
	#[must_use]
	pub unsafe fn from_raw_parts(ptr: *mut T, len: usize, cap: usize) -> Self {
		Self { ptr, len, cap }
	}
}
