use colored::Colorize;
use env_logger::filter::{Builder as FilterBuilder, Filter};
use log::{LevelFilter, Log, Metadata, Record};
use std::{env, thread};

pub struct Logger {
    filter: Filter,
}

impl Logger {
    pub fn new() -> Self {
        let mut builder = FilterBuilder::new();
        builder.filter(None, LevelFilter::Info);

        if let Ok(rust_log) = env::var("RUST_LOG") {
            builder.parse(&rust_log);
        }

        Self {
            filter: builder.build(),
        }
    }

    pub fn filter(&self) -> LevelFilter {
        self.filter.filter()
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record<'_>) {
        if self.filter.matches(record) {
            let thread_name = thread::current().name().unwrap_or_default().to_string();
            let timestamp = time::strftime("%Y-%m-%d %H:%M:%S %Z", &time::now()).unwrap();

            let stderr_isatty = atty::is(atty::Stream::Stderr);
            let timestamp = if stderr_isatty {
                timestamp.bold()
            } else {
                timestamp.normal()
            };
            let thread_name = if stderr_isatty {
                thread_name.blue().bold()
            } else {
                thread_name.normal()
            };
            let log_level = record.level();
            let log_target = record.target();
            let log_message = record.args();
            eprintln!("#{} {} {} {}  {}", timestamp, thread_name, log_level, log_target, log_message);
        }
    }

    fn flush(&self) {}
}
