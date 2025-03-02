#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(flight_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use flight_os::println;
use core::panic::PanicInfo;
mod sim;



pub fn setup(){
    println!("Hello World{}", "!");
}

pub fn load_map_files(){
    println!("Loading map files...");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    setup();
    load_map_files();

    // Sleep 1 second (busy-wait)
    //flight_os::sleep(1_000_000);
    
    // Switch to 320x200 graphics mode and draw a centered white line
    flight_os::graphics::switch_to_graphics_mode();
    sim::main();  // Call the main function from the `sim` module

    flight_os::graphics::draw_centered_line();

    // Draw a slightly shorter horizontal line below the first one using the new draw_line() function.
    flight_os::graphics::draw_line((100, 110), (220, 110));

    flight_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    flight_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    flight_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    flight_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
