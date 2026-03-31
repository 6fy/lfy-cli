use tracing_subscriber::prelude::*;

use crate::constants::env;

pub fn init_logging() {
    let stderr_filter = std::env::var(env::LOG_LEVEL).ok();
    let log_file_dir = std::env::var(env::LOG_FILE).ok();

    if stderr_filter.is_none() && log_file_dir.is_none() {
        return;
    }

    let registry = tracing_subscriber::registry();

    let stderr_layer = stderr_filter.map(|filter| {
        let env_filter = tracing_subscriber::EnvFilter::new(filter);
        tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_target(false)
            .compact()
            .with_filter(env_filter)
    });

    let (file_layer, guard) = if let Some(ref dir) = log_file_dir {
        let file_appender = tracing_appender::rolling::daily(dir, "lfy.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let layer = tracing_subscriber::fmt::layer()
            .json()
            .with_writer(non_blocking)
            .with_target(true)
            .with_filter(tracing_subscriber::EnvFilter::new("lfy_cli=debug"));
        (Some(layer), Some(guard))
    } else {
        (None, None)
    };

    let subscriber = registry.with(stderr_layer).with(file_layer);
    if tracing::subscriber::set_global_default(subscriber).is_ok() {
        std::mem::forget(guard);
    }
}
