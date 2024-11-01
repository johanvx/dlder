use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct Port(u32);

#[derive(Clone)]
pub struct Proxy {
    pub port: Option<Port>,
}

impl Proxy {
    pub fn new() -> Self {
        Self { port: None }
    }

    pub fn set(&mut self, port: u32) {
        self.port = Some(Port(port));
    }

    pub fn get(&self) -> Option<Port> {
        self.port.clone()
    }

    pub fn remove(&mut self) {
        self.port = None;
    }

    pub fn addr(&self) -> Option<String> {
        self.port.map(|p| format!("http://127.0.0.1:{}", p.0))
    }
}

impl From<&str> for Proxy {
    fn from(value: &str) -> Self {
        Self {
            port: value.parse::<u32>().ok().map(|p| Port(p)),
        }
    }
}
