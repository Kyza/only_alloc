#![no_std]
#![cfg_attr(feature = "alloc_trait", feature(allocator_api))]

use core::alloc::{GlobalAlloc, Layout};

#[cfg(feature = "alloc_trait")]
use core::alloc::{AllocError, Allocator};
#[cfg(feature = "alloc_trait")]
use core::ptr::NonNull;

/// A global allocator wrapper that disables deallocating memory within it.
/// ```rs
/// use mimalloc::MiMalloc;
/// use only_alloc::GlobalOnlyAlloc;
///
/// #[global_allocator]
/// static ONLY_MIMALLOC: GlobalOnlyAlloc<MiMalloc> = GlobalOnlyAlloc(MiMalloc);
/// ```
#[derive(Copy, Clone, Default, Debug)]
pub struct GlobalOnlyAlloc<T: GlobalAlloc>(pub T);

unsafe impl<T: GlobalAlloc> GlobalAlloc for GlobalOnlyAlloc<T> {
	#[inline]
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unsafe { self.0.alloc(layout) }
	}

	#[inline]
	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
		unsafe { self.0.alloc_zeroed(layout) }
	}

	#[inline]
	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}

	#[inline]
	unsafe fn realloc(
		&self,
		ptr: *mut u8,
		layout: Layout,
		new_size: usize,
	) -> *mut u8 {
		if new_size > layout.size() {
			unsafe { self.0.realloc(ptr, layout, new_size) }
		} else {
			ptr
		}
	}
}

#[cfg(feature = "alloc_trait")]
/// An allocator wrapper that disables deallocating memory within it.
#[derive(Copy, Clone, Default, Debug)]
pub struct OnlyAlloc<T: Allocator>(pub T);

#[cfg(feature = "alloc_trait")]
unsafe impl<T: Allocator> Allocator for OnlyAlloc<T> {
	#[inline]
	fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		self.0.allocate(layout)
	}

	#[inline]
	unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}

	#[inline]
	fn allocate_zeroed(
		&self,
		layout: Layout,
	) -> Result<NonNull<[u8]>, AllocError> {
		self.0.allocate_zeroed(layout)
	}

	#[inline]
	unsafe fn grow(
		&self,
		ptr: NonNull<u8>,
		old_layout: Layout,
		new_layout: Layout,
	) -> Result<NonNull<[u8]>, AllocError> {
		unsafe { self.0.grow(ptr, old_layout, new_layout) }
	}

	#[inline]
	unsafe fn grow_zeroed(
		&self,
		ptr: NonNull<u8>,
		old_layout: Layout,
		new_layout: Layout,
	) -> Result<NonNull<[u8]>, AllocError> {
		unsafe { self.0.grow_zeroed(ptr, old_layout, new_layout) }
	}

	#[inline]
	unsafe fn shrink(
		&self,
		ptr: NonNull<u8>,
		old_layout: Layout,
		new_layout: Layout,
	) -> Result<NonNull<[u8]>, AllocError> {
		if new_layout.size() > old_layout.size()
			|| unsafe { ptr.as_ref() } % new_layout.align() as u8 != 0
		{
			return Err(AllocError);
		}
		Ok(NonNull::slice_from_raw_parts(ptr, new_layout.size()))
	}

	#[inline]
	fn by_ref(&self) -> &Self
	where
		Self: Sized,
	{
		self
	}
}
