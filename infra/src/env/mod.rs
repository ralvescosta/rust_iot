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

    pub amqp_host: &'static str,
    pub amqp_port: u16,
    pub amqp_user: &'static str,
    pub amqp_password: &'static str,
    pub amqp_vhost: &'static str,
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
            amqp_host: "localhost",
            amqp_port: 5672,
            amqp_user: "admin",
            amqp_password: "password",
            amqp_vhost: "",
        })
    }

    pub fn amqp_uri(&self) -> String {
        format!(
            "amqp://{}{}@{}:{}{}",
            self.amqp_user, self.amqp_password, self.amqp_host, self.amqp_port, self.amqp_vhost
        )
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
            amqp_host: "amqp://localhost",
            amqp_port: 5672,
            amqp_user: "admin",
            amqp_password: "password",
            amqp_vhost: "",
        })
    }
}
