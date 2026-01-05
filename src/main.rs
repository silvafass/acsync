use acsync::{
    cli_helper::{self, Arg, ArgsParser},
    create_args_parser,
};

create_args_parser! {
    @attr #[derive(Debug)]
    /// This is another convenient file synchronizer
    enum Command {
        /// Copy files from a origin to a destination directory
        Replicate {
            /// Directory with original files
            origin: Arg<String>,
            /// Destination directory to where files will be replicated
            destination: Arg<String>,
            /// Restore/restored back from destination directory to original director
            back: Option<bool>,
            /// Run command without sideeffect
            dryrun: Option<bool>,
        },
        /// Replication setup
        Setup {
            /// Directory with original files
            origin: Option<String>,
            /// Destination directory to where files will be replicated
            destination: Option<String>,
        },
        @default Entry {},
    }
}

fn main() {
    let command = Command::parse();

    if command.debug() {
        dbg!(&command);
    }

    match &command {
        Command::Replicate {
            origin,
            destination,
            back,
            dryrun,
            ..
        } => {
            if let (None, ..) | (.., None) = (origin, destination) {
                eprintln!("ERROR: Origin and destination arguments must be informed!");
                std::process::exit(1);
            }

            if back.unwrap_or_default() {
                println!("Syncing back...");
            }
            if dryrun.unwrap_or_default() {
                println!("Dry run mode...");
            }
        }
        Command::Setup {
            origin,
            destination,
            ..
        } => {
            if let (None, ..) | (.., None) = (origin, destination) {
                eprintln!("ERROR: Origin and destination arguments must be informed!");
                std::process::exit(1);
            }
        }
        Command::Entry { .. } => command.print_help(),
    }
}
