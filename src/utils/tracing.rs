use color_eyre::config::HookBuilder;
use poem::{FromRequest, Request, RequestBody};
use tracing_appender::{
    non_blocking,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

pub fn init_tracing(log_level: &str, with_hook: bool) {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "./logs", "lgr_ehr.log");

    let (non_blocking_file, _guard) = non_blocking(file_appender);
    let (non_blocking_stdout, _guard2) = non_blocking(std::io::stdout());

    // TODO: make this .env configurable
    if with_hook {
        HookBuilder::default()
            .issue_url("https://github.com/dpdresser/lgr_ehr/issues/auto")
            .install()
            .unwrap();
    }

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
        .with(EnvFilter::new(log_level))
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .with(ErrorLayer::default())
        .init();

    std::mem::forget(_guard);
    std::mem::forget(_guard2);
}

pub fn init_tracing_for_tests() {
    let (non_blocking_stdout, _guard2) = non_blocking(std::io::stdout());

    registry()
        .with(
            fmt::layer()
                .with_writer(non_blocking_stdout)
                .with_ansi(true),
        )
        .with(EnvFilter::new("debug,sqlx=warn,hyper=warn,reqwest=warn"))
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .with(ErrorLayer::default())
        .init();

    std::mem::forget(_guard2);
}

#[derive(Clone)]
pub struct RequestContext {
    pub request_id: String,
}

impl<'a> FromRequest<'a> for RequestContext {
    async fn from_request(req: &'a Request, _: &mut RequestBody) -> poem::Result<Self> {
        let request_id = req
            .header("x-request-id")
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        Ok(RequestContext { request_id })
    }
}

// TODO: Add log file cleanup policy
