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
