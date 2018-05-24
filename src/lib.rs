#![feature(unique)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(lang_items)]
#![no_std]

extern crate volatile;
extern crate rlibc;
extern crate spin;


#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page

    use core::fmt::Write;
    vga_buffer::clear_screen();
    vga_buffer::WRITER.lock().write_str("Hello worrrrlddd!!!");
    write!(vga_buffer::WRITER.lock(), "The numbers are {} and {}", 42, 1.0/3.0);
    println!("this");
    loop{}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
