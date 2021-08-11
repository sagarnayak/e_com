//GLOBAL
pub const B_CRYPT_COST: u32 = 12;
pub const DEFAULT_PAGE_SIZE: u32 = 5;
pub const MAX_PAGE_SIZE: u32 = 100;

//dev
pub const HOST_DEV: &str = "127.0.0.1";
pub const PORT_DEV: u16 = 5432;
pub const USER_DEV: &str = "postgres";
pub const PASSWORD_DEV: &str = "admin";
pub const DATABASE_NAME_DEV: &str = "postgres";

pub const JWT_SECRET_DEV: &str = "GA=a48]zpEV[#F|W^oiw5Wy{}7$H7.?Q[RV!8Y?=-f v[7^VW`lLn$rB2jo@]ho2";

pub const ADMIN_NAME_DEV: &str = "he_who_remains";
pub const ADMIN_EMAIL_DEV: &str = "he_who_remains@apu.com";
pub const ADMIN_PASSWORD_DEV: &str = "admin";

//test
pub const HOST_TEST: &str = "127.0.0.1";
pub const PORT_TEST: u16 = 5432;
pub const USER_TEST: &str = "postgres";
pub const PASSWORD_TEST: &str = "admin";
pub const DATABASE_NAME_TEST: &str = "postgres";

pub const JWT_SECRET_TEST: &str = "GA=a48]zpEV[#F|W^oiw5Wy{}7$H7.?Q[RV!8Y?=-f v[7^VW`lLn$rB2jo@]ho2";

pub const ADMIN_NAME_TEST: &str = "he_who_remains";
pub const ADMIN_EMAIL_TEST: &str = "he_who_remains@apu.com";
pub const ADMIN_PASSWORD_TEST: &str = "admin";

//prod
pub const HOST_PROD: &str = "127.0.0.1";
pub const PORT_PROD: u16 = 5432;
pub const USER_PROD: &str = "postgres";
pub const PASSWORD_PROD: &str = "admin";
pub const DATABASE_NAME_PROD: &str = "postgres";

pub const JWT_SECRET_PROD: &str = "GA=a48]zpEV[#F|W^oiw5Wy{}7$H7.?Q[RV!8Y?=-f v[7^VW`lLn$rB2jo@]ho2";

pub const ADMIN_NAME_PROD: &str = "he_who_remains";
pub const ADMIN_EMAIL_PROD: &str = "he_who_remains@apu.com";
pub const ADMIN_PASSWORD_PROD: &str = "admin";

//custom status code
pub const NEED_PLATFORM_AUTH: u16 = 4011;