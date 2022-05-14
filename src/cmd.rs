use clap::{crate_version, Arg, Command};

pub fn cmd_app() -> Command<'static> {
	Command::new("webhere")
		.version(&crate_version!()[..])
		.about("Simple static file web serving using warp")
		.arg(Arg::new("port").short('p').takes_value(true).help("port (default 8080)"))
}
