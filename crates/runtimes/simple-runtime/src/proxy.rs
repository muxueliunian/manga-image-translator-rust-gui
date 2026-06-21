/// Windows 下：若用户未显式设置 *_PROXY 环境变量，则读取系统代理（Clash 等
/// 设置的「系统代理」）并注入到 HTTPS_PROXY/HTTP_PROXY/ALL_PROXY，使 ureq
/// （其默认 Config 走 Proxy::try_from_env）自动经代理访问外网。
#[cfg(windows)]
pub fn apply_system_proxy() {
    const VARS: [&str; 6] = [
        "ALL_PROXY",
        "all_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "HTTP_PROXY",
        "http_proxy",
    ];
    if VARS.iter().any(|v| std::env::var_os(v).is_some()) {
        return; // 尊重用户显式配置
    }
    let Some(proxy) = read_windows_system_proxy() else {
        return;
    };
    for v in ["HTTPS_PROXY", "HTTP_PROXY", "ALL_PROXY"] {
        std::env::set_var(v, &proxy);
    }
    log::info!("Detected Windows system proxy, routing HTTP(S) via {proxy}");
}

#[cfg(windows)]
fn read_windows_system_proxy() -> Option<String> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Internet Settings")
        .ok()?;
    let enable: u32 = key.get_value("ProxyEnable").ok()?;
    if enable == 0 {
        return None;
    }
    let server: String = key.get_value("ProxyServer").ok()?;
    let server = server.trim();
    if server.is_empty() {
        return None;
    }
    // ProxyServer 两种形态：简单 "host:port"；分协议 "http=h:p;https=h:p;socks=h:p"
    let endpoint = if server.contains('=') {
        let mut http = None;
        let mut https = None;
        for part in server.split(';') {
            if let Some((k, v)) = part.split_once('=') {
                match k.trim().to_ascii_lowercase().as_str() {
                    "https" => https = Some(v.trim().to_string()),
                    "http" => http = Some(v.trim().to_string()),
                    _ => {}
                }
            }
        }
        https.or(http)?
    } else {
        server.to_string()
    };
    let endpoint = endpoint.trim();
    if endpoint.is_empty() {
        return None;
    }
    if endpoint.contains("://") {
        Some(endpoint.to_string())
    } else {
        Some(format!("http://{endpoint}"))
    }
}

#[cfg(not(windows))]
pub fn apply_system_proxy() {}
