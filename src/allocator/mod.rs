use core::alloc;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: *mut u8 = 0x00_80_00_00u32 as *mut u8;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn init_heap() -> Result<(), usize> {
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}
