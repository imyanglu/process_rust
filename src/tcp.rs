use std::collections::HashMap;
use windows::Win32::{
    NetworkManagement::IpHelper::{
        GetExtendedTcpTable, MIB_TCPROW_OWNER_PID, TCP_TABLE_OWNER_PID_ALL,
    },
    Networking::WinSock::AF_INET,
};

pub fn get_tcp_connections() -> HashMap<u32, Vec<u16>> {
    let mut pid_ports_map = HashMap::<u32, Vec<u16>>::new();
    unsafe {
        let mut size = 0u32;

        // 第一次调用获取所需缓冲区大小
        let _ = GetExtendedTcpTable(
            None,
            &mut size,
            false.into(),
            AF_INET.0 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        let mut buffer = vec![0u8; size as usize];

        let result = GetExtendedTcpTable(
            Some(buffer.as_mut_ptr() as *mut _),
            &mut size,
            false.into(),
            AF_INET.0 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        // if result != NO_ERROR {
        //     println!("GetExtendedTcpTable failed with error: {:?}", result);
        //     return Ok(());
        // }

        let entry_count = *(buffer.as_ptr() as *const u32);
        let row_ptr = buffer.as_ptr().add(4) as *const MIB_TCPROW_OWNER_PID;
        let rows = std::slice::from_raw_parts(row_ptr, entry_count as usize);

        for row in rows {
            let ip = std::net::Ipv4Addr::from(row.dwLocalAddr.to_ne_bytes());
            let port = u16::from_be(row.dwLocalPort as u16);
            let pid = row.dwOwningPid;
            if pid != 0 {
                let entry = pid_ports_map.entry(pid).or_insert_with(Vec::new);
                entry.push(port);
            }
        }
    }

    pid_ports_map
}
