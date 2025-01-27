#![no_std]
#![feature(start)]

use core::ptr::addr_of;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    let data_addr = ( unsafe {addr_of!(INPUT_BUFFER)}  as *const _ as u16).to_be_bytes();
    let sub_addr = (process_input_pair as *const () as u16).to_be_bytes();
    let result_addr = (unsafe {addr_of!(TOTAL)} as *const _ as u16).to_be_bytes();
    // Doesn't fit
    //let display_result_addr = (display_result as *const () as u16).to_be_bytes();
    
    unsafe {
        delay();
        while !ak() {
            scandisplay(data_addr [0], data_addr[1], 1); 
        }
        delay();
        while !ak() {
            scandisplay(sub_addr[0], sub_addr[1], 2); 
        }
        delay();
        while !ak() {
            scandisplay(result_addr[0], result_addr[1], 3); 
        }

        // Doesn't fit
        //scandisplay(display_result_addr[0], display_result_addr[1], 4); }
        //delay();
    }


    unsafe { TOTAL = 0; }
    unsafe { INPUT_BUFFER [0] = 0; }
    unsafe { INPUT_BUFFER [1] = 0; }
    process_input_pair(); // Don't compile me out dummy!
    unsafe { start() };

    0
}

extern "C" {
    fn scandisplay(a: u8, b: u8, c: u8);
    fn delay();
    fn start();
    fn getkey() -> u8;
    fn getchar2() -> u8;
    fn ak() -> bool;
} 

#[link_section=".zp.bss"]
static mut INPUT_BUFFER: [u32; 2] = [0u32; 2];
#[link_section=".zp.bss"]
static mut TOTAL: u64 = 0;

#[inline(never)]
extern fn process_input_pair() {
   unsafe { TOTAL += u32::from_be(INPUT_BUFFER[0]).abs_diff(u32::from_be(INPUT_BUFFER[1])) as u64 }; 
   unsafe { start(); }
}

// Doesn't fit
//#[inline(never)]
//extern fn display_result() {
    //let result_bytes = unsafe { TOTAL }.to_be_bytes();
    ////for (i, word) in result_bytes.windows(2).enumerate()
    //for i in 0..4
    //{
        //unsafe { scandisplay(result_bytes[i], result_bytes[i+1], i as u8) };
        //delay();
    //}
//}
