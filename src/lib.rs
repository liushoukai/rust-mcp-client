pub mod server;

use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct IpInfo {
    pub ip: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: String,
    pub timezone: String,
}

impl IpInfo {
    pub fn location(&self) -> Option<(f64, f64)> {
        let mut parts = self.loc.split(',');
        let lat = parts.next()?.parse::<f64>().ok()?;
        let lon = parts.next()?.parse::<f64>().ok()?;
        Some((lat, lon))
    }
}

impl fmt::Display for IpInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "IP Information:")?;
        writeln!(f, "  IP Address: {}", self.ip)?;
        writeln!(f, "  City: {}", self.city)?;
        writeln!(f, "  Region: {}", self.region)?;
        writeln!(f, "  Country: {}", self.country)?;
        if let Some((lat, lon)) = self.location() {
            writeln!(f, "  Location: Latitude={}, Longitude={}", lat, lon)?;
        }
        writeln!(f, "  Organization: {}", self.org)?;
        writeln!(f, "  Timezone: {}", self.timezone)?;
        Ok(())
    }
}

pub async fn fetch_ip_info() -> Result<IpInfo, reqwest::Error> {
    let url = "https://ipinfo.io/json";
    tracing::debug!("正在从 {} 获取 IP 信息", url);

    let response = reqwest::get(url).await?;
    tracing::debug!("收到响应,状态码: {}", response.status());

    let ip_info = response.json::<IpInfo>().await?;
    tracing::debug!("成功解析 IP 信息: {:?}", ip_info);

    Ok(ip_info)
}
