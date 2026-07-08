use log::LevelFilter;

/// Helper function to set program's log level
///
/// verbose - User-provided verbose argument
pub fn _set_log_level(verbose: bool) {
    let log_level: LevelFilter = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    colog::default_builder().filter_level(log_level).init();

    ()
}
