use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
    time::Duration,
};

use shadow_tls::RunningArgs;

pub const BING_HTTP_REQUEST: &[u8; 47] = b"GET / HTTP/1.1\r\nHost: bing.com\r\nAccept: */*\r\n\r\n";
pub const BING_HTTP_RESP: &[u8; 12] = b"HTTP/1.1 301";

pub const CAPTIVE_HTTP_REQUEST: &[u8; 56] =
    b"GET / HTTP/1.1\r\nHost: captive.apple.com\r\nAccept: */*\r\n\r\n";
pub const CAPTIVE_HTTP_RESP: &[u8; 15] = b"HTTP/1.1 200 OK";

pub fn test_ok(
    client: RunningArgs,
    server: RunningArgs,
    http_request: &[u8],
    http_response: &[u8],
) {
    let client_listen = match &client {
        RunningArgs::Client { listen_addr, .. } => listen_addr.clone(),
        RunningArgs::Server { .. } => panic!("not valid client args"),
    };
    client.build().expect("build client failed").start(1);
    server.build().expect("build server failed").start(1);

    // sleep 1s to make sure client and server have started
    std::thread::sleep(Duration::from_secs(3));
    let mut conn = TcpStream::connect(client_listen).unwrap();
    conn.write_all(http_request)
        .expect("unable to send http request");
    conn.shutdown(Shutdown::Write).unwrap();

    // 增加读取超时以避免永久阻塞
    conn.set_read_timeout(Some(Duration::from_secs(5)))
        .expect("Failed to set read timeout");

    // 尝试读取响应，处理可能的错误
    let mut buf = vec![0; http_response.len()];
    match conn.read_exact(&mut buf) {
        Ok(_) => {
            assert_eq!(&buf, http_response);
        }
        Err(e) => {
            panic!("Failed to read response: {:?}. This could be due to protocol changes requiring test adjustments.", e);
        }
    }
}

// 专为V3模式设计的测试函数，增加了容错和更长的等待时间
pub fn test_ok_v3(
    client: RunningArgs,
    server: RunningArgs,
    http_request: &[u8],
    http_response: &[u8],
) {
    let client_listen = match &client {
        RunningArgs::Client { listen_addr, .. } => listen_addr.clone(),
        RunningArgs::Server { .. } => panic!("not valid client args"),
    };
    client.build().expect("build client failed").start(1);
    server.build().expect("build server failed").start(1);

    // 为V3模式增加更长的启动等待时间
    std::thread::sleep(Duration::from_secs(5));

    // 尝试多次连接，增加测试稳定性
    let mut conn = None;
    for _ in 0..3 {
        match TcpStream::connect(&client_listen) {
            Ok(stream) => {
                conn = Some(stream);
                break;
            }
            Err(_) => {
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }

    let mut conn = match conn {
        Some(c) => c,
        None => panic!("Failed to connect after multiple attempts"),
    };

    conn.set_write_timeout(Some(Duration::from_secs(5)))
        .expect("Failed to set write timeout");
    conn.write_all(http_request)
        .expect("unable to send http request");
    conn.shutdown(Shutdown::Write).unwrap();

    // 增加读取超时以避免永久阻塞
    conn.set_read_timeout(Some(Duration::from_secs(5)))
        .expect("Failed to set read timeout");

    // 尝试读取响应，使用更灵活的方式确认响应的开始部分
    let mut buf = vec![0; 1024]; // 使用更大的缓冲区
    match conn.read(&mut buf) {
        Ok(n) if n >= http_response.len() => {
            let response_start = &buf[..http_response.len()];
            if response_start != http_response {
                panic!(
                    "Response doesn't match expected. Got: {:?}, Expected: {:?}",
                    String::from_utf8_lossy(response_start),
                    String::from_utf8_lossy(http_response)
                );
            }
        }
        Ok(n) => {
            panic!(
                "Response too short: got {} bytes, expected at least {} bytes",
                n,
                http_response.len()
            );
        }
        Err(e) => {
            panic!("Failed to read response: {:?}", e);
        }
    }
}

pub fn test_hijack(client: RunningArgs) {
    let client_listen = match &client {
        RunningArgs::Client { listen_addr, .. } => listen_addr.clone(),
        RunningArgs::Server { .. } => panic!("not valid client args"),
    };
    client.build().expect("build client failed").start(1);

    // sleep 1s to make sure client and server have started
    std::thread::sleep(Duration::from_secs(3));
    let mut conn = TcpStream::connect(client_listen).unwrap();
    conn.write_all(b"dummy").unwrap();
    conn.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
    let mut dummy_buf = [0; 1];
    assert!(!matches!(conn.read(&mut dummy_buf), Ok(1)));
}
