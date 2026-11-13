pub struct ConsDesc {
    pub name: &'static str,
    pub flags: u32,
    pub write: fn(&[u8]) -> usize,
    pub read: fn(&mut [u8]) -> usize,
}

pub trait Console {
    fn write(buf: &[u8]) -> usize;
    fn read(buf: &mut [u8]) -> usize;
}

impl ConsDesc {
    pub const fn of<C: Console>(name: &'static str, flags: u32) -> ConsDesc {
        ConsDesc {
            name,
            flags,
            write: C::write,
            read: C::read,
        }
    }
}
