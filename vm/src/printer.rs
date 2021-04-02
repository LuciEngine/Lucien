use lucien_core::logger::logger;
use slog::debug;
use ruwren::Printer;

// todo redirect wren print to ui text
pub struct LogPrinter;

impl Printer for LogPrinter {
    fn print(&mut self, s: String) {
        if s.len() > 0 {
            debug!(logger(), "* [wren] {}", s);
        }
    }
}
