#![no_std]
#![no_main]

use core::arch::asm;
use core::ptr::write_volatile;

/*
    rs-cli has two special memory locations for output and controlling the simulation.
        * 0x80000000 - Write a character here to output it to stdout
        * 0x80000004 - Write a non zero value here to terminate the simulation
*/

static STDOUT_ADDRESS: u32 = 0x8000_0000;

// Entry point of the program
#[no_mangle]
#[link_section = ".text.start"]
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        ".global _start",
        // Initialize the .bss section to zero
        "la x1, _bss_start", // Load the start address of .bss
        "la x2, _bss_end",   // Load the end address of .bss
        "beq x1, x2, 1f",    // If _bss_start == _bss_end, skip zeroing .bss; 1f = main_entry
        "0:",                // Loop label for zeroing .bss (zero_bss_loop)
        "sw x0, 0(x1)",      // Store zero at the current address in .bss
        "addi x1, x1, 4",    // Move to the next word in .bss
        "bne x1, x2, 0b",    // Repeat until the end of .bss is reached; 0b = zero_bss_loop
        // Main entry point
        "1:",                  // Label for main entry point (main_entry)
        "la sp, _stack_start", // Load the stack pointer address
        "jal ra, main",       // Jump to the main function
        // Exit point (RSS expects that to terminate the program something will be written to the address 0x80000004)
        "la x1, 0x80000004", // Load address
        "li x2, 1",                       // Write 1
        "sw x2, 0(x1)",                   // Store it to the address 0x80000004
        options(noreturn),
    );
}

#[no_mangle]
#[link_section = ".text"]
unsafe extern "C" fn main() {
    for byte in "Hello, world!\n".bytes() {
        unsafe { write_volatile(STDOUT_ADDRESS as *mut u8, byte) };
    }
}

#[panic_handler]
fn on_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
