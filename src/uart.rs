pub const UARTO_BASE: usize = 0x09000000; // UART base address for the 'virt' machine

pub fn init() {
    // UART initialization code here
}

pub fn put_c(c: u8) {
    unsafe {
        let uart = UARTO_BASE as *mut u8;
        uart.write_volatile(c);
    }
}

pub fn put_s(s: &str) {
    for c in s.bytes() {
        put_c(c);
    }
}
