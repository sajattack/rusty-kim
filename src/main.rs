#![no_std]
#![feature(start)]
#![allow(unused_imports)]

use ufmt_stdio::*;
use core::panic::PanicInfo;
use core::convert::Infallible;

//#[panic_handler]
//fn panic(_: &PanicInfo) -> ! {
    //if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        //println!("panic occurred: {}", s);
    //} else {
        //println!("panic occurred");
    //}
    //loop {}
//}

use panic_halt as _;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {

    let seg1: u8 = 0x13;
    let seg2: u8 = 0x37;
    let mut last_keypress = 0x00;
    let mut cur_keypress;

    // serial output
    println!("HACK THE PLANET!!!");

    loop {

        // 7seg output, keypad input
        unsafe {
             cur_keypress = getkey();
        }
            
        if cur_keypress == 0x15 {
            unsafe {
                scandisplay2(seg1, seg2, last_keypress);
            }
        }
        else {
            unsafe {
                scandisplay2(seg1, seg2, cur_keypress);
            }
            last_keypress = cur_keypress;
        }

        // ascii to hex on serial
        //let c: u8 = unsafe { getchar2() };
        //println!(" {:x}", c);
    }
}

extern "C" {
    fn scandisplay2(a: u8, b: u8, c: u8);
    fn getkey() -> u8;
} 
