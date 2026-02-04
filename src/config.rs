use anyhow::{Error as AnyErr, Result};
use config::{Config, Environment, File};
use std::borrow::Cow;

// auto-connect to DB, keep pool global
lazy_static::lazy_static! {
    #[derive(Debug)]
    static ref SETTINGS: Settings<'static> = load().expect("failed loading config");
}

// METHODS
pub fn port() -> u16 {
    SETTINGS.http_port
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct Settings<'a> {
    rust_log: String,
    rust_backtrace: String,
    http_port: u16,
    database_url: Option<Cow<'a, str>>,
}

fn load<'a>() -> Result<Settings<'a>, AnyErr> {
    let s = Config::builder()
        .set_default("http_port", 8080)?
        .set_default("rust_log", "auth-rs-warp=debug")?
        .set_default("rust_backtrace", 1)?
        .set_default("database_url", Some("auth.db"))?
        .add_source(File::with_name("./.config/api_config").required(false))
        .add_source(Environment::default())
        .build()?;
    let settings: Settings<'a> = s.try_deserialize::<Settings>()?;
    validate(&settings)?;

    std::env::set_var("RUST_LOG", &settings.rust_log);
    std::env::set_var("RUST_BACKTRACE", &settings.rust_backtrace);

    Ok(settings)
}

fn validate(settings: &Settings) -> Result<(), AnyErr> {
    anyhow::ensure!(settings.http_port != 0, "http port can't be 0");

    Ok(())
}
