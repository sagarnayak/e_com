use serde::Deserialize;

use crate::core::constants::{
    DATABASE_NAME_DEV,
    DATABASE_NAME_PROD,
    DATABASE_NAME_TEST,
    HOST_DEV,
    HOST_PROD,
    HOST_TEST,
    JWT_SECRET_DEV,
    JWT_SECRET_PROD,
    JWT_SECRET_TEST,
    PASSWORD_DEV,
    PASSWORD_PROD,
    PASSWORD_TEST,
    PORT_DEV,
    PORT_PROD,
    PORT_TEST,
    USER_DEV,
    USER_PROD,
    USER_TEST,
};

impl ConfigData {
    fn conf_data_for_env(environment: ENV) -> ConfigData {
        match environment {
            ENV::Development => ConfigData {
                database: DatabaseConfig {
                    host: HOST_DEV.to_string(),
                    port: PORT_DEV,
                    user: USER_DEV.to_string(),
                    password: PASSWORD_DEV.to_string(),
                    database_name: DATABASE_NAME_DEV.to_string(),
                },
                jwt: JWTConfig {
                    secret: JWT_SECRET_DEV.to_string(),
                },
            },
            ENV::Testing => ConfigData {
                database: DatabaseConfig {
                    host: HOST_TEST.to_string(),
                    port: PORT_TEST,
                    user: USER_TEST.to_string(),
                    password: PASSWORD_TEST.to_string(),
                    database_name: DATABASE_NAME_TEST.to_string(),
                },
                jwt: JWTConfig {
                    secret: JWT_SECRET_TEST.to_string(),
                },
            },
            ENV::Production => ConfigData {
                database: DatabaseConfig {
                    host: HOST_PROD.to_string(),
                    port: PORT_PROD,
                    user: USER_PROD.to_string(),
                    password: PASSWORD_PROD.to_string(),
                    database_name: DATABASE_NAME_PROD.to_string(),
                },
                jwt: JWTConfig {
                    secret: JWT_SECRET_PROD.to_string(),
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
    Development,
    Testing,
    Production,
}

impl ConfigData {
    pub fn new() -> ConfigData {
        let env: &str = &*std::env::var("RUN_ENV").unwrap_or_else(|_| "dev".into());
        return match env {
            "dev" => ConfigData::conf_data_for_env(ENV::Development),
            "prod" => ConfigData::conf_data_for_env(ENV::Production),
            "test" => ConfigData::conf_data_for_env(ENV::Testing),
            _ => ConfigData::conf_data_for_env(ENV::Development),
        };
    }
}