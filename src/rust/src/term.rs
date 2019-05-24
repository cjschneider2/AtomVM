use std::intrinsics::transmute;

pub static TERM_BOXED_VALUE_TAG: u64 = 0x2;
pub static TERM_CATCH_TAG: u64 = 0x1B;
pub static TERM_BOXED_TAG_MASK: u64 = 0x3F;
pub static TERM_BOXED_TUPLE: u64 = 0x0;
pub static TERM_BOXED_REF: u64 = 0x10;
pub static TERM_BOXED_FUN: u64 = 0x14;
pub static TERM_ATOM_TAG: u64 = 0xB;
pub static TERM_INT_TAG: u64 = 0xF;
pub static TERM_PID_TAG: u64 = 0x3;
pub static TERM_BOXED_HEAP_BINARY: u64 = 0x24;
pub static BINARY_HEADER_SIZE: u64 = 2;

pub type Term = u64;

#[no_mangle]
pub extern "C" fn term_is_catch_label(term: Term) -> bool {
    term & 0x3F == TERM_CATCH_TAG
}


#[no_mangle]
pub extern "C" fn term_is_boxed(term: Term) -> bool {
    term & 0x3 == TERM_BOXED_VALUE_TAG
}

#[no_mangle]
pub extern "C" fn term_is_atom(term: Term) -> bool {
    term & 0x3F == TERM_ATOM_TAG
}

#[no_mangle]
pub extern "C" fn term_is_invalid_term(term: Term) -> bool {
    term == 0x0
}

#[no_mangle]
pub extern "C" fn term_is_nil(term: Term) -> bool {
    term & 0x3F == 0x3B
}

#[no_mangle]
pub extern "C" fn term_is_non_empty_list(term: Term) -> bool {
    term & 0x3 == 0x1
}


#[no_mangle]
pub extern "C" fn term_is_integer(term: Term) -> bool {
    term & 0xF == TERM_INT_TAG
}

#[no_mangle]
pub extern "C" fn term_is_pid(term: Term) -> bool {
    term & 0xF == TERM_PID_TAG
}

#[no_mangle]
pub extern "C" fn term_to_const_term_ptr(term: Term) -> *const Term {
    unsafe {
        transmute(term & !0x3)
    }
}

#[no_mangle]
pub extern "C" fn term_to_term_ptr(term: Term) -> *mut Term {
    unsafe {
        transmute(term & !0x3)
    }
}

#[no_mangle]
pub extern "C" fn term_is_list(term: Term) -> bool {
    term_is_non_empty_list(term) || term_is_nil(term)
}

#[no_mangle]
pub extern "C" fn term_is_movable_boxed(term: Term) -> bool {
    if term_is_boxed(term) {
        let value = unsafe { *(term_to_const_term_ptr(term)) };
        match value & 0x3F {
            0x10 => return true,
            _ => return false,
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn term_get_size_from_boxed_header(term: Term) -> u64 {
    term >> 6
}

#[no_mangle]
pub extern "C" fn term_is_reference(term: Term) -> bool {
    if term_is_boxed(term) {
        let value = unsafe { *(term_to_const_term_ptr(term)) };
        if value & 0x3F == TERM_BOXED_REF {
            return true;
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn term_boxed_size(term: Term) -> u64 {
    if term_is_boxed(term) {
        let value = unsafe { *(term_to_const_term_ptr(term)) };
        return term_get_size_from_boxed_header(term);
    }
    panic!();
}

#[no_mangle]
pub extern "C" fn term_is_binary(term: Term) -> bool {
    if term_is_boxed(term) {
        let value = unsafe { *(term_to_const_term_ptr(term)) };
        if value & 0x3F == 0x24 {
            return true;
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn term_is_tuple(term: Term) -> bool {
    if term_is_boxed(term) {
        let value = unsafe { *(term_to_const_term_ptr(term)) };
        if value & 0x3F == 0 {
            return true;
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn term_is_function(term: Term) -> bool {
    if term_is_boxed(term) {
        let value = unsafe { *(term_to_const_term_ptr(term)) };
        if value & 0x3F == TERM_BOXED_FUN {
            return true;
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn term_is_cp(term: Term) -> bool {
    term & 0x3 == 0
}

#[no_mangle]
pub extern "C" fn term_invalid_term() -> Term {
    0
}

#[no_mangle]
pub extern "C" fn term_nil() -> Term {
    0x3B
}

#[no_mangle]
pub extern "C" fn term_to_atom_index(term: Term) -> Term {
    term >> 6
}

#[no_mangle]
pub extern "C" fn term_from_atom_index(term: Term) -> Term {
    term << 6 | 0xB
}

#[no_mangle]
pub extern "C" fn term_to_int32(term: Term) -> i32 {
    match term & 0xF {
        0xF => {
            let val = term >> 4;
            if val > std::i32::MAX as u64 {
                eprintln!("warning: value larger than datatype, result truncated");
            }
            return val as i32;
        }
        _ => {
            eprintln!("Term is not an integer: {}", term);
            return 0;
        }
    }
}

#[no_mangle]
pub extern "C" fn term_to_catch_label_and_module(term: Term,
                                                 module_idx: &mut u64,
) -> u64 {
    *module_idx = term >> 24;
    term >> 6 & 0xFFFF
}

#[no_mangle]
pub extern "C" fn term_to_local_process_id(term: Term) -> u64 {
    match term & 0xF {
        0x3 => term >> 4,
        _ => {
            eprintln!("Term is not a pid: {}", term);
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn term_from_local_process_id(
    local_process_id: u64
) -> Term {
    (local_process_id << 4) | 0x3
}

#[no_mangle]
pub extern "C" fn term_from_int4(value: u8) -> Term {
    (value as u64) << 4 | 0xF
}

#[no_mangle]
pub extern "C" fn term_from_int11(value: u16) -> Term {
    (value as u64) << 4 | 0xF
}

#[no_mangle]
pub extern "C" fn term_from_int32(value: i32) -> Term {
    (value as u64) << 4 | 0xF
}

#[no_mangle]
pub extern "C" fn term_from_int64(value: i64) -> Term {
    let max = 0x0FFFFFFFFFFFFFFFi64;
    if value > max || value < -max {
        panic!("Value too large, should be put on heap");
    }
    (value as u64) << 4 | 0xF
}

#[no_mangle]
pub extern "C" fn term_from_catch_label(module_index: u64,
                                        label: u64,
) -> Term {
    module_index << 24 | label << 6 | TERM_CATCH_TAG
}

#[no_mangle]
pub extern "C" fn term_from_catch_local_process_id(local_pid: u64) -> Term {
    local_pid << 4 | 0x3
}

#[no_mangle]
pub extern "C" fn term_binary_data_size_in_terms(size: u64) -> Term {
    ((size + 8 - 1) >> 3) + 1
}

#[no_mangle]
pub extern "C" fn term_binary_size(term: Term) -> Term {
    if term_is_boxed(term) {
        unsafe { *(term_to_const_term_ptr(term)) }
    } else {
        panic!("Trying to unbox invalid pointer");
    }
}

#[no_mangle]
pub extern "C" fn term_binary_data(term: Term) -> *const Term {
    if term_is_boxed(term) {
        unsafe { term_to_const_term_ptr(term + 2) }
    } else {
        panic!("Trying to unbox invalid pointer");
    }
}

#[no_mangle]
pub extern "C" fn term_to_ref_ticks(term: Term) -> u64 {
    if term_is_boxed(term) {
        unsafe { *(term_to_const_term_ptr(term)) }
    } else {
        panic!("Trying to unbox invalid pointer");
    }
}

#[no_mangle]
pub extern "C" fn term_put_tuple_element(
    term: Term,
    element_idx: u32,
    put_val: Term,
) {
    if term_is_boxed(term) {
        unsafe {
            let value = *(term_to_term_ptr(term));
            if (value & TERM_BOXED_VALUE_TAG) == 0
            && element_idx < (value >> 6) as u32 {
                *(term_to_term_ptr(term)) = put_val;
            }
        }
    } else {
        panic!("Trying to unbox invalid pointer");
    }
}

// Requires Context
/*
#[no_mangle]
pub extern "C" fn term_from_literal_binary(
    data: *const u8,
    size: u32,
    context: &Context
) -> Term {
    let size_in_terms = term_binary_data_size_in_terms(size as u64);
    let value = memory_heap_alloc(context, size_in_terms + 1);
    value[0] = size_in_terms << 6 | 0x24; // heap binary?
    value[1] = size;

}
*/

