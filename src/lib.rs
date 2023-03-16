mod log;
mod net;
mod path;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::log::logconfig;
    use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
    use crate::net::tcp_server;

    #[test]
    fn log_test() {
        logconfig::config("/home/noo1y/rust_project/opengames_utils/src/log/log_config.yml");
        error!("Goes to stderr and file");
        warn!("Goes to stderr and file");
        info!("Goes to stderr and file");
        debug!("Goes to file only");
        trace!("Goes to file only");
    }

    #[test]
    fn log_file_test() {
        logconfig::log_file_config("log/requests.log");
    }

    #[test]
    fn async_test(){
        tcp_server::tokio_test();
    }

}
