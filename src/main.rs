#![no_std]
#![feature(start)]

use core::ptr::addr_of_mut;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {

    let data_addr = unsafe { core::mem::transmute::<u16, [u8;2]>({ addr_of_mut!(INPUT_BUFFER) }  as *const _ as u16) };
    let sub_addr = unsafe { core::mem::transmute::<u16, [u8;2]>(process_input_pair as *const () as u16) };
    let result_addr = unsafe { core::mem::transmute::<u16, [u8;2]>( addr_of_mut!( TOTAL ) as *const _ as u16)};
    
    unsafe { scandisplay(data_addr [1], data_addr[0], 1); }
    delay();
    
    unsafe { scandisplay(sub_addr[1], sub_addr[0], 2); }
    delay();
    
    unsafe { scandisplay(result_addr[1], result_addr[0], 3); }
    delay();

    unsafe { TOTAL = 0; }
    unsafe { INPUT_BUFFER [0] = 0; }
    unsafe { INPUT_BUFFER [1] = 0; }
    unsafe { brk() };
    0
}

extern "C" {
    fn scandisplay(a: u8, b: u8, c: u8);
    fn nop();
    fn brk();
} 

static mut INPUT_BUFFER: [u32; 2] = [0u32; 2];
static mut TOTAL: u64 = 0;

#[inline(never)]
extern fn process_input_pair() {
   unsafe { TOTAL += INPUT_BUFFER[0].abs_diff(INPUT_BUFFER[1]) as u64 }; 
}

#[inline(never)]
fn delay() {
    for _ in 0..u8::MAX {
            core::hint::spin_loop();
            unsafe { nop() };
    }
}
