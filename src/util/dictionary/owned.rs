use std::ops::{Deref, DerefMut};
use std::iter::FromIterator;
use std::ptr;

use ffi::*;
use super::mutable;

pub struct Owned<'a> {
	inner: mutable::Ref<'a>,
}

impl<'a> Owned<'a> {
	pub unsafe fn own(ptr: *mut AVDictionary) -> Self {
		Owned { inner: mutable::Ref::wrap(ptr) }
	}

	pub unsafe fn disown(mut self) -> *mut AVDictionary {
		let result = self.inner.as_mut_ptr();
		self.inner = mutable::Ref::wrap(ptr::null_mut());

		result
	}
}

impl<'a> Owned<'a> {
	pub fn new() -> Self {
		unsafe {
			Owned { inner: mutable::Ref::wrap(ptr::null_mut()) }
		}
	}
}

impl<'a, 'b> FromIterator<(&'b str, &'b str)> for Owned<'a> {
	fn from_iter<T: IntoIterator<Item=(&'b str, &'b str)>>(iterator: T) -> Self {
		let mut result = Owned::new();

		for (key, value) in iterator {
			result.set(key, value);
		}

		result
	}
}

impl<'a> Deref for Owned<'a> {
	type Target = mutable::Ref<'a>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<'a> DerefMut for Owned<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<'a> Clone for Owned<'a> {
	fn clone(&self) -> Self {
		let mut dictionary = Owned::new();
		dictionary.clone_from(self);

		dictionary
	}

	fn clone_from(&mut self, source: &Self) {
		unsafe {
			let mut ptr = self.as_mut_ptr();
			av_dict_copy(&mut ptr, source.as_ptr(), 0);
			self.inner = mutable::Ref::wrap(ptr);
		}
	}
}

impl<'a> Drop for Owned<'a> {
	fn drop(&mut self) {
		unsafe {
			av_dict_free(&mut self.inner.as_mut_ptr());
		}
	}
}