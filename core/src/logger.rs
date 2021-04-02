// Middleware that provides Luci Engine logger

use sloggers::terminal::TerminalLoggerBuilder;
use sloggers::types::Severity;
use sloggers::Build;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::Logger;

// static mut LOADER: &dyn Loader = &ResourceLoader;

static mut LOGGER: Option<&Logger> = None;
static INITIALIZED: AtomicBool = AtomicBool::new(false);

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

fn set_logger(logger: &'static Logger) {
    set_logger_inner(|| logger)
}

fn set_logger_inner<F>(make_logger: F)
where
    F: FnOnce() -> &'static Logger,
{
    unsafe {
        if std::ptr::read(&INITIALIZED).into_inner()
        {
            println!("loader already initialized");
        }
        else {
            LOGGER = Some(make_logger());
            INITIALIZED.store(true, Ordering::Relaxed);
        }
    }
}

pub fn logger() -> &'static Logger {
    unsafe {
        if !std::ptr::read(&INITIALIZED).into_inner() {
            CoreLogBuilder::new().get_logger()
        } else {
            &LOGGER.unwrap()
        }
    }
}

impl CoreLogBuilder {
    pub fn new() -> Self {
        CoreLogBuilder {
            builder: TerminalLoggerBuilder::new(),
        }
    }

    // Build logger, default: Debug, Stdout, no Source info
    pub fn get_logger(&mut self) -> &'static Logger {
        // todo use env variables
        unsafe {
            if !std::ptr::read(&INITIALIZED).into_inner() {
                self.level(Level::Debug)
                    .destination(Destination::Stderr)
                    .source(Source::None);
                let logger = Box::new(self.builder.build().unwrap());
                set_logger(Box::leak(logger));
            }
            &LOGGER.unwrap()
        }
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
