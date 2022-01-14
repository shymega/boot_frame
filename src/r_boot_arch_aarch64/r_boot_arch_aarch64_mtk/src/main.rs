#![no_std] // disable std, running on embedded
#![no_main] // we don't use conventional main on embedded
#![feature(alloc_error_handler)]

extern crate core;
extern crate alloc;

use core::panic::PanicInfo;
use core::ptr::null_mut;
use alloc::alloc::{GlobalAlloc, Layout};

// define dummy allocator - see os.phil-opp.com
// this is temporary, and will NOT boot
pub struct DummyAlloc;

// implement dummy allocator - again, thanks to phil-opp
unsafe impl GlobalAlloc for DummyAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("Deallocation should NEVER be called. Bailing.");
    }
}

// now we specify the allocator - global
#[global_allocator]
static ALLOCATOR: DummyAlloc = DummyAlloc;

// dummy handler for allocation errors
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Error during alloc: {:?}", layout);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // we don't handle this yet, so let's loop
    // TODO: add handling logic
    loop {}
}

#[no_mangle]
pub extern "C" fn r_boot_main() {
    loop {}
}