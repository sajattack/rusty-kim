#![no_std]
#![feature(start)]

use ufmt_stdio::{ufmt, println};
use core::ptr::addr_of_mut;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {

    // This string formatting blows too much RAM
    //println!("Enter the 32-bit input pairs as 8 LE bytes at 0x{:04x}" & unsafe { INPUT_BUFFER } as *const _ as u16);
    //println!("Then set PC to 0x{:04x} and press GO", &process_input_pair as *const _ as u16);
    //println!("Repeat until all 2000 numbers are entered :D");
    //println!("Then set PC to 0x{:04x} and press GO to view the result", &display_result as *const _ as u16);
    //println!("Result is displayed as  16-bit hex numbers on the 7seg display, separated by blinks");

    let input_addr = unsafe { core::mem::transmute::<u16, [u8;2]>({ addr_of_mut!(INPUT_BUFFER) }  as *const _ as u16) };
    let sub_addr = unsafe { core::mem::transmute::<u16, [u8;2]>(process_input_pair as *const () as u16) };
    let disp_addr = unsafe { core::mem::transmute::<u16, [u8;2]>(display_result as *const () as u16) };
    
    unsafe { scandisplay(input_addr[1], input_addr[0], 1); }
    delay();
    //unsafe { cleardisplay(); }
    //delay();
    
    unsafe { scandisplay(sub_addr[1], sub_addr[0], 2); }
    delay();
    //unsafe { cleardisplay(); }
    //delay();
    
    unsafe { scandisplay(disp_addr[1], disp_addr[0], 3); }
    delay();
    
    0
}

extern "C" {
    fn scandisplay(a: u8, b: u8, c: u8);
    fn cleardisplay();
    fn nop();
} 

static mut INPUT_BUFFER: [u32; 2] = [0u32; 2];
static mut TOTAL: u64 = 0;

#[inline(never)]
extern fn process_input_pair() {
   unsafe { TOTAL += INPUT_BUFFER[0].abs_diff(INPUT_BUFFER[1]) as u64 }; 
}

#[inline(never)]
extern fn display_result() {
    let four16bit = unsafe { core::mem::transmute::<u64, [u16; 4]>( TOTAL ) };

    for i in 0..4u8 {
        let two8bit = unsafe { core::mem::transmute::<u16, [u8; 2]>(four16bit[i as usize]) };
        unsafe { 
            cleardisplay(); 
            scandisplay(two8bit[1], two8bit[0], i);
        }
        delay();
        unsafe {
            cleardisplay();
        }
    }
}

#[inline(never)]
fn delay() {
    for _ in 0..u16::MAX {
        core::hint::spin_loop();
        for _ in 0..255 {
            core::hint::spin_loop();
            unsafe { nop() };
        }
    }
}
