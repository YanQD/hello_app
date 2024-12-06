#![no_std]
#![no_main]
use core::{panic::PanicInfo, arch::asm};

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;
static mut ABI_ENTRY: usize = 0;

#[no_mangle]
// #[link_section = ".text.entry"]
unsafe extern "C" fn _start(abi_entry: usize) -> ! {
    ABI_ENTRY = abi_entry;
    let arg0: u8 = b'C';
    // let arg1: u8 = b'!';

    hello();
    putchar(arg0);
    // putchar(arg1);
    sputs("ArceOS v5678!");
    // puts("ArceOS v5678!");
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
        unsafe {asm!("
            li      a0, {abi_num}
            slli    t0, t0, 3
            la      t1, {abi_entry}
            ld      t1, (t1)
            jalr    t1",
            abi_entry = sym ABI_ENTRY,
            abi_num = const $abi_num,
            in("a1") $arg0,
            clobber_abi("C"),
        )}
    }}
}

fn sputs(s: &str) {
    s.chars().for_each(|c| {
        putchar(c as u8)
    });
}

fn puts(s: &str) {
    for c in s.bytes() {
        putchar(c);
    }
}