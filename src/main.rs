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

fn replicate<P: AsRef<std::path::Path>>(
    source: P,
    target: P,
    dryrun: bool,
    debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let source = source.as_ref().to_path_buf();
    let target = target.as_ref().to_path_buf();

    let paths_iter = FileSearcher::new(&source)
        .into_iter()
        .filter_map(|result| result.ok());

    let mut file_copied_count = 0;
    let mut file_dated_count = 0;
    let mut file_overrided_count = 0;
    let mut directory_created_count = 0;
    let mut file_count = 0;

    for source_path in paths_iter {
        let relative_path = source_path.strip_prefix(&source)?;
        let target_path = PathBuf::from(&target).join(relative_path);

        if target_path.exists() {
            let source_metadata = source_path.metadata()?;
            let target_metadata = target_path.metadata()?;

            if source_metadata.modified()? > target_metadata.modified()? {
                file_dated_count += 1;
                println!(
                    "\nFile {} is newer than\
                    \nfile {},\
                    \nDo you want to override the file content? (Y/N) ",
                    source_path.display(),
                    target_path.display()
                );

                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if input.starts_with("y") || input.starts_with("Y") {
                    if debug {
                        println!("Copying file {} ...", relative_path.display());
                    }
                    if !dryrun {
                        std::fs::copy(&source_path, &target_path)?;
                        file_overrided_count += 1;
                    }
                }
            } else if debug {
                println!("File already exists: {}", target_path.display());
            }
        } else if source_path.is_dir() {
            if debug {
                println!("Creating directory {} ...", target_path.display());
            }
            if !dryrun {
                let source_metadata = source_path.metadata()?;

                std::fs::DirBuilder::new().create(&target_path)?;
                directory_created_count += 1;

                std::fs::set_permissions(&target_path, source_metadata.permissions())?;
            }
        } else {
            if debug {
                println!("Copying file {} ...", relative_path.display());
            }
            if !dryrun {
                std::fs::copy(&source_path, &target_path)?;
                file_copied_count += 1;
            }
        }
        file_count += 1;
    }

    println!("\n");
    println!("files copied: {file_copied_count}");
    println!("files dated: {file_dated_count}");
    println!("files overrided: {file_overrided_count}");
    println!("directory created: {directory_created_count}");
    println!("files found: {file_count}");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    let command = Command::parse();

    let result = match &command {
        Command::Replicate {
            origin,
            destination,
            back,
            dryrun,
            debug,
        } => {
            let back = back.unwrap_or_default();
            let dryrun = dryrun.unwrap_or_default();
            let debug = debug.unwrap_or_default();

            if back {
                println!("Syncing back...");
            }
            if dryrun {
                println!("Dry run mode...");
            }

            let origin = origin.as_ref().ok_or("Origin argument must be informed!")?;
            let destination = destination
                .as_ref()
                .ok_or("Destination argument must be informed!")?;

            if back {
                replicate(destination, origin, dryrun, debug)
            } else {
                replicate(origin, destination, dryrun, debug)
            }
        }
        Command::Entry { .. } => {
            command.print_help();
            Ok(())
        }
    };

    println!("Elapsed execution time: {:?}", now.elapsed());

    result
}
