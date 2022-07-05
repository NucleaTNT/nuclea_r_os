#![feature(custom_test_frameworks)]
#![no_main]
#![no_std]
#![test_runner(nuclea_r_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use nuclea_r_os::memory::{
    self,
    heap::{self, HEAP_SIZE},
    paging::BootInfoFrameAllocator,
};
use x86_64::VirtAddr;

entry_point!(main);

fn main(_boot_info: &'static BootInfo) -> ! {
    nuclea_r_os::init();

    let phys_mem_offset = VirtAddr::new(_boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::paging::init_offset_page_table(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(&_boot_info.memory_map) };
    heap::init_heap(&mut mapper, &mut frame_allocator).expect("Heap Initialization Failed.");

    test_main();

    nuclea_r_os::hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    nuclea_r_os::test_panic_handler(_info);
}

#[test_case]
fn test_simple_allocation() {
    let heap_val1 = Box::new(17);
    let heap_val2 = Box::new(38);
    assert_eq!(*heap_val1, 17);
    assert_eq!(*heap_val2, 38);
}

#[test_case]
fn test_dynamic_vals() {
    let n = 1000;
    let mut vec = Vec::new();

    for i in 0..n {
        vec.push(i);
    }

    // nth partial sum
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn test_memory_freeing() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn test_memory_freeing_long_lived() {
    let long_lived = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1);
}
