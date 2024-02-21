use multiboot2::{BootInformation, MemoryAreaType, MemoryMapTag};
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

    unsafe {
        ALLOCATOR.lock().init(
            heap_start_addr as *mut u8, 
            heap_len, 
        );
    };

}


