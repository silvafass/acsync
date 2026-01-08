use acsync::CustomError;
use acsync::fs::FileSearcher;
use acsync::{
    cli_helper::{self, Arg, ArgsParser},
    create_args_parser,
};
use std::time::Instant;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    let command = Command::parse();

    if command.debug() {
        dbg!(&command);
    }

    let result = match &command {
        Command::Replicate {
            origin,
            destination,
            back,
            dryrun,
            ..
        } => {
            if let (None, ..) | (.., None) = (origin, destination) {
                Err(CustomError::InvalidInput(
                    "Origin and destination arguments must be informed!",
                ))?;
            }

            if back.unwrap_or_default() {
                println!("Syncing back...");
            }
            if dryrun.unwrap_or_default() {
                println!("Dry run mode...");
            }

            let paths_iter = FileSearcher::new(origin.as_ref().unwrap())
                .into_iter()
                .filter_map(|result| result.ok());

            let mut file_count = 0;
            for _ in paths_iter {
                file_count += 1;
            }
            println!("files found: {file_count}");

            Ok(())
        }
        Command::Setup {
            origin,
            destination,
            ..
        } => {
            if let (None, ..) | (.., None) = (origin, destination) {
                Err(CustomError::InvalidInput(
                    "Origin and destination arguments must be informed!",
                ))?;
            }

            Ok(())
        }
        Command::Entry { .. } => {
            command.print_help();
            Ok(())
        }
    };

    println!("Elapsed execution time: {:?}", now.elapsed());

    result
}
