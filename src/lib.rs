#![feature(allocator_api, nonnull)]
use std::heap::{Alloc, Heap, Layout};
use std::{mem, ptr};
const kBlockSize: usize = 4096;

struct Arena {
    blocks_: *mut u8,
    alloc_bytes_remaining_: usize,
}

fn allocator() {
    unsafe {
        let ptr = Heap.alloc(Layout::from_size_align(512 * 1024, 4 * 1024).unwrap())
            .unwrap_or_else(|e| Heap.oom(e));
        let mut raw: *mut i32 = mem::transmute::<*mut u8, *mut i32>(ptr);
        for i in 0..(512 * 1024 / 4) {
            ptr::write(raw, i as i32);
            raw = raw.offset(1)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::allocator()
    }
}
