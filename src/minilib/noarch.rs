use crate::minilib::sys_write;

pub unsafe fn strlen(mut s: *const u8) -> usize {
    let mut count = 0;
    while *s != b'\0' {
        count += 1;
        s = s.add(1);
    }
    count
}

pub fn print_str(s: &[u8]) {
    unsafe {
        sys_write(s.as_ptr(), s.len());
    }
}
