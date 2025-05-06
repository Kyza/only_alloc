#![no_std]

use core::alloc::{GlobalAlloc, Layout};

use alloc_metrics::{MetricAlloc, global_metrics};
use mimalloc::MiMalloc;
use only_alloc::GlobalOnlyAlloc;

fn reset_metrics() {
	global_metrics().allocations = 0;
	global_metrics().allocated_bytes = 0;
}

static ONLY_MIMALLOC: GlobalOnlyAlloc<MetricAlloc<MiMalloc>> =
	GlobalOnlyAlloc(MetricAlloc::new(MiMalloc));

#[test]
fn use_after_free() {
	let l = Layout::new::<u8>();

	assert_eq!(global_metrics().allocations, 0);

	for i in 0..1000 {
		let num = rand::random_range(0..=u8::MAX);

		let num_ptr = unsafe { ONLY_MIMALLOC.alloc(l) };
		unsafe { *num_ptr = num };
		unsafe { ONLY_MIMALLOC.dealloc(num_ptr, l) };

		assert_eq!(global_metrics().allocations, i + 1);

		assert_eq!(unsafe { *num_ptr }, num);
	}

	reset_metrics();
}
