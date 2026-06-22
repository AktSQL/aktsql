#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "windows")]
use std::mem;

pub fn process_memory_label() -> String {
    process_resident_memory_bytes()
        .map(format_bytes)
        .unwrap_or_else(|| String::from("--"))
}

#[cfg(target_os = "linux")]
fn process_resident_memory_bytes() -> Option<u64> {
    parse_linux_status_rss_bytes(&fs::read_to_string("/proc/self/status").ok()?)
}

#[cfg(target_os = "macos")]
fn process_resident_memory_bytes() -> Option<u64> {
    macos_resident_memory_bytes()
}

#[cfg(target_os = "windows")]
fn process_resident_memory_bytes() -> Option<u64> {
    windows_resident_memory_bytes()
}

#[cfg(all(unix, not(any(target_os = "linux", target_os = "macos"))))]
fn process_resident_memory_bytes() -> Option<u64> {
    unix_rusage_resident_memory_bytes()
}

#[cfg(not(any(unix, target_os = "windows")))]
fn process_resident_memory_bytes() -> Option<u64> {
    None
}

#[cfg(target_os = "linux")]
fn parse_linux_status_rss_bytes(status: &str) -> Option<u64> {
    let value = status
        .lines()
        .find_map(|line| line.strip_prefix("VmRSS:"))?
        .split_whitespace()
        .next()?;

    value.parse::<u64>().ok().map(|kib| kib * 1024)
}

#[cfg(target_os = "macos")]
fn macos_resident_memory_bytes() -> Option<u64> {
    let mut info = unsafe { std::mem::zeroed::<libc::proc_taskinfo>() };
    let size = std::mem::size_of::<libc::proc_taskinfo>() as i32;
    let read = unsafe {
        libc::proc_pidinfo(
            std::process::id() as i32,
            libc::PROC_PIDTASKINFO,
            0,
            (&mut info as *mut libc::proc_taskinfo).cast(),
            size,
        )
    };

    (read == size).then_some(info.pti_resident_size)
}

#[cfg(target_os = "windows")]
fn windows_resident_memory_bytes() -> Option<u64> {
    use windows_sys::Win32::System::ProcessStatus::{
        GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS,
    };
    use windows_sys::Win32::System::Threading::GetCurrentProcess;

    let mut counters = unsafe { mem::zeroed::<PROCESS_MEMORY_COUNTERS>() };
    counters.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
    let ok = unsafe {
        GetProcessMemoryInfo(
            GetCurrentProcess(),
            &mut counters,
            mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        )
    };

    (ok != 0).then_some(counters.WorkingSetSize as u64)
}

#[cfg(all(unix, not(any(target_os = "linux", target_os = "macos"))))]
fn unix_rusage_resident_memory_bytes() -> Option<u64> {
    let mut usage = unsafe { std::mem::zeroed::<libc::rusage>() };
    let ok = unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) };

    (ok == 0).then(|| (usage.ru_maxrss as u64) * 1024)
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes as f64;
    let mut unit = 0usize;

    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{bytes} {}", UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn parses_linux_rss_from_proc_status() {
        let status = "Name:\taktsql\nVmRSS:\t  12345 kB\nVmSize:\t  54321 kB\n";

        assert_eq!(parse_linux_status_rss_bytes(status), Some(12_641_280));
    }

    #[test]
    fn formats_bytes_for_status_bar() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MiB");
    }
}
