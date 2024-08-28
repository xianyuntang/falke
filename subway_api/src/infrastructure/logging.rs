use tracing::error;

pub fn init_tracing() {
    tracing_subscriber::fmt::init();
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            error!("panic occurred: {s}");
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            error!("panic occurred: {s}");
        } else {
            error!("panic occurred with unknown payload");
        }
    }));
}
