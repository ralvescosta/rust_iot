#[derive(Clone, Default, PartialEq, Eq)]
pub enum Environment {
    #[default]
    Local,
    Dev,
    Staging,
    Prod,
}

#[derive(Clone, Default)]
pub struct Config {
    pub app_name: &'static str,
    pub env: Environment,

    pub mqtt_host: &'static str,
    pub mqtt_port: u16,
    pub mqtt_user: &'static str,
    pub mqtt_password: &'static str,

    pub log_level: &'static str,
    pub enable_rumqttc_logging: bool,
}

impl Config {
    pub fn new() -> Box<Self> {
        Box::new(Config {
            app_name: "app-name",
            env: Environment::Local,
            mqtt_host: "localhost",
            mqtt_port: 1883,
            mqtt_user: "mqtt_user",
            mqtt_password: "password",
            log_level: "debug",
            enable_rumqttc_logging: false,
        })
    }

    #[cfg(test)]
    pub fn mock() -> Box<Self> {
        Box::new(Config {
            app_name: "app-name",
            env: Environment::Local,
            mqtt_host: "localhost",
            mqtt_port: 1883,
            mqtt_user: "mqtt_user",
            mqtt_password: "password",
            log_level: "debug",
            enable_rumqttc_logging: false,
        })
    }
}
