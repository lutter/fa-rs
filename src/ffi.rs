// Copyright (C) 2014  Daniel Trebbien
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 3 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.

use libc::{FILE, c_char, c_int, size_t};

pub struct Fa;

#[link(name = "fa")]
extern {
    pub fn fa_compile(re: *const c_char, size: size_t, fa: *mut *mut Fa) -> c_int;
    pub fn fa_minimize(fa: *mut Fa) -> c_int;
    pub fn fa_as_regexp(fa: *mut Fa, regexp: *mut *const c_char, regexp_len: *mut size_t) -> c_int;
    pub fn fa_union(fa1: *mut Fa, fa2: *mut Fa) -> *mut Fa;
    pub fn fa_contains(fa1: *mut Fa, fa2: *mut Fa) -> c_int;
    pub fn fa_equals(fa1: *mut Fa, fa2: *mut Fa) -> c_int;
    pub fn fa_free(fa: *mut Fa);
    pub fn fa_dot(out: *mut FILE, fa: *mut Fa);
}
