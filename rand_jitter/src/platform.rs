#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "windows")))]
pub fn get_nstime() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // The correct way to calculate the current time is
    // `dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64`
    // But this is faster, and the difference in terms of entropy is
    // negligible (log2(10^9) == 29.9).
    dur.as_secs() << 30 | dur.subsec_nanos() as u64
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn get_nstime() -> u64 {
    // On Mac OS and iOS std::time::SystemTime only has 1000ns resolution.
    // We use `mach_absolute_time` instead. This provides a CPU dependent
    // unit, to get real nanoseconds the result should by multiplied by
    // numer/denom from `mach_timebase_info`.
    // But we are not interested in the exact nanoseconds, just entropy. So
    // we use the raw result.
    unsafe { libc::mach_absolute_time() }
}

#[cfg(target_os = "windows")]
pub fn get_nstime() -> u64 {
    unsafe {
        let mut t = super::mem::zeroed();
        winapi::um::profileapi::QueryPerformanceCounter(&mut t);
        *t.QuadPart() as u64
    }
}
