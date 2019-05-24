
const AVMPACK_SIZE: usize = 24;

#[no_mangle]
pub extern "C" fn pad(size: usize) -> usize{
    ((size + 4 - 1) >> 2) << 2
}
