use serde::Deserialize;

use crate::core::constants::{
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
    fn conf_data_for_env(
        environment: ENV,
        google_api_key: String,
        admin_name: String,
        admin_email: String,
        admin_password: String,
        jwt_key: Option<String>,
    ) -> ConfigData {
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
                    secret: if jwt_key.is_some() {
                        jwt_key.unwrap()
                    } else {
                        JWT_SECRET_DEV.to_string()
                    },
                },
                admin_data: AdminData {
                    admin_name,
                    admin_email,
                    admin_password,
                },
                paging_conf: PagingConf {
                    default_page_size: DEFAULT_PAGE_SIZE,
                    max_page_size: MAX_PAGE_SIZE,
                },
                google_api_key,
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
                    secret: if jwt_key.is_some() {
                        jwt_key.unwrap()
                    } else {
                        JWT_SECRET_TEST.to_string()
                    },
                },
                admin_data: AdminData {
                    admin_name,
                    admin_email,
                    admin_password,
                },
                paging_conf: PagingConf {
                    default_page_size: DEFAULT_PAGE_SIZE,
                    max_page_size: MAX_PAGE_SIZE,
                },
                google_api_key,
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
                    secret: if jwt_key.is_some() {
                        jwt_key.unwrap()
                    } else {
                        JWT_SECRET_PROD.to_string()
                    },
                },
                admin_data: AdminData {
                    admin_name,
                    admin_email,
                    admin_password,
                },
                paging_conf: PagingConf {
                    default_page_size: DEFAULT_PAGE_SIZE,
                    max_page_size: MAX_PAGE_SIZE,
                },
                google_api_key,
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
    pub google_api_key: String,
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
        let google_api_key = match std::env::var("GOOGLE_API_KEY") {
            Ok(positive) => {
                positive
            }
            Err(_) => {
                println!("we need a google key to proceed.");
                panic!();
            }
        };
        let admin_name = match std::env::var("ADMIN_NAME") {
            Ok(positive) => {
                positive
            }
            Err(_) => {
                println!("we need admin name to proceed.");
                panic!();
            }
        };
        let admin_email = match std::env::var("ADMIN_EMAIL") {
            Ok(positive) => {
                positive
            }
            Err(_) => {
                println!("we need admin email to proceed.");
                panic!();
            }
        };
        let admin_password = match std::env::var("ADMIN_PASSWORD") {
            Ok(positive) => {
                positive
            }
            Err(_) => {
                println!("we need admin password to proceed.");
                panic!();
            }
        };
        let jwt_key: Option<String> = match std::env::var("JWT_KEY") {
            Ok(positive) => {
                Some(positive)
            }
            Err(_) => {
                println!("Dint found any jwt key. Going to use default in constants.");
                None
            }
        };
        println!("the env is :: {}", &env);
        return match env {
            "dev" => ConfigData::conf_data_for_env(
                ENV::Development,
                google_api_key,
                admin_name,
                admin_email,
                admin_password,
                jwt_key,
            ),
            "prod" => ConfigData::conf_data_for_env(
                ENV::Production,
                google_api_key,
                admin_name,
                admin_email,
                admin_password,
                jwt_key,
            ),
            "test" => ConfigData::conf_data_for_env(
                ENV::Testing,
                google_api_key,
                admin_name,
                admin_email,
                admin_password,
                jwt_key,
            ),
            _ => ConfigData::conf_data_for_env(
                ENV::Development,
                google_api_key,
                admin_name,
                admin_email,
                admin_password,
                jwt_key,
            ),
        };
    }
}