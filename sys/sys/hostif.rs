mod console;

pub const HOSTIF_ABI_VERSION: u32 = 1;

pub trait HostIf {
    const CONSOLES: &'static [console::ConsDesc];

    fn abi_version() -> u32;
    fn serial_write(port: u32, buf: &[u8]);
}
