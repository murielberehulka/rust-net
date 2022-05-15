pub const LISTENER_EVENT_TOKEN: mio::Token = mio::Token(0);

pub struct StaticFilesSettings {
    pub root_path: &'static str,
    pub enable_cache: bool
}
impl Default for StaticFilesSettings {
    fn default() -> Self {
        Self {
            root_path: "public",
            enable_cache: false
        }
    }
}

pub struct SocketSettings {
    pub max_payloads: usize
}
impl Default for SocketSettings {
    fn default() -> Self {
        Self {
            max_payloads: 5
        }
    }
}

pub struct Settings {
    pub address: [u8; 4],
    pub port: u16,
    pub socket: SocketSettings,
    pub static_files: Option<StaticFilesSettings>
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            address: [127,0,0,1],
            port: 3000,
            socket: SocketSettings::default(),
            static_files: Some(Default::default())
        }
    }
}