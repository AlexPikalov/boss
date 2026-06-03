#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(boss::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use boss::{exit_qemu, println, test_case};

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(boss::QemuExitCode::Success);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    boss::test_panic_handler(info)
}

test_case!(test_println, {
    println!("test_println output");
});
