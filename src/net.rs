use std::collections::HashMap;

use std::error::Error;
use std::fmt::Write;

use crate::model::IPRes;
pub async fn get_ipv4_info(ipv4: &str) -> Result<IPRes, Box<dyn Error>> {
    let url = format!("https://web-api.nordvpn.com/v1/ips/lookup/{}", ipv4);
    let resp = reqwest::get(url).await?;

    let text = resp.text().await?;
    let res = serde_json::from_str::<IPRes>(&text)?;

    Ok(res)
}
pub fn report(err: &dyn Error) -> String {
    let mut s = String::new();

    // 写入主错误信息
    let _ = writeln!(s, "Error: {}", err);

    // 遍历 error source 链
    let mut source = err.source();
    while let Some(e) = source {
        let _ = writeln!(s, "Caused by: {}", e);
        source = e.source();
    }

    s
}
