#[cfg(feature = "kv")]
fn main() {
    use log::*;
    use simplelog::*;

    #[cfg(feature = "ansi_term")]
    let config = ConfigBuilder::new()
        .set_write_log_enable_colors(true)
        .build();
    #[cfg(not(feature = "ansi_term"))]
    let config = Config::default();

    TermLogger::init(
        LevelFilter::Trace,
        config,
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();
    error!(test = 1; "error with keys");
    warn!(test = 2; "warning with keys");
    info!(string = "value"; "info with keys");
    debug!(dur = "5s"; "debug with keys");
    trace!(test = 9000; "trace with keys");
}

#[cfg(not(feature = "kv"))]
fn main() {
    println!("this example requires the kv feature.");
}
