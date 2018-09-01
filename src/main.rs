#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_bunyan;
extern crate slog_term;

use slog::*;
use std::fs::File;
use std::sync::Mutex;

pub fn simulate_server(log: Logger) {
    let server = log.new(o!("host" => "localhost", "port" => "8080"));
    let peer1 = server.new(o!("peer_addr" => "8.8.8.8", "port" => "18230"));
    let peer2 = server.new(o!("peer_addr" => "82.9.9.9", "port" => "42381"));

    info!(server, "starting");
    info!(server, "listening");
    debug!(peer2, "connected");
    debug!(peer2, "message received"; "length" => 2);
    debug!(peer1, "connected");
    warn!(peer2, "weak encryption requested"; "algo" => "xor");
    debug!(peer2, "response sent"; "length" => 8);
    debug!(peer2, "disconnected");
    debug!(peer1, "message received"; "length" => 2);
    debug!(peer1, "response sent"; "length" => 8);
    debug!(peer1, "disconnected");
    crit!(server, "internal error");
    info!(server, "exit");
}

fn main() {
    let file = File::create("output.log").expect("error creating log file");
    let file = Mutex::new(slog_bunyan::default(file)).fuse();
    let file = slog_async::Async::new(file).build();

    let term = slog_term::TermDecorator::new().build();
    let term = slog_term::FullFormat::new(term)
        .use_original_order()
        .use_utc_timestamp()
        .build()
        .fuse();
    let term = slog_async::Async::new(term).build();

    let logger = slog::Logger::root(
        Duplicate::new(
            LevelFilter::new(term, Level::Debug),
            LevelFilter::new(file, Level::Warning),
        ).fuse(),
        o!("version" => "0.5", "foo" => "bar"),
    );

    simulate_server(logger);
}
