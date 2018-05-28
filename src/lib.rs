#![feature(unique)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(lang_items)]
#![no_std]

extern crate volatile;
extern crate rlibc;
extern crate spin;
extern crate multiboot2;
extern crate x86_64;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod vga_buffer;

mod memory;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
        use memory::FrameAllocator;

    vga_buffer::clear_screen();
    println!("what is gucci??{}", "!" );

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");
    let elf_section_tag = boot_info.elf_sections_tag()
        .expect("elf-section tag required");

    let kernel_start = elf_section_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_section_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();
    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel_start:{:x}, kernel_end{:x}", kernel_start, kernel_end);
    println!("multiboot_start:{:x}, multiboot_end:{:x}", multiboot_start, multiboot_end );

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());

    println!("{:?}", frame_allocator.allocate_frame());

    enable_nxe_bit();
    enable_write_protect_bit();
    memory::remap_the_kernel(&mut frame_allocator, boot_info);
    println!("It did not crash!");

    loop{}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
    line: u32) -> !
{
    println!("\n\nPANIC IN {} at line {}:",file, line);
    println!("   {}", fmt);
    loop{}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}
