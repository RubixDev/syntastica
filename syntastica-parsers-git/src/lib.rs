#![doc = include_str!("../README.md")]
//!
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

syntastica_macros::parsers_ffi!();

/// Basic implementation of some libc functions that tree-sitter parsers can link to.
#[cfg(all(
    feature = "runtime-c2rust",
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown",
    target_env = ""
))]
mod wasm_c_bridge {
    use std::{ffi::CStr, mem::MaybeUninit};

    /// <https://en.cppreference.com/w/c/program/abort>
    #[no_mangle]
    extern "C" fn abort() {
        panic!("program aborted");
    }

    /// <https://en.cppreference.com/w/c/string/wide/towupper>
    #[no_mangle]
    extern "C" fn towupper(wc: u32) -> u32 {
        let Some(char) = char::from_u32(wc) else {
            return wc;
        };
        let mut uppercase = char.to_uppercase();
        if uppercase.len() == 1 {
            uppercase.next().unwrap() as u32
        } else {
            wc
        }
    }

    /// <https://en.cppreference.com/w/c/string/wide/towlower>
    #[no_mangle]
    extern "C" fn towlower(wc: u32) -> u32 {
        let Some(char) = char::from_u32(wc) else {
            return wc;
        };
        let mut uppercase = char.to_lowercase();
        if uppercase.len() == 1 {
            uppercase.next().unwrap() as u32
        } else {
            wc
        }
    }

    /// <https://en.cppreference.com/w/c/memory/malloc>
    #[no_mangle]
    extern "C" fn malloc(size: usize) -> *mut MaybeUninit<u8> {
        Box::leak(Box::<[u8]>::new_uninit_slice(size)).as_mut_ptr()
    }

    /// <https://en.cppreference.com/w/c/memory/calloc>
    #[no_mangle]
    extern "C" fn calloc(num: usize, size: usize) -> *mut u8 {
        let mut values = Box::<[u8]>::new_uninit_slice(size * num);
        unsafe {
            for i in 0..size * num {
                values[i].as_mut_ptr().write(0);
            }
            Box::leak(values.assume_init()).as_mut_ptr()
        }
    }

    /// <https://en.cppreference.com/w/c/memory/realloc>
    #[no_mangle]
    extern "C" fn realloc(ptr: *mut u8, new_size: usize) -> *mut MaybeUninit<u8> {
        free(ptr);
        Box::leak(Box::<[u8]>::new_uninit_slice(new_size)).as_mut_ptr()
    }

    /// <https://en.cppreference.com/w/c/memory/free>
    #[no_mangle]
    extern "C" fn free(_ptr: *mut u8) {
        // surely a bit of memory leakage isn't _that_ bad... :)
    }

    #[no_mangle]
    extern "C" fn __assert2(file: *const i8, line: i32, func: *const i8, error: *const i8) {
        let file = unsafe { CStr::from_ptr(file) }.to_string_lossy();
        let func = unsafe { CStr::from_ptr(func) }.to_string_lossy();
        let error = unsafe { CStr::from_ptr(error) }.to_string_lossy();
        panic!("assertion failed in {file} on line {line} in {func}: {error}");
    }

    /// <https://en.cppreference.com/w/c/string/byte/strcmp>
    #[no_mangle]
    extern "C" fn strcmp(lhs: *const i8, rhs: *const i8) -> i32 {
        let lhs = unsafe { CStr::from_ptr(lhs) };
        let rhs = unsafe { CStr::from_ptr(rhs) };
        lhs.cmp(rhs) as i32
    }

    /// <https://en.cppreference.com/w/c/string/byte/strncpy>
    #[no_mangle]
    extern "C" fn strncpy(dest: *mut i8, src: *const i8, count: usize) -> *mut i8 {
        for i in 0..count {
            let cp = unsafe { src.add(i).read() };
            unsafe { dest.add(i).write(cp) }
        }
        dest
    }

    /// <https://en.cppreference.com/w/c/string/byte/memchr>
    #[no_mangle]
    extern "C" fn memchr(ptr: *const u8, ch: i32, count: usize) -> *mut u8 {
        let ch = ch as u8;
        for i in 0..count {
            if unsafe { ptr.add(i).read() } == ch {
                return unsafe { ptr.add(i) as *mut _ };
            }
        }
        std::ptr::null_mut()
    }
}
