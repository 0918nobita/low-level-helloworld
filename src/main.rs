#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() {
    let msg = "Hello, World!\n";
    let slice = msg.as_bytes();
    let ptr = slice as *const [u8] as *const u8;
    unsafe {
      kaku(1, ptr, 14);
      owaru(0);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#[link(name="syscall", kind="static")]
extern "C" {
  fn kaku(fd: i64, ptr: *const u8, len: usize);
  fn owaru(fd: i64);
}
