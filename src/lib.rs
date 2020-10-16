pub use clap;
use clap::{App, ArgMatches};

pub trait CommandBase {
    type Response;
    type Error;

    const NAME: &'static str;
    fn subcommand<'a, 'b>() -> App<'a, 'b>;
    fn run(args: &ArgMatches) -> Result<Self::Response, Self::Error>;
}