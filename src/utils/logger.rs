use gethostname::gethostname;
pub use log::LevelFilter;
use log::{kv, Log, Metadata, Record};
use std::io::{self, StdoutLock, Write};
use std::time;

#[derive(Debug)]
struct Logger {}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            let level = get_level(record.level());
            let time = time::UNIX_EPOCH.elapsed().unwrap().as_millis();
            write!(
                &mut handle,
                "{{\"level\":{},\"time\":{},\"hostname\":{:?},\"msg\":",
                level,
                time,
                gethostname()
            )
            .unwrap();
            serde_json::to_writer(&mut handle, record.args()).unwrap();
            format_kv_pairs(&mut handle, &record);
            writeln!(&mut handle, "}}").unwrap();
        }
    }
    fn flush(&self) {}
}

fn get_level(level: log::Level) -> u8 {
    match level {
        log::Level::Trace => 10,
        log::Level::Debug => 20,
        log::Level::Info => 30,
        log::Level::Warn => 40,
        log::Level::Error => 50,
    }
}

fn format_kv_pairs<'b>(mut out: &mut StdoutLock<'b>, record: &Record) {
    struct Visitor<'a, 'b> {
        string: &'a mut StdoutLock<'b>,
    }

    impl<'kvs, 'a, 'b> kv::Visitor<'kvs> for Visitor<'a, 'b> {
        fn visit_pair(
            &mut self,
            key: kv::Key<'kvs>,
            val: kv::Value<'kvs>,
        ) -> Result<(), kv::Error> {
            write!(self.string, ",\"{}\":\"{}\"", key, val)?;
            Ok(())
        }
    }

    let mut visitor = Visitor { string: &mut out };
    record.key_values().visit(&mut visitor).unwrap();
}

pub fn init(log_level: String) {
    let log_level = log_level;
    let log_level = match log_level.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    let logger = Box::new(Logger {});
    log::set_boxed_logger(logger).expect("Could not start logging");
    log::set_max_level(log_level);
}
