// Middleware that provides Luci Engine logger

use sloggers::terminal::TerminalLoggerBuilder;
use sloggers::types::Severity;
use sloggers::Build;

#[derive(Debug)]
pub struct CoreLogBuilder {
    builder: TerminalLoggerBuilder,
}

#[allow(dead_code)]
pub enum Destination {
    Stdout,
    Stderr,
}

#[allow(dead_code)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[allow(dead_code)]
pub enum Source {
    None,
    Module,
    File,
}

impl CoreLogBuilder {
    pub fn new() -> Self {
        CoreLogBuilder {
            builder: TerminalLoggerBuilder::new(),
        }
    }

    // Build logger, default: Debug, Stdout, no Source info
    pub fn get_logger(&mut self) -> slog::Logger {
        // todo use env variables
        self
            .level(Level::Debug)
            .destination(Destination::Stderr)
            .source(Source::None);
        self.builder.build().unwrap()
    }
}

pub trait LoggerConfigTrait {
    fn level(&mut self, level: Level) -> &mut Self;
    fn destination(&mut self, dest: Destination) -> &mut Self;
    fn source(&mut self, source: Source) -> &mut Self;
}

impl LoggerConfigTrait for CoreLogBuilder {
    #[allow(dead_code)]
    fn level(&mut self, level: Level) -> &mut Self {
        let lv = match level {
            Level::Error => Severity::Error,
            Level::Warn => Severity::Warning,
            Level::Info => Severity::Info,
            Level::Debug => Severity::Debug,
            Level::Trace => Severity::Trace,
        };
        self.builder.level(lv);
        self
    }

    #[allow(dead_code)]
    fn destination(&mut self, dest: Destination) -> &mut Self {
        let output = match dest {
            Destination::Stdout => sloggers::terminal::Destination::Stdout,
            Destination::Stderr => sloggers::terminal::Destination::Stderr,
        };
        self.builder.destination(output);
        self
    }

    #[allow(dead_code)]
    fn source(&mut self, source: Source) -> &mut Self {
        let src = match source {
            Source::None => sloggers::types::SourceLocation::None,
            Source::Module => sloggers::types::SourceLocation::ModuleAndLine,
            Source::File => sloggers::types::SourceLocation::LocalFileAndLine,
        };
        self.builder.source_location(src);
        self
    }
}
