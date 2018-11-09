use env_logger::*;

pub fn init() {
    Builder::from_default_env()
        .default_format_timestamp(false)
        .init();
}
