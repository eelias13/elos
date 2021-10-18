#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(elos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use elos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    elos::init();

    #[cfg(test)]
    test_main();

    println!("initialization complete");
    println!("elos operational");

    elos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    elos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    elos::test_panic_handler(info)
}
