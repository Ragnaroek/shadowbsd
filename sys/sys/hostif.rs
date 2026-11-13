pub const HOSTIF_ABI_VERSION: u32 = 1;

pub trait HostIf {
    fn abi_version() -> u32;
}
