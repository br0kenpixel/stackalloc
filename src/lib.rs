#![no_std]

extern crate alloc;

mod allocregion;
mod memcell;

use alloc::alloc::GlobalAlloc;
pub use allocregion::AllocatedRegion;
use core::alloc::Layout;
use memcell::MemCell;

pub struct StackAllocator<const MEMSIZE: usize, const NALLOCS: usize> {
    regions: MemCell<[AllocatedRegion; NALLOCS]>,
    mempool: MemCell<[u8; MEMSIZE]>,
}

impl<const MEMSIZE: usize, const NALLOCS: usize> StackAllocator<MEMSIZE, NALLOCS> {
    pub const fn new() -> Self {
        const _UNINIT: AllocatedRegion = AllocatedRegion::new_uninit();
        Self {
            regions: MemCell::new([_UNINIT; NALLOCS]),
            mempool: MemCell::new([0; MEMSIZE]),
        }
    }

    pub fn malloc(&self, amount: usize) -> *mut u8 {
        let free_index = self.find_next_free(amount);
        let region = self.find_next_uninit_region();

        *region = AllocatedRegion::new(free_index, amount);
        unsafe { self.mempool_mut().as_mut_ptr().add(free_index) }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn free(&self, ptr: *mut u8) {
        let start_index = unsafe { ptr.offset_from(self.mempool.as_ptr()) } as usize;
        let region = self.find_used_region(start_index);

        *region = AllocatedRegion::new_uninit();
    }

    pub fn mempool_addr(&self) -> usize {
        self.mempool.as_ptr() as usize
    }

    fn mempool_mut(&self) -> &mut [u8] {
        self.mempool.get_mut()
    }

    fn regions_mut(&self) -> &mut [AllocatedRegion] {
        self.regions.get_mut()
    }

    fn find_used_region(&self, index: usize) -> &mut AllocatedRegion {
        self.regions_mut()
            .iter_mut()
            .find(|r| r.init() && r.start() == index)
            .unwrap()
    }

    fn find_next_uninit_region(&self) -> &mut AllocatedRegion {
        self.regions_mut().iter_mut().find(|r| !r.init()).unwrap()
    }

    fn index_in_use(&self, i: usize) -> bool {
        for region in self.regions.iter() {
            if region.init() && region.contains(i) {
                return true;
            }
        }
        false
    }

    fn find_next_free(&self, size: usize) -> usize {
        let mut start = None;
        let mut count = 1;
        for i in 0..MEMSIZE {
            if count == size {
                return start.unwrap();
            }

            if !self.index_in_use(i) {
                if start.is_none() {
                    start = Some(i);
                }
            } else if start.is_some() {
                start = None;
                count = 1;
                continue;
            }

            if start.is_some() {
                count += 1;
            }
        }

        panic!("Failed to find free region");
    }
}

unsafe impl<const MEMSIZE: usize, const NALLOCS: usize> GlobalAlloc
    for StackAllocator<MEMSIZE, NALLOCS>
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.malloc(layout.size() + layout.align())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        self.free(ptr);
    }
}

#[test]
fn test_inner() {
    let allocator: StackAllocator<8, 1> = StackAllocator::new();

    for i in 0..8 {
        assert!(!allocator.index_in_use(i));
    }

    assert_eq!(allocator.find_next_free(4), 0);
}
