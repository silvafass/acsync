# Simple File Synchronizer Tool

This is a project with the practical purpose of learning the Rust language, as well as implementing a solution to use as a backup tool for my local files.

## Expected features

* This should be minimally functional for file backup and restoration.
* This should be able to copy and sync files recursively between the source and destination directories.
* This should preserve the original file's metadata as much as possible.
* This should provide some level os file integritetino validation based in file metadata and/or checksum generation.
* this should never delete or overwrite a file without a prior safeguard as a guarantee of restoration.

## Requirements for learning purposes

* For initial implementation, it should preferably not use any third-party dependencies.
    * Let's try using only the Rust standard library.
    * The use of any third-party dependencies must be justified and documented.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
