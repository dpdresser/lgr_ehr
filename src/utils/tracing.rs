use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

use crate::utils::config::AppSettings;

pub fn init_tracing() {
    let settings = AppSettings::from_env();
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "./logs", "lgr_ehr.log");

    let (non_blocking_file, _guard) = non_blocking(file_appender);
    let (non_blocking_stdout, _guard2) = non_blocking(std::io::stdout());

    registry()
        .with(
            fmt::layer()
                .with_writer(non_blocking_file)
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(
            fmt::layer()
                .with_writer(non_blocking_stdout)
                .with_ansi(true),
        )
        .with(EnvFilter::new(&settings.rust_log))
        .init();

    std::mem::forget(_guard);
    std::mem::forget(_guard2);
}

// TODO: Add log file cleanup policy
