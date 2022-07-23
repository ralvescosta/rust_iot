#[derive(Clone, Default)]
pub struct Config {
    pub app_name: &'static str,

    pub mqtt_host: &'static str,
    pub mqtt_port: u16,
    pub mqtt_user: &'static str,
    pub mqtt_password: &'static str,

    pub is_logging_enabled: bool,
    pub log_level: &'static str,
}

impl Config {
    pub fn new() -> Box<Self> {
        Box::new(Config {
            app_name: "app-name",
            mqtt_host: "localhost",
            mqtt_port: 1883,
            mqtt_user: "mqtt_user",
            mqtt_password: "password",
            is_logging_enabled: true,
            log_level: "debug",
        })
    }
}
