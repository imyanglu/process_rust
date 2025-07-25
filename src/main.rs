use std::{env, fs, path::Path};

use crate::process::kill_process;
pub mod convert;
pub mod model;
pub mod net;
pub mod perfect;
pub mod process;
pub mod tcp;
pub mod utils;
fn query_process_by_port(port_num: u16) {
    let pid_port_map = tcp::get_tcp_connections();
    let mut pid_num = 0;
    for (pid, port) in pid_port_map.iter() {
        if port.contains(&port_num) {
            pid_num = *pid;
            break;
        }
    }
    if pid_num == 0 {
        println!("未找到端口 {} 的进程!", port_num);
        return;
    }
    let ports = pid_port_map.get(&pid_num).unwrap();
    let ports_str = ports
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let process_info = process::get_process_info(pid_num).unwrap();
    print!(
        "pid {:5} 端口{:4} 名称{:12} 地址{:16} 内存 {} 私有内存 {}",
        pid_num,
        ports_str,
        process_info.name,
        process_info.path(),
        process_info.memory_kb,
        process_info.private_memory_kb
    );
}
#[tokio::main]
async fn main() {
    let res = convert::load_config().await;
    println!("运行结束");

    // let res = net::get_ipv4_info("8.138.99.181").await;
    // if let Err(e) = res {
    //     println!("{}", &e.to_string());
    // }
    // let args: Vec<String> = env::args().into_iter().skip(1).collect();
    // let command = args.get(0);
    // let params = args.get(1);
    // if command.is_none() || params.is_none() {
    //     println!("参数为空!");
    //     return;
    // }
    // let c = command.unwrap();
    // let p = params.unwrap();
    // let upper_c = c.to_uppercase();
    // match upper_c.as_str() {
    //     "-P" => {
    //         query_process_by_port(p.parse::<u16>().unwrap());
    //     }
    //     "-K" => {
    //         let pid = p.parse::<u32>().unwrap();
    //         let process_info = process::get_process_info(pid).unwrap();

    //         kill_process(pid);
    //         println!("已结束进程");
    //         println!(
    //             "pid {:5}  名称{:12} 地址{:16} 内存 {} 私有内存 {}",
    //             pid,
    //             process_info.name,
    //             process_info.path(),
    //             process_info.memory_kb,
    //             process_info.private_memory_kb
    //         );
    //     }
    //     "-KP" => {
    //         let pid_port_map = tcp::get_tcp_connections();
    //         let port_num = p.parse::<u16>().unwrap();
    //         let mut pid_num = 0;
    //         for (pid, port) in pid_port_map.iter() {
    //             if port.contains(&port_num) {
    //                 pid_num = *pid;
    //                 break;
    //             }
    //         }
    //         if pid_num == 0 {
    //             println!("未找到端口 {} 的进程!", port_num);
    //             return;
    //         }
    //         kill_process(pid_num);
    //         println!("已结束进程");
    //     }
    //     "-RD" => {
    //         let path = Path::new(p);
    //         if path.exists() {
    //             let res = fs::remove_dir_all(path);
    //             if let Err(e) = res {
    //                 println!("删除文件夹失败:{}", &e.to_string());
    //             }
    //             return;
    //         }
    //         println!("文件夹不存在")
    //     }
    //     _ => {}
    // };
}
