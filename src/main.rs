use acsync::fs::FileSearcher;
use acsync::{
    cli_helper::{self, Arg, ArgsParser},
    create_args_parser,
};
use std::os::unix::fs::MetadataExt;
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
            /// Question to user if desire override dated files
            override_question: Option<bool>,
            /// Restore back from destination directory to original director
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
    override_question: bool,
    dryrun: bool,
    debug: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let source = source.as_ref().to_path_buf();
    let target = target.as_ref().to_path_buf();

    let includes: Vec<String> =
        if let Ok(includes) = std::fs::read_to_string(source.join(".acsync_includes")) {
            includes
                .split_terminator('\n')
                .map(|item| item.to_string())
                .collect()
        } else {
            vec![]
        };
    let excludes: Vec<String> =
        if let Ok(excludes) = std::fs::read_to_string(source.join(".acsync_excludes")) {
            excludes
                .split_terminator('\n')
                .map(|item| item.to_string())
                .collect()
        } else {
            vec![]
        };

    let paths_iter = FileSearcher::new(&source)
        .includes(&includes)
        .excludes(&excludes)
        .into_iter()
        .filter_map(|result| result.ok());

    let mut file_copied_count = 0;
    let mut file_dated_count = 0;
    let mut file_overrided_count = 0;
    let mut directory_created_count = 0;
    let mut file_count = 0;

    if source.is_dir() && !target.exists() {
        if debug {
            println!("Creating target directory {} ...", target.display());
        }
        if !dryrun {
            let source_metadata = source.metadata()?;

            std::fs::DirBuilder::new().create(&target)?;
            directory_created_count += 1;

            std::fs::set_permissions(&target, source_metadata.permissions())?;
        }
    }

    for source_path in paths_iter {
        let relative_path = source_path.strip_prefix(&source)?;
        let target_path = PathBuf::from(&target).join(relative_path);
        let source_size = source_path.metadata()?.size();
        let target_size = target_path.metadata()?.size();

        let mut check_parent_directory = target_path.as_path();
        while let Some(parent) = check_parent_directory.parent()
            && !parent.exists()
        {
            check_parent_directory = parent;
            let check_relative_path_directory = parent.strip_prefix(&target)?;
            let check_source_path_directory =
                PathBuf::from(&source).join(check_relative_path_directory);
            if check_source_path_directory.is_dir() {
                if debug {
                    println!("Creating directory {} ...", parent.display());
                }
                if !dryrun {
                    let source_metadata = check_source_path_directory.metadata()?;

                    std::fs::DirBuilder::new().create(&parent)?;
                    directory_created_count += 1;

                    std::fs::set_permissions(&parent, source_metadata.permissions())?;
                }
            }
        }

        if target_path.exists() {
            let source_modified_date = source_path.metadata()?.modified()?;
            let target_modified_date = target_path.metadata()?.modified()?;
            if source_modified_date > target_modified_date && source_size != target_size {
                file_dated_count += 1;
                if debug {
                    println!(
                        "File {} is dated in {:?}",
                        target_path.display(),
                        source_modified_date.duration_since(target_modified_date)?
                    );
                }
                if override_question {
                    println!("Do you want to override the file content? (Y/N) ");

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
                }
            } else if debug {
                println!("File already exists: {}", target_path.display());
            }
        } else if source_path.is_file() {
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

    println!("{:#^80}", " Stats ");
    println!("Copied files: {file_copied_count}");
    println!("Dated files: {file_dated_count}");
    println!("Overrided files: {file_overrided_count}");
    println!("Directory created: {directory_created_count}");
    println!("Files found: {file_count}");
    println!("{:#^80}\n", "");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    let command = Command::parse();

    let result = match &command {
        Command::Replicate {
            origin,
            destination,
            override_question,
            back,
            dryrun,
            debug,
        } => {
            let override_question = override_question.unwrap_or_default();
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
                replicate(destination, origin, override_question, dryrun, debug)
            } else {
                replicate(origin, destination, override_question, dryrun, debug)
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
