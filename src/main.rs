use std::env;

fn main() {
    let http_proxy = env::var("HTTP_PROXY").unwrap_or_else(|_| "未设置".to_string());
    let https_proxy = env::var("HTTPS_PROXY").unwrap_or_else(|_| "未设置".to_string());

    println!("HTTP_PROXY: {}", http_proxy);
    println!("HTTPS_PROXY: {}", https_proxy);
}
