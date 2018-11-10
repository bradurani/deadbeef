use env_logger::*;

pub fn init() {
    Builder::from_env("LOG_LEVEL")
        .default_format_timestamp(false)
        .default_format_module_path(false)
        .target(Target::Stdout)
        .init();
}
