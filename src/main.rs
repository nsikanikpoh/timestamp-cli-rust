use clap::{Parser, ArgAction};
use timestampcli::run;

#[derive(Parser)]
#[command(name = "timestampcli", version = "0.0.1", author = "Nsikan Ikpoh", about = "Timestamp in Rust")]
struct Opts {
    #[clap(short, long, action = ArgAction::Count)]
    verbose_level: u8,

    #[clap(long, default_value = "default")]
    format: String,

    #[clap(short, long, env = "TIMESTAMPCLI_DEBUG")]
    debug: bool,

    #[clap(subcommand)]
    cmd: Option<Command>,
}

#[derive(Parser)]
enum Command {
    #[clap(name = "sec", about = "Get information about a current time stamp")]
    Info(InfoOpts),
}

#[derive(Parser)]
struct InfoOpts {
    #[clap(help = "Seconds to add to the current timestamp", default_value_t = 0)]
    seconds: u64,
}

fn main() {
    let opts = Opts::parse();

    // Example usage of the global flags
    if opts.debug {
        println!("Debug mode enabled");
    }

    match opts.cmd.unwrap_or(Command::Info(InfoOpts { seconds: 0 })) {
        Command::Info(info_opts) => {
            // Example usage of the verbosity level
            match opts.verbose_level {
                0 => {
                    // Quiet mode
                }
                1 => {
                    println!("Running in verbose mode level 1");
                }
                2 => {
                    println!("Running in verbose mode level 2");
                }
                3 | _ => {
                    println!("Running in verbose mode level 3");
                }
            }

            let output = run(info_opts.seconds, &opts.format);
            println!("{}", output);
        } 

    }
}
