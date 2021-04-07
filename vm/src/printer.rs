use lucien_core::logger::logger;
use ruwren::Printer;
use slog::debug;

// todo redirect wren print to ui text
pub struct LogPrinter;

impl Printer for LogPrinter {
    fn print(&mut self, s: String) {
        if s.trim().len() > 0 {
            debug!(logger(), "* [wren] {}", s);
        }
    }
}
