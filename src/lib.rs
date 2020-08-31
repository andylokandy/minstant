mod coarse_now;
#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
mod tsc_now;

#[inline]
pub fn init() {
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    tsc_now::init();
}

#[inline]
pub fn is_tsc_available() -> bool {
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    if true {
        return tsc_now::is_tsc_available();
    }

    false
}

#[inline]
pub fn now() -> u64 {
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    if is_tsc_available() {
        return tsc_now::now();
    }

    coarse_now::now()
}

#[inline]
pub fn cycles_per_second() -> u64 {
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    if is_tsc_available() {
        return tsc_now::cycles_per_second();
    }

    1_000_000_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tsc_available() {
        let _ = is_tsc_available();
    }

    #[test]
    fn test_now() {
        init();
        let mut prev = 0;
        for _ in 0..100 {
            let cur = now();
            assert!(cur >= prev);
            prev = cur;
        }
    }

    #[test]
    fn test_cycles_per_second() {
        let _ = cycles_per_second();
    }
}
