use core::any::Any;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: *mut u8 = unsafe { 0x8000_0000u32 as *mut u8 };
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB
                                         /*
                                         use x86_64::{
                                             structures::paging::{
                                                 mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
                                             },
                                             VirtAddr,
                                         };
                                         */
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn init_heap(_mapper: &mut impl Any, _frame_allocator: &mut impl Any) -> Result<(), usize> {
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}
