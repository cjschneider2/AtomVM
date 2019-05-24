#ifndef ATOMVM_LIBRUST_H
#define ATOMVM_LIBRUST_H

/* Generated with cbindgen:0.8.7 */

/* Warning this is an automatically generated file! */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

typedef struct HashSet_String HashSet_String;

typedef struct String String;

typedef const String *AtomString;

typedef HashSet_String AtomHashTable;

typedef struct {
    FILE* fd;
    void *mapped;
    uint64_t size;
} MappedFile;

typedef uint64_t Term;

bool atom_are_equals(AtomString a, AtomString b);

const uint8_t *atom_string_data(AtomString atom);

uintptr_t atom_string_len(AtomString atom);

void atom_string_to_c(AtomString atom_string, uint8_t *buf_ptr, size_t buf_size);

void atom_write_mfa(uint8_t *buf_ptr,
                    size_t buf_size,
                    AtomString module,
                    AtomString function,
                    uint32_t arity);

uintptr_t atomshashtable_get_count(const AtomHashTable *hash_table);

// NOTE: I have no idea why this function is here... value is never used
uint32_t atomshashtable_get_value(AtomHashTable *hash_table,
                                  const AtomString *atom,
                                  uint32_t default_value);

bool atomshashtable_has_key(const AtomHashTable *hash_table, const AtomString *atom);

bool atomshashtable_insert(AtomHashTable *hash_table, AtomString string, uint32_t value);

AtomHashTable *atomshashtable_new(void);

void mapped_file_close(const MappedFile *mf);

MappedFile *mapped_file_open_beam(const char *file_name);

uintptr_t pad(uintptr_t size);

const Term *term_binary_data(Term term);

Term term_binary_data_size_in_terms(uint64_t size);

Term term_binary_size(Term term);

uint64_t term_boxed_size(Term term);

Term term_from_atom_index(Term term);

Term term_from_catch_label(uint64_t module_index, uint64_t label);

Term term_from_catch_local_process_id(uint64_t local_pid);

Term term_from_int11(uint16_t value);

Term term_from_int32(int32_t value);

Term term_from_int4(uint8_t value);

Term term_from_int64(int64_t value);

Term term_from_local_process_id(uint64_t local_process_id);

uint64_t term_get_size_from_boxed_header(Term term);

Term term_invalid_term(void);

bool term_is_atom(Term term);

bool term_is_binary(Term term);

bool term_is_boxed(Term term);

bool term_is_catch_label(Term term);

bool term_is_cp(Term term);

bool term_is_function(Term term);

bool term_is_integer(Term term);

bool term_is_invalid_term(Term term);

bool term_is_list(Term term);

bool term_is_movable_boxed(Term term);

bool term_is_nil(Term term);

bool term_is_non_empty_list(Term term);

bool term_is_pid(Term term);

bool term_is_reference(Term term);

bool term_is_tuple(Term term);

Term term_nil(void);

void term_put_tuple_element(Term term, uint32_t element_idx, Term put_val);

Term term_to_atom_index(Term term);

uint64_t term_to_catch_label_and_module(Term term, uint64_t *module_idx);

const Term *term_to_const_term_ptr(Term term);

int32_t term_to_int32(Term term);

uint64_t term_to_local_process_id(Term term);

uint64_t term_to_ref_ticks(Term term);

Term *term_to_term_ptr(Term term);

#endif /* ATOMVM_LIBRUST_H */
