pub mod cmd;
pub mod logger;
pub mod resources;

pub type Logger = slog::Logger;
pub type ArgFlags = clap::ArgMatches<'static>;
