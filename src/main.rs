#![allow(unused_imports)]
#![feature(custom_test_frameworks)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![test_runner(nuclea_r_os::test_runner)]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use nuclea_r_os::{
    memory::{
        heap,
        paging::{init_offset_page_table, BootInfoFrameAllocator},
    },
    output::vga,
    println,
};
use x86_64::{
    structures::paging::{Page, PageTable, Translate},
    VirtAddr,
};

entry_point!(kernel_main);

///
/// Kernel Entry Point
///
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Welcome! {}", ":D\n");
    nuclea_r_os::init();

    let phys_mem_offset = VirtAddr::new(_boot_info.physical_memory_offset);
    let mut mapper = unsafe { init_offset_page_table(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(&_boot_info.memory_map) };

    heap::init_heap(&mut mapper, &mut frame_allocator).expect("Heap Initialization Failed.");

    // Allocate num on heap
    let heap_val = Box::new(39);
    println!("heap_val at {:p}", heap_val);

    // Create dynamically-sized vector
    let mut vec = Vec::new();
    for i in 0..359 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // Create a reference-counted vector -> automatically freed when count reaches 0
    let ref_counted = Rc::new(vec![4, 5, 6, 7]);
    let cloned_ref = ref_counted.clone();
    println!(
        "Current reference count is: {}.",
        Rc::strong_count(&cloned_ref)
    );
    core::mem::drop(ref_counted);
    println!("Reference count is {} now.", Rc::strong_count(&cloned_ref));

    #[cfg(test)]
    test_main();

    println!("\n! === KERNEL END === !\n");
    nuclea_r_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    vga::WRITER.lock().color_code = vga::ColorCode {
        foreground_color: vga::Color::Red,
        background_color: vga::Color::Black,
    };

    println!("\n! === PANIC === !");
    println!(
        " Location: {}[{}:{}]",
        _info.location().unwrap().file(),
        _info.location().unwrap().line(),
        _info.location().unwrap().column()
    );
    println!(" Message: \"{:#?}\"", _info.message().unwrap());
    // println!(" Payload: {:#?}", _info.payload().downcast_ref::<&str>());
    println!("! ============= !");

    nuclea_r_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    nuclea_r_os::test_panic_handler(_info);
}

#[test_case]
fn test_grounding_assertion() {
    // Ensure all the laws of the universe are still in line.
    assert_eq!(1, 1);
}
