//! The wickra-genome C ABI — the hub every C-capable language links against.
//!
//! The surface is tiny and JSON-shaped, exactly like [`genome_core::Genome`]:
//! construct a handle from a spec JSON, drive it with command JSONs (`set_spec`,
//! `feed`, `build`, `vector`, `similar`, `cluster`, `anomaly`, `version`), read
//! back response JSONs, and free the handle. No genome type crosses the boundary
//! by value — the handle is opaque and the payloads are always UTF-8 JSON strings.
//!
//! Responses use a caller-owned buffer with a length-out protocol (the classic
//! C two-call idiom): call with `out = NULL`, `cap = 0` to learn the length
//! `len`, then allocate `len + 1` and call again. When `len < cap` the response
//! is written immediately. Negative returns are reserved for unusable arguments
//! ([`WICKRA_GENOME_ERR_NULL`], [`WICKRA_GENOME_ERR_UTF8`]) and caught panics
//! ([`WICKRA_GENOME_ERR_PANIC`]); a non-negative return is always the response
//! length. Domain errors come back in-band as `{"ok":false,"error":...}` JSON.

use core::ffi::{c_char, CStr};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

use genome_core::Genome;

/// A required pointer argument (`handle` or `cmd_json`) was null.
pub const WICKRA_GENOME_ERR_NULL: i32 = -1;
/// `cmd_json` was not valid UTF-8.
pub const WICKRA_GENOME_ERR_UTF8: i32 = -2;
/// A panic was caught at the FFI boundary.
pub const WICKRA_GENOME_ERR_PANIC: i32 = -3;

/// An opaque handle to a genome instance. Created by [`wickra_genome_new`] and
/// destroyed by [`wickra_genome_free`]; never dereferenced by the caller.
///
/// The handle caches the most recent command's response in `pending` so the
/// two-call length protocol does not run the (potentially expensive) query
/// twice. The cache is keyed on the raw command bytes and cleared once the
/// response has been delivered.
pub struct WickraGenome {
    inner: Genome,
    pending: Option<(Vec<u8>, String)>,
}

/// Read a NUL-terminated C string as `&str`, or `None` on null / bad UTF-8.
///
/// # Safety
/// `ptr` must be null or a valid NUL-terminated C string.
unsafe fn opt_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(ptr) }.to_str().ok()
}

/// Construct a genome handle from a spec JSON. Returns null on a null /
/// non-UTF-8 / invalid spec (the spec must carry a non-empty `features` and
/// `symbols`). Free it with [`wickra_genome_free`].
///
/// # Safety
/// `spec_json` must be null or a valid NUL-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn wickra_genome_new(spec_json: *const c_char) -> *mut WickraGenome {
    let result = catch_unwind(|| {
        let spec = unsafe { opt_str(spec_json) }?;
        Genome::new(spec).ok().map(|inner| {
            Box::into_raw(Box::new(WickraGenome {
                inner,
                pending: None,
            }))
        })
    });
    result.ok().flatten().unwrap_or(ptr::null_mut())
}

/// Destroy a genome handle. Null is a no-op.
///
/// # Safety
/// `handle` must be null or a handle previously returned by
/// [`wickra_genome_new`] and not already freed.
#[no_mangle]
pub unsafe extern "C" fn wickra_genome_free(handle: *mut WickraGenome) {
    if !handle.is_null() {
        drop(unsafe { Box::from_raw(handle) });
    }
}

/// Apply a command JSON and write the response JSON into the caller's buffer.
///
/// Returns the response length in bytes (excluding the terminating NUL), or a
/// negative error code. When `len < cap`, the response and a trailing NUL have
/// been written to `out`; otherwise `out` is left untouched. Pass `out = NULL`,
/// `cap = 0` to query the length.
///
/// # Safety
/// `handle` must be a valid handle; `cmd_json` a valid NUL-terminated C string;
/// `out` either null or a writable buffer of at least `cap` bytes.
#[no_mangle]
pub unsafe extern "C" fn wickra_genome_command(
    handle: *mut WickraGenome,
    cmd_json: *const c_char,
    out: *mut c_char,
    cap: usize,
) -> i32 {
    if handle.is_null() || cmd_json.is_null() {
        return WICKRA_GENOME_ERR_NULL;
    }
    let Some(cmd) = (unsafe { opt_str(cmd_json) }) else {
        return WICKRA_GENOME_ERR_UTF8;
    };
    let store = unsafe { &mut *handle };

    let is_retry = matches!(&store.pending, Some((bytes, _)) if bytes.as_slice() == cmd.as_bytes());
    if !is_retry {
        // `command_json` returns the response (or an in-band error JSON) directly;
        // only a panic is exceptional.
        let Ok(response) = catch_unwind(AssertUnwindSafe(|| store.inner.command_json(cmd))) else {
            return WICKRA_GENOME_ERR_PANIC;
        };
        store.pending = Some((cmd.as_bytes().to_vec(), response));
    }

    let (len, delivered) = {
        let response = &store.pending.as_ref().expect("pending set above").1;
        let bytes = response.as_bytes();
        let len = bytes.len();
        let delivered = len < cap && !out.is_null();
        if delivered {
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), out.cast::<u8>(), len);
                *out.add(len) = 0;
            }
        }
        (len, delivered)
    };
    if delivered {
        store.pending = None;
    }
    i32::try_from(len).unwrap_or(i32::MAX)
}

/// The library version as a static NUL-terminated string (do not free).
#[no_mangle]
pub extern "C" fn wickra_genome_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0")
        .as_ptr()
        .cast::<c_char>()
}

#[cfg(test)]
mod tests {
    use super::{
        wickra_genome_command, wickra_genome_free, wickra_genome_new, wickra_genome_version,
        WickraGenome, WICKRA_GENOME_ERR_NULL,
    };
    use core::ffi::{c_char, CStr};
    use std::ffi::CString;
    use std::ptr;

    const SPEC: &str = r#"{"features":[{"kind":"price","field":"close"}],"symbols":["AAA"]}"#;

    fn read_buf(buf: &[u8]) -> String {
        CStr::from_bytes_until_nul(buf)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    }

    fn handle() -> *mut WickraGenome {
        let spec = CString::new(SPEC).unwrap();
        unsafe { wickra_genome_new(spec.as_ptr()) }
    }

    #[test]
    fn version_command_round_trip() {
        let handle = handle();
        assert!(!handle.is_null());
        let cmd = CString::new(r#"{"cmd":"version"}"#).unwrap();
        let len = unsafe { wickra_genome_command(handle, cmd.as_ptr(), ptr::null_mut(), 0) };
        assert!(len > 0);
        let mut buf = vec![0u8; usize::try_from(len).unwrap() + 1];
        let len2 = unsafe {
            wickra_genome_command(
                handle,
                cmd.as_ptr(),
                buf.as_mut_ptr().cast::<c_char>(),
                buf.len(),
            )
        };
        assert_eq!(len2, len);
        assert!(read_buf(&buf).contains("\"version\""));
        unsafe { wickra_genome_free(handle) };
    }

    #[test]
    fn unknown_cmd_is_in_band_error() {
        let handle = handle();
        let cmd = CString::new(r#"{"cmd":"nope"}"#).unwrap();
        let len = unsafe { wickra_genome_command(handle, cmd.as_ptr(), ptr::null_mut(), 0) };
        assert!(len > 0);
        let mut buf = vec![0u8; usize::try_from(len).unwrap() + 1];
        unsafe {
            wickra_genome_command(
                handle,
                cmd.as_ptr(),
                buf.as_mut_ptr().cast::<c_char>(),
                buf.len(),
            );
        }
        assert!(read_buf(&buf).contains("\"ok\":false"));
        unsafe { wickra_genome_free(handle) };
    }

    #[test]
    fn too_small_buffer_leaves_out_untouched() {
        let handle = handle();
        let cmd = CString::new(r#"{"cmd":"version"}"#).unwrap();
        let mut buf = vec![0xAAu8; 3];
        let len = unsafe {
            wickra_genome_command(
                handle,
                cmd.as_ptr(),
                buf.as_mut_ptr().cast::<c_char>(),
                buf.len(),
            )
        };
        assert!(usize::try_from(len).unwrap() >= buf.len());
        assert!(buf.iter().all(|&b| b == 0xAA));
        unsafe { wickra_genome_free(handle) };
    }

    #[test]
    fn invalid_spec_returns_null() {
        let bad = CString::new(r#"{"seed":1}"#).unwrap();
        let handle = unsafe { wickra_genome_new(bad.as_ptr()) };
        assert!(handle.is_null());
    }

    #[test]
    fn null_guards() {
        let cmd = CString::new(r#"{"cmd":"version"}"#).unwrap();
        let code =
            unsafe { wickra_genome_command(ptr::null_mut(), cmd.as_ptr(), ptr::null_mut(), 0) };
        assert_eq!(code, WICKRA_GENOME_ERR_NULL);
        let handle = handle();
        let code = unsafe { wickra_genome_command(handle, ptr::null(), ptr::null_mut(), 0) };
        assert_eq!(code, WICKRA_GENOME_ERR_NULL);
        unsafe { wickra_genome_free(handle) };
    }

    #[test]
    fn free_null_is_a_noop() {
        unsafe { wickra_genome_free(ptr::null_mut()) };
    }

    #[test]
    fn version_is_nul_terminated() {
        let p = wickra_genome_version();
        let v = unsafe { CStr::from_ptr(p) }.to_str().unwrap();
        assert_eq!(v, env!("CARGO_PKG_VERSION"));
    }
}
