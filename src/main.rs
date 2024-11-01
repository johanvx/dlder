use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, ExecutableCommand};
use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::Path;
use std::process::{exit, Command};

static YT_DLP: &'static str = "./yt-dlp.exe";

struct Proxy {
    pub port: Option<u32>,
}

impl Proxy {
    fn new() -> Self {
        Self { port: None }
    }

    fn get_addr(&self) -> Option<String> {
        self.port.map(|p| format!("http://127.0.0.1:{}", p))
    }

    fn ask_from_stdin(&mut self) {
        let mut use_proxy = String::new();
        print!("是否启用代理? 1 启用代理, 0 不启用代理: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut use_proxy)
            .expect("未成功读取输入");
        let use_proxy = match use_proxy.trim().parse::<u32>() {
            Ok(n) if n <= 1 => n == 1,
            _ => panic!("非法输入! 请输入 1 或 0."),
        };

        match use_proxy {
            false => println!("不启用代理"),
            true => {
                print!("请输入代理的端口号: ");
                let mut port = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut port).expect("未成功读取输入");
                match port.trim().parse::<u32>().ok() {
                    None => eprintln!("端口号解析错误, 将不启用代理."),
                    p => self.port = p,
                }
            }
        }
    }
}

fn main() {
    // request_admin_privilege();
    match Path::new(YT_DLP).exists() {
        false => eprintln!(
            "Error: {} not found. Please put it in the current working directory",
            YT_DLP
        ),
        true => {
            // 代理 & 端口号
            let mut proxy = Proxy::new();
            proxy.ask_from_stdin();
            let proxy_addr = proxy.get_addr();
            if let Some(ref addr) = proxy_addr {
                println!("代理地址: {}", addr);
            }

            loop {
                // URL
                let mut url = String::new();
                print!("请输入视频网址: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut url).expect("未成功读取网址");
                let url = url.trim();

                // Build args
                let mut args = vec![
                    url,
                    "--no-playlist",
                    "--format",
                    "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4] / bv*+ba/b",
                    "--ffmpeg-location",
                    "./ffmpeg.exe",
                    "--cookies",
                    "./cookies.txt",
                ];
                if let Some(ref addr) = proxy_addr {
                    args.push("--proxy");
                    args.push(addr);
                }

                let status = Command::new(YT_DLP)
                    .args(&args)
                    .status()
                    .expect("未成功执行视频下载程序");

                if status.success() {
                    println!("成功从 {} 下载视频", url);
                } else {
                    println!("下载失败, {}", status);
                }

                println!("");
            }
        }
    }
}

fn request_admin_privilege() {
    let mut res = winres::WindowsResource::new();
    res.set_manifest(
        r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#,
    );
}
