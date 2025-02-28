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
#[allow(non_camel_case_types)]
mod wasm_c_bridge {
    use std::{
        ffi::{c_void, CStr},
        mem::MaybeUninit,
    };

    type wint_t = u32;
    type size_t = usize;
    type c_char = i8;
    type int = i32;

    /// <https://en.cppreference.com/w/c/program/abort>
    #[no_mangle]
    extern "C" fn abort() {
        panic!("program aborted");
    }

    /// <https://en.cppreference.com/w/c/string/wide/towupper>
    #[no_mangle]
    extern "C" fn towupper(wc: wint_t) -> wint_t {
        let Some(char) = char::from_u32(wc) else {
            return wc;
        };
        let mut uppercase = char.to_uppercase();
        if uppercase.len() == 1 {
            uppercase.next().unwrap() as wint_t
        } else {
            wc
        }
    }

    /// <https://en.cppreference.com/w/c/string/wide/towlower>
    #[no_mangle]
    extern "C" fn towlower(wc: wint_t) -> wint_t {
        let Some(char) = char::from_u32(wc) else {
            return wc;
        };
        let mut uppercase = char.to_lowercase();
        if uppercase.len() == 1 {
            uppercase.next().unwrap() as wint_t
        } else {
            wc
        }
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswalnum>
    #[no_mangle]
    extern "C" fn iswalnum(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_alphanumeric()) as int
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswalpha>
    #[no_mangle]
    extern "C" fn iswalpha(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_alphabetic()) as int
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswblank>
    #[no_mangle]
    extern "C" fn iswblank(ch: wint_t) -> int {
        (ch == b' ' as wint_t || ch == b'\t' as wint_t) as int
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswcntrl>
    #[no_mangle]
    extern "C" fn iswcntrl(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_control()) as int
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswdigit>
    #[no_mangle]
    extern "C" fn iswdigit(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_numeric()) as int
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswgraph>
    #[no_mangle]
    extern "C" fn iswgraph(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_ascii_graphic()) as int
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswlower>
    #[no_mangle]
    extern "C" fn iswlower(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_lowercase()) as int
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswprint>
    #[no_mangle]
    extern "C" fn iswprint(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_ascii_graphic() || ch == ' ') as int
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswpunct>
    #[no_mangle]
    extern "C" fn iswpunct(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_ascii_punctuation()) as int
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswspace>
    #[no_mangle]
    extern "C" fn iswspace(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_whitespace()) as int
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswupper>
    #[no_mangle]
    extern "C" fn iswupper(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_uppercase()) as int
    }

    /// <https://en.cppreference.com/w/c/string/wide/iswxdigit>
    #[no_mangle]
    extern "C" fn iswxdigit(ch: wint_t) -> int {
        char::from_u32(ch).is_some_and(|ch| ch.is_ascii_hexdigit()) as int
    }

    /// <https://en.cppreference.com/w/c/memory/malloc>
    #[no_mangle]
    extern "C" fn malloc(size: size_t) -> *mut MaybeUninit<u8> {
        Box::leak(Box::<[u8]>::new_uninit_slice(size)).as_mut_ptr()
    }

    /// <https://en.cppreference.com/w/c/memory/calloc>
    #[no_mangle]
    extern "C" fn calloc(num: size_t, size: size_t) -> *mut c_void {
        let mut values = Box::<[u8]>::new_uninit_slice(size * num);
        unsafe {
            for i in 0..size * num {
                values[i].as_mut_ptr().write(0);
            }
            Box::leak(values.assume_init()).as_mut_ptr() as *mut _
        }
    }

    /// <https://en.cppreference.com/w/c/memory/realloc>
    #[no_mangle]
    extern "C" fn realloc(ptr: *mut c_void, new_size: size_t) -> *mut c_void {
        free(ptr);
        Box::leak(Box::<[u8]>::new_uninit_slice(new_size)).as_mut_ptr() as *mut _
    }

    /// <https://en.cppreference.com/w/c/memory/free>
    #[no_mangle]
    extern "C" fn free(_ptr: *mut c_void) {
        // surely a bit of memory leakage isn't _that_ bad... :)
        // TODO: perhaps improve with sth like this:
        //  <https://github.com/ezrosent/allocators-rs/blob/master/malloc-bind/src/lib.rs>
    }

    #[no_mangle]
    extern "C" fn __assert2(
        file: *const c_char,
        line: int,
        func: *const c_char,
        error: *const c_char,
    ) {
        let file = unsafe { CStr::from_ptr(file) }.to_string_lossy();
        let func = unsafe { CStr::from_ptr(func) }.to_string_lossy();
        let error = unsafe { CStr::from_ptr(error) }.to_string_lossy();
        panic!("assertion failed in {file} on line {line} in {func}: {error}");
    }

    /// <https://en.cppreference.com/w/c/string/byte/strcmp>
    #[no_mangle]
    extern "C" fn strcmp(lhs: *const c_char, rhs: *const c_char) -> int {
        let lhs = unsafe { CStr::from_ptr(lhs) };
        let rhs = unsafe { CStr::from_ptr(rhs) };
        lhs.cmp(rhs) as int
    }

    /// <https://en.cppreference.com/w/c/string/byte/strncpy>
    #[no_mangle]
    extern "C" fn strncpy(dest: *mut c_char, src: *const c_char, count: size_t) -> *mut c_char {
        for i in 0..count {
            let cp = unsafe { src.add(i).read() };
            unsafe { dest.add(i).write(cp) }
        }
        dest
    }

    /// <https://en.cppreference.com/w/c/string/byte/memchr>
    #[no_mangle]
    extern "C" fn memchr(ptr: *const c_void, ch: int, count: size_t) -> *mut c_void {
        let ptr = ptr as *const u8;
        let ch = ch as u8;
        for i in 0..count {
            if unsafe { ptr.add(i).read() } == ch {
                return unsafe { ptr.add(i) as *mut _ };
            }
        }
        std::ptr::null_mut()
    }
}
