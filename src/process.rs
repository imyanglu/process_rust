use serde::Serialize;
use std::collections::HashMap;
use std::ffi::OsString;

use std::fmt::Display;

use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::thread::sleep;
use windows::{
    Win32::{
        Foundation::*,
        Storage::FileSystem::{GetLogicalDriveStringsW, QueryDosDeviceW},
        System::{ProcessStatus::*, Threading::*},
    },
    core::PCWSTR,
};
// 将 Windows UTF-16 字符串转换为 Rust 字符串
fn wide_to_string(wide: &[u16]) -> String {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..len])
}

fn get_process_memory_usage(pid: u32) -> Option<(String, usize)> {
    unsafe {
        let handle_res = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
        if handle_res.is_err() {
            return None;
        }
        let h_process = handle_res.unwrap();
        // 获取映像路径
        let mut buffer = [0u16; 260];
        let len = K32GetProcessImageFileNameW(h_process, &mut buffer) as usize;
        let file_path = if len > 0 {
            wide_to_string(&buffer)
        } else {
            "<未知>".to_string()
        };
        let mut mem_counters = PROCESS_MEMORY_COUNTERS::default();
        let mem_ok = K32GetProcessMemoryInfo(
            h_process,
            &mut mem_counters,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        )
        .as_bool();
        let memory_kb = if mem_ok {
            mem_counters.WorkingSetSize / 1024
        } else {
            0
        };
        let _ = CloseHandle(h_process);
        // println!("进程路径: {}", file_path);
        return Some((file_path, memory_kb));
    }
}

#[derive(Serialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo {
    pub name: String,
    path: String,
    memory_kb: usize,
    private_memory_kb: usize,
    pid: u32,
}
impl ProcessInfo {
    pub fn is_system(&self) -> bool {
        let sys_prefix = Path::new("C:\\Windows\\System32");
        let file_path = Path::new(&self.path);
        file_path.starts_with(sys_prefix)
    }
}
impl Display for ProcessInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<32}{:<120}", self.name, self.path)
    }
}

pub fn get_process_info(pid: u32) -> Option<ProcessInfo> {
    unsafe {
        // 获取进程句柄
        let h_process_res = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
        if h_process_res.is_err() {
            return None;
        }
        let h_process = h_process_res.unwrap();
        // 获取私有内存
        let mut mem_ex: PROCESS_MEMORY_COUNTERS_EX = std::mem::zeroed();
        mem_ex.cb = size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
        let _ = K32GetProcessMemoryInfo(h_process, &mut mem_ex as *mut _ as *mut _, mem_ex.cb);
        // let private_usage_kb = mem_ex.PrivateUsage / 1024;
        // 获取映像路径及内存大小
        let mut buffer = [0u16; 260];
        let len = K32GetProcessImageFileNameW(h_process, &mut buffer) as usize;
        let image_path = if len > 0 {
            wide_to_string(&buffer)
        } else {
            "<未知>".to_string()
        };
        let mut mem_counters = PROCESS_MEMORY_COUNTERS::default();
        let mem_ok = K32GetProcessMemoryInfo(
            h_process,
            &mut mem_counters,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        )
        .as_bool();
        let memory_kb = if mem_ok {
            mem_counters.WorkingSetSize / 1024
        } else {
            0
        };
        let mut counters = PROCESS_MEMORY_COUNTERS_EX::default();
        let _ = GetProcessMemoryInfo(
            h_process,
            &mut counters as *mut _ as *mut _,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32,
        );

        let process_name = image_path.split("\\").last().unwrap().to_string();

        let _ = CloseHandle(h_process);
        Some(ProcessInfo {
            name: process_name,
            path: image_path,
            private_memory_kb: counters.PrivateUsage / 1024,
            pid: pid,
            memory_kb,
        })
    }
}
pub fn get_poc() -> Option<Vec<ProcessInfo>> {
    let path_map = get_driver_path();
    let mut process_list: Vec<ProcessInfo> = vec![];
    unsafe {
        let mut pids = [0u32; 1024];
        let size = (pids.len() * std::mem::size_of::<u32>()) as u32;
        let mut bytes_returned = 0;

        if !K32EnumProcesses(pids.as_mut_ptr(), size, &mut bytes_returned).as_bool() {
            return None;
        }
        let mem_size = std::mem::size_of::<u32>();

        let count = bytes_returned as usize / mem_size;
        for &pid in &pids[..count] {
            if pid == 0 {
                continue;
            }
            let process_info = get_process_info(pid);
            if let Some(mut process_info) = process_info {
                let real_path = process_info.path.clone();
                let real_path_res = convert_nt_path(&real_path, &path_map);
                if real_path_res.is_some() {
                    process_info.path = real_path_res.unwrap();
                }
                process_list.push(process_info);
            }
        }
        return Some(process_list);
    }
}

pub fn kill_process(pid: u32) -> bool {
    unsafe {
        let handle_res = OpenProcess(PROCESS_TERMINATE, false, pid);
        if handle_res.is_err() {
            return false;
        }
        let h_process = handle_res.unwrap();
        let _ = TerminateProcess(h_process, 0);
        let _ = CloseHandle(h_process);
        return true;
    }
}
pub fn get_driver_path() -> HashMap<String, String> {
    let mut map = HashMap::new();

    let mut buffer = [0u16; 512];
    let len = unsafe { GetLogicalDriveStringsW(Some(&mut buffer)) } as usize;

    if len == 0 {
        return map;
    }
    let drives = buffer[..len]
        .split(|&c| c == 0)
        .filter(|s| !s.is_empty())
        .map(|s| OsString::from_wide(s).to_string_lossy().to_string())
        .collect::<Vec<String>>();
    for drive in drives {
        let drive_trimmed = drive.trim_end_matches('\\');
        let drive_wide: Vec<u16> = std::ffi::OsStr::new(drive_trimmed)
            .encode_wide()
            .chain(Some(0))
            .collect();
        let mut device_path = [0u16; 1024];
        let len = unsafe { QueryDosDeviceW(PCWSTR(drive_wide.as_ptr()), Some(&mut device_path)) };
        let result = String::from_utf16_lossy(&device_path[..len as usize])
            .trim_end_matches('\0')
            .to_string();
        if !map.contains_key(drive_trimmed) {
            map.insert(result, drive_trimmed.to_string());
        }
        if len == 0 {
            continue;
        }
    }
    map
}

fn convert_nt_path(nt_path: &str, nt_to_dos: &HashMap<String, String>) -> Option<String> {
    for (nt_prefix, dos_prefix) in nt_to_dos {
        if nt_path.starts_with(nt_prefix) {
            let rest = &nt_path[nt_prefix.len()..];
            // 拼接盘符和剩余路径，注意加反斜杠
            let full_path = format!("{}\\{}", dos_prefix, rest.trim_start_matches('\\'));
            return Some(full_path);
        }
    }
    None
}
