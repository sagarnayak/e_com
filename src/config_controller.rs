use serde::Deserialize;

use crate::core::constants::{
    ADMIN_EMAIL_DEV,
    ADMIN_EMAIL_PROD,
    ADMIN_EMAIL_TEST,
    ADMIN_NAME_DEV,
    ADMIN_NAME_PROD,
    ADMIN_NAME_TEST,
    ADMIN_PASSWORD_DEV,
    ADMIN_PASSWORD_PROD,
    ADMIN_PASSWORD_TEST,
    DATABASE_NAME_DEV,
    DATABASE_NAME_PROD,
    DATABASE_NAME_TEST,
    DEFAULT_PAGE_SIZE,
    HOST_DEV,
    HOST_PROD,
    HOST_TEST,
    JWT_SECRET_DEV,
    JWT_SECRET_PROD,
    JWT_SECRET_TEST,
    MAX_PAGE_SIZE,
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
                admin_data: AdminData {
                    admin_name: ADMIN_NAME_DEV.to_string(),
                    admin_email: ADMIN_EMAIL_DEV.to_string(),
                    admin_password: ADMIN_PASSWORD_DEV.to_string(),
                },
                paging_conf: PagingConf {
                    default_page_size: DEFAULT_PAGE_SIZE,
                    max_page_size: MAX_PAGE_SIZE,
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
                admin_data: AdminData {
                    admin_name: ADMIN_NAME_TEST.to_string(),
                    admin_email: ADMIN_EMAIL_TEST.to_string(),
                    admin_password: ADMIN_PASSWORD_TEST.to_string(),
                },
                paging_conf: PagingConf {
                    default_page_size: DEFAULT_PAGE_SIZE,
                    max_page_size: MAX_PAGE_SIZE,
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
                admin_data: AdminData {
                    admin_name: ADMIN_NAME_PROD.to_string(),
                    admin_email: ADMIN_EMAIL_PROD.to_string(),
                    admin_password: ADMIN_PASSWORD_PROD.to_string(),
                },
                paging_conf: PagingConf {
                    default_page_size: DEFAULT_PAGE_SIZE,
                    max_page_size: MAX_PAGE_SIZE,
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
pub struct AdminData {
    pub admin_name: String,
    pub admin_email: String,
    pub admin_password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PagingConf {
    pub default_page_size: u32,
    pub max_page_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigData {
    pub database: DatabaseConfig,
    pub jwt: JWTConfig,
    pub admin_data: AdminData,
    pub paging_conf: PagingConf,
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
        /*let google_api_key: &str = &*std::env::var("GOOGLE_API_KEY")
            .unwrap_or_else(|_| "testing_api_key-this_is_not_valid-going_to_panic".into());
        let jwt_key: &str = &*std::env::var("JWT_KEY")
            .unwrap_or_else(|_| "testing_api_key-this_is_not_valid-going_to_panic".into());
        println!("the env is :: {}", &env);
        println!("the google api key is :: {}", &google_api_key);*/
        return match env {
            "dev" => ConfigData::conf_data_for_env(ENV::Development),
            "prod" => ConfigData::conf_data_for_env(ENV::Production),
            "test" => ConfigData::conf_data_for_env(ENV::Testing),
            _ => ConfigData::conf_data_for_env(ENV::Development),
        };
    }
}