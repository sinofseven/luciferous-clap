pub use clap;
use clap::{App, ArgMatches};

pub trait SubCommand<R, E> {
    const NAME: &'static str;
    fn subcommand<'a, 'b>() -> App<'a, 'b>;
    fn run(args: &ArgMatches) -> Result<R, E>;
}
