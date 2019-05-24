use libc;
use std::ffi::CStr;
use std::collections::HashSet;

type AtomString = &'static String;
type AtomHashTable = HashSet<String>;

#[no_mangle]
pub extern "C" fn atom_string_to_c(atom_string: AtomString,
                                   buf_ptr: *mut u8,
                                   buf_size: libc::size_t)
{
    let mut buffer = unsafe {
        std::slice::from_raw_parts_mut(buf_ptr, buf_size)
    };

    buffer.copy_from_slice(atom_string.as_bytes());
}

#[no_mangle]
pub extern "C" fn atom_are_equals(a: AtomString, b: AtomString) -> bool {
    a == b
}


#[no_mangle]
pub extern "C" fn atom_write_mfa(buf_ptr: *mut u8,
                                 buf_size: libc::size_t,
                                 module: AtomString,
                                 function: AtomString,
                                 arity: u32)
{
    let mut buffer = unsafe {
        std::slice::from_raw_parts_mut(buf_ptr, buf_size)
    };

    let mod_fn_arity_str = format!("{}:{}/{}", module, function, arity);

    buffer.copy_from_slice(mod_fn_arity_str.as_bytes())
}

#[no_mangle]
pub extern "C" fn atom_string_len(atom: AtomString) -> usize {
    atom.len()
}


#[no_mangle]
pub extern "C" fn atom_string_data(atom: AtomString) -> *const u8 {
    atom.as_ptr()
}


#[no_mangle]
pub extern "C" fn atomshashtable_new() -> *mut AtomHashTable {
    let mut p_set = Box::new(HashSet::new());
    Box::into_raw(p_set)
}

#[no_mangle]
pub extern "C" fn atomshashtable_insert(hash_table: &mut AtomHashTable,
                                        string: AtomString,
                                        value: u32,
) -> bool {
    hash_table.insert(string.clone())
}

/// NOTE: I have no idea why this function is here... value is never used
#[no_mangle]
pub extern "C" fn atomshashtable_get_value(hash_table: &mut AtomHashTable,
                                           atom: &AtomString,
                                           default_value: u32,
) -> u32 {
    return default_value;
}

#[no_mangle]
pub extern "C" fn atomshashtable_has_key(hash_table: &AtomHashTable,
                                         atom: &AtomString,
) -> bool {
    hash_table.contains(atom.clone())
}

#[no_mangle]
pub extern "C" fn atomshashtable_get_count(hash_table: &AtomHashTable) -> usize {
    hash_table.len()
}
