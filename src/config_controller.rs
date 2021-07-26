use serde::Deserialize;

impl ConfigData {
    fn conf_data_for_env(environment: ENV) -> ConfigData {
        match environment {
            ENV::Development => ConfigData {
                database: DatabaseConfig {
                    host: "127.0.0.1".to_string(),
                    port: 5432,
                    user: "postgres".to_string(),
                    password: "admin".to_string(),
                    database_name: "postgres".to_string(),
                },
                jwt: JWTConfig {
                    secret: "GA=a48]zpEV[#F|W^oiw5Wy{}7$H7.?Q[RV!8Y?=-f v[7^VW`lLn$rB2jo@]ho2".to_string(),
                },
            },
            ENV::Testing => ConfigData {
                database: DatabaseConfig {
                    host: "127.0.0.1".to_string(),
                    port: 5432,
                    user: "postgres".to_string(),
                    password: "admin".to_string(),
                    database_name: "postgres".to_string(),
                },
                jwt: JWTConfig {
                    secret: "GA=a48]zpEV[#F|W^oiw5Wy{}7$H7.?Q[RV!8Y?=-f v[7^VW`lLn$rB2jo@]ho2".to_string(),
                },
            },
            ENV::Production => ConfigData {
                database: DatabaseConfig {
                    host: "127.0.0.1".to_string(),
                    port: 5432,
                    user: "postgres".to_string(),
                    password: "admin".to_string(),
                    database_name: "postgres".to_string(),
                },
                jwt: JWTConfig {
                    secret: "GA=a48]zpEV[#F|W^oiw5Wy{}7$H7.?Q[RV!8Y?=-f v[7^VW`lLn$rB2jo@]ho2".to_string(),
                },
            },
            ENV::Default => ConfigData {
                database: DatabaseConfig {
                    host: "127.0.0.1".to_string(),
                    port: 5432,
                    user: "postgres".to_string(),
                    password: "admin".to_string(),
                    database_name: "postgres".to_string(),
                },
                jwt: JWTConfig {
                    secret: "GA=a48]zpEV[#F|W^oiw5Wy{}7$H7.?Q[RV!8Y?=-f v[7^VW`lLn$rB2jo@]ho2".to_string(),
                },
            },
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JWTConfig {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigData {
    pub database: DatabaseConfig,
    pub jwt: JWTConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Default,
    Development,
    Testing,
    Production,
}

impl ConfigData {
    pub fn new() -> ConfigData {
        let env: &str = &*std::env::var("RUN_ENV").unwrap_or_else(|_| "def".into());
        return match env {
            "dev" => ConfigData::conf_data_for_env(ENV::Development),
            "prod" => ConfigData::conf_data_for_env(ENV::Production),
            "test" => ConfigData::conf_data_for_env(ENV::Testing),
            _ => ConfigData::conf_data_for_env(ENV::Default),
        };
    }
}