use crossbeam_queue::SegQueue;

use serde::Deserialize;
use serde_json::Value;
use std::{
    fs,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use threadpool::ThreadPool;

use crate::net;
#[derive(Deserialize, Debug)]
struct Info {
    pub ip: String,
    referer: String,
    agent: String,
    platform: String,
}
#[derive(Deserialize)]
struct Views {
    infos: String,
}

pub async fn load_config() {
    let str = fs::read_to_string("views.json").expect("config.json 文件不存在!");
    println!("文件读取一共{}", str.len());
    let views = serde_json::from_str::<Vec<Views>>(&str).expect("转Views失败");
    let mut ip_arr: Vec<String> = Vec::new();
    views.iter().for_each(|x| {
        let infos = &x.infos;
        let info = serde_json::from_str::<Info>(&infos).expect("转Info失败");
        ip_arr.push(info.ip);
    });
    ip_arr.sort();
    ip_arr.dedup();
    println!("ip_arr.len={}", ip_arr.len());
    ip_2_country(&ip_arr).await;
}

pub async fn ip_2_country(ips: &Vec<String>) {
    let max_threads = 8;
    let queue = Arc::new(Mutex::new(ips.clone()));
    let ip_infos = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    let start = Instant::now();
    for _ in 0..max_threads {
        let q_c = Arc::clone(&queue);
        let ip_infos_c = Arc::clone(&ip_infos);
        let handle = tokio::spawn(async move {
            loop {
                let ip = q_c.lock().unwrap().pop();
                if ip.is_none() {
                    break;
                };
                if let Some(ip) = ip {
                    let res = net::get_ipv4_info(&ip).await;
                    if res.is_err() {
                        let e = res.err().unwrap();
                        println!("请求失败{}", e.to_string());
                        // q_c.lock().unwrap().push(ip);
                    } else {
                        let res = res.unwrap();
                        ip_infos_c.lock().unwrap().push(res);
                    }
                }
                thread::sleep(Duration::from_millis(1000));
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    let ip_arr = ip_infos.lock().unwrap().to_vec();
    println!("成功条数{}", ip_arr.len());
    let file_content = serde_json::to_string_pretty(&ip_arr).unwrap();
    let end = Instant::now();
    let duration = end - start;
    println!("全部转换完成,时长={}s", duration.as_secs_f32());
    fs::write("ip_info.json", file_content).expect("写入文件失败");
    // net::get_ipv4_info(ip);
}
