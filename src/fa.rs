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

//! Rust foreign-function interface wrapper of the [Augeas libfa Finite Automata library](http://augeas.net/libfa/index.html).

#![crate_id = "fa#0.1.0"]
#![crate_type = "lib"]

extern crate libc;

use libc::{c_char, size_t};
use std::c_str::{ToCStr};
use std::option::{Option};
use std::mem;
use std::path::{Path};
use std::ptr;
use std::raw::{Slice};
use std::str;

mod ffi;

/// Represents a finite automaton.
pub struct Fa {
    fa: *mut ffi::Fa,
}

impl Fa {

    /// Compiles the given regular expression into a finite automaton. The syntax for regular
    /// expressions is extended POSIX syntax, with some exceptions:
    ///
    ///  1. `'.'` does not match newlines.
    ///  2. The start-of-line and end-of-line anchors (`^` and `$`) are not supported.
    pub fn compile(re: &str) -> Option<Fa> {
        unsafe {
            re.with_c_str_unchecked(|re_c_str| -> Option<Fa> {
                let mut fa: *mut ffi::Fa = ptr::mut_null();
                if ffi::fa_compile(re_c_str, re.len() as size_t, &mut fa as *mut *mut ffi::Fa) == 0 {
                    assert!(fa.is_not_null());
                    Some(Fa {
                        fa: fa
                    })
                } else {
                    None
                }
            })
        }
    }

    /// Minimizes this finite automaton in place using Hopcroft's state minimization algorithm.
    pub fn minimize(&mut self) {
        unsafe {
            ffi::fa_minimize(self.fa);
        }
    }

    /// Converts this finite automaton into a regular expression. When the regular expression
    /// is compiled to an automaton, it is guaranteed that that automaton and this automaton
    /// accept the same language.
    ///
    /// The code tries to be semi-clever about keeping the generated regular expression short.
    /// To guarantee reasonably short regexps, this automaton should be [minimized](#method.minimize)
    /// before `as_regexp()` is called.
    ///
    /// # Return value
    /// `None` if an error occurred (out of memory). Otherwise, the regular expression string
    /// is returned.
    pub fn as_regexp(&self) -> Option<String> {
        unsafe {
            let mut regexp: *c_char = ptr::null();
            let mut regexp_len: size_t = 0;
            if ffi::fa_as_regexp(self.fa, &mut regexp as *mut *c_char, &mut regexp_len as *mut size_t) == 0 {
                assert!(regexp.is_not_null());

                // Convert the libfa buffer to a slice, and then to a String.
                // See CString::as_bytes()
                // http://doc.rust-lang.org/src/rustrt/home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/librustrt/c_str.rs.html
                let bytes: &[u8] = mem::transmute(Slice {
                        data: regexp,
                        len: regexp_len as uint,
                    });
                let opt_re = match str::from_utf8(bytes) {
                        None => None,
                        Some(re) => Some(re.to_string()),
                    };

                // Free the buffer allocated by libfa before returning.
                // See CString::drop()
                libc::free(regexp as *mut libc::c_void);

                opt_re
            } else {
                None
            }
        }
    }

    /// Tests whether the language of this finite automaton contains the language of `other`.
    pub fn contains(&self, other: &Fa) -> bool {
        unsafe {
            ffi::fa_contains(other.fa, self.fa) == 1
        }
    }

    /// Returns a finite automaton that accepts the union of the languages that this `Fa` and
    /// `other` accept (the `'|'` operator in regular expressions).
    pub fn union(&self, other: &Fa) -> Fa {
        unsafe {
            Fa {
                fa: ffi::fa_union(self.fa, other.fa)
            }
        }
    }

    /// Creates a file at `path` (overwriting if a file already exists) containing this finite
    /// automaton in [Graphviz](http://graphviz.org) DOT format.
    ///
    /// Note: This method internally uses `fopen()` from the C standard library. This may fail
    /// and the file might fail to be created.
    pub fn make_dot_file(&self, path: &Path) {
        path.with_c_str(|path| unsafe {
            let out = libc::fopen(path, "w+b\0".as_bytes().as_ptr() as *c_char);
            if out.is_not_null() {
                ffi::fa_dot(out as *mut libc::FILE, self.fa);
                libc::fclose(out);
            }
        });
    }
}

impl Drop for Fa {
    fn drop(&mut self) {
        unsafe {
            assert!(self.fa.is_not_null());
            ffi::fa_free(self.fa);
            self.fa = ptr::mut_null();
        }
    }
}

impl PartialEq for Fa {
    fn eq(&self, other: &Fa) -> bool {
        unsafe {
            ffi::fa_equals(self.fa, other.fa) == 1
        }
    }
}
