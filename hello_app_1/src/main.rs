#![no_std]
#![no_main]
use core::{panic::PanicInfo, arch::asm};

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;
static mut ABI_ENTRY: usize = 0;

#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start(_abi_entry: usize) -> ! {
    let arg0: u8 = b'C';
    // let arg1: u8 = b'!';

    hello();
    putchar(arg0);
    // putchar(arg1);
    puts("ArceOS v5678!");
    terminate(0);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn hello() {
    abi_call!(SYS_HELLO, 0);
}

fn putchar(c: u8) {
    abi_call!(SYS_PUTCHAR, c);
}

fn terminate(exit_code: i32) {
    abi_call!(SYS_TERMINATE, exit_code);
}


#[macro_export]
macro_rules! abi_call {
    ($abi_num: expr, $arg0: expr) => {{
        unsafe {
            asm!("
                addi sp, sp, -16*8
                sd ra, 8*15 (sp)
                sd t0, 8*14 (sp)
                sd t1, 8*13 (sp)
                sd t2, 8*12 (sp)
                sd t3, 8*11 (sp)
                sd t4, 8*10 (sp)
                sd t5, 8*9  (sp)
                sd t6, 8*8  (sp)
                sd a0, 8*7  (sp)
                sd a1, 8*6  (sp)
                sd a2, 8*5  (sp)
                sd a3, 8*4  (sp)
                sd a4, 8*3  (sp)
                sd a5, 8*2  (sp)
                sd a6, 8*1  (sp)
                sd a7, 8*0  (sp)

                li      t0, {abi_num}
                slli    t0, t0, 3     
                add     t1, a7, t0
                ld      t1, (t1)
                jalr    t1

                ld ra, 8*15 (sp)
                ld t0, 8*14 (sp)
                ld t1, 8*13 (sp)
                ld t2, 8*12 (sp)
                ld t3, 8*11 (sp)
                ld t4, 8*10 (sp)
                ld t5, 8*9  (sp)
                ld t6, 8*8  (sp)
                ld a0, 8*7  (sp)
                ld a1, 8*6  (sp)
                ld a2, 8*5  (sp)
                ld a3, 8*4  (sp)
                ld a4, 8*3  (sp)
                ld a5, 8*2  (sp)
                ld a6, 8*1  (sp)
                ld a7, 8*0  (sp)
                addi sp, sp, 16*8",
                abi_num = const $abi_num,
                in("a0") $arg0,
                clobber_abi("C"),
            )
        }
    }}
}

// fn puts(s: &str) {
//     s.chars().for_each(|c| {
//         putchar(c as u8)
//     });
// }

fn puts(s: &str) {
    for c in s.bytes() {
        putchar(c);
    }
}