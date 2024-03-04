use core::alloc::GlobalAlloc;

use crate::println;
use multiboot2::{BootInformation, MemoryAreaType};
use linked_list_allocator::LockedHeap;


#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[no_mangle]
pub fn init(mbi: &BootInformation) {
    let memory_areas = mbi
        .memory_map_tag()
        .unwrap()
        .memory_areas();

    let heap_start_addr = mbi.end_address();
    let heap_len = {
        // find the area containing the kernel and the mbi
        let area = memory_areas.iter().find(|a|
            ((a.start_address() as usize) < heap_start_addr) && 
            (((a.start_address() + a.size()) as usize) > heap_start_addr) && 
            (a.typ() == MemoryAreaType::Available)
        ).unwrap();

        (area.size() as usize) - (heap_start_addr - (area.start_address() as usize))
    };

    println!("Initializing heap at range: {} - {} with lenght of: {}", heap_start_addr, heap_start_addr + heap_len, heap_len);

    unsafe {
        ALLOCATOR.lock().init(
            heap_start_addr as *mut u8, 
            heap_len, 
        );
    };


    // TODO checks if allocator implementation is valid
    // remove for release 
    unsafe {
        use core::alloc::Layout;
        let layout = Layout::from_size_align(heap_len, 8).unwrap();
        let ptr = ALLOCATOR.alloc(layout);
        match ptr as u64 {
            0 => println!("allocator test: FAILED"),
            _ => println!("allocator test: SUCCESS"),
        }
        ALLOCATOR.dealloc(ptr, layout);
    }
}


