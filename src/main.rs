use acsync::fs::FileSearcher;
use acsync::{
    cli_helper::{self, Arg, ArgsParser},
    create_args_parser,
};
use std::path::PathBuf;
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
            if back.unwrap_or_default() {
                println!("Syncing back...");
            }
            if dryrun.unwrap_or_default() {
                println!("Dry run mode...");
            }

            let origin = origin.as_ref().ok_or("Origin argument must be informed!")?;
            let destination = destination
                .as_ref()
                .ok_or("Destination argument must be informed!")?;
            let paths_iter = FileSearcher::new(origin)
                .into_iter()
                .filter_map(|result| result.ok());

            let mut file_count = 0;
            for origin_path in paths_iter {
                let relative_path = origin_path.strip_prefix(origin)?;
                let destination_path = PathBuf::from(destination).join(relative_path);

                if destination_path.exists() {
                    println!("File {} already exists", destination_path.display());
                } else if origin_path.is_dir() {
                    println!("Creating directory {} ...", destination_path.display());
                    std::fs::DirBuilder::new().create(&destination_path)?;
                } else {
                    println!("Copying file {} ...", relative_path.display());
                    std::fs::copy(&origin_path, &destination_path)?;
                }
                file_count += 1;
            }
            println!("files found: {file_count}");

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
