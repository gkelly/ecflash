use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Mutex;

lazy_static! {
    static ref FILE: Mutex<File> = Mutex::new(
        OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/port")
            .expect("failed to open /dev/port")
        );
}

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let mut buf = [0];

    let mut file = FILE.lock().unwrap();
    file.seek(SeekFrom::Start(port as u64)).unwrap();
    file.read(&mut buf).unwrap();

    buf[0]
}

#[inline(always)]
pub unsafe fn outb(port: u16, value: u8) {
    let mut file = FILE.lock().unwrap();
    file.seek(SeekFrom::Start(port as u64)).unwrap();
    file.write(&[value]).unwrap();
}
