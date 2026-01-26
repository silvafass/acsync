# ACSync - Another Convenient File Synchronizer

`acsync` is a lighweight, simple command-line file synchronizer tool. It was created as a learning project and as a pratical backup tool for my local files.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Examples](#examples)
- [Testing](#testing)
- [License](#license)
- [Contribution](#contribution)

---

## Features

| Feature | Status |
|---------|--------|
| Recursively copy directories | ✅ |
| Preserve file permissions | ✅ |
| Skip files based on *include* / *exclude* patterns | ✅ |
| Dry‑run mode (no changes are written) | ✅ |
| Override‑prompt for dated files | ✅ |
| Statistics report after sync | ✅ |
| Minimal dependencies (only stdlib) | ✅ |

---

## Installation

`acsync` uses Cargo, so you need a recent Rust toolchain installed.

```bash
cargo install --git https://github.com/silvafass/acsync
```

---

## Usage

```bash
Copy files from a origin to a destination directory

Usage: acsync replicate [OPTIONS] [ARGS]...

Arguments:
        origin               Directory with original files
        destination          Destination directory to where files will be replicated

Options:
        --override_question  Question to user if desire override dated files
        --back               Restore back from destination directory to original director
        --dryrun             Run command without sideeffect
        --debug              Enable debug mode
```

### Examples

#### 1. Simple copy

```bash
acsync replicate /home/user/Documents /media/backup/Documents
```

#### 2. Dry‑run with debug output

```bash
acsync replicate /home/user/Documents /media/backup/Documents --dryrun --debug
```

The program will walk the tree, print each file it *would* copy, and give a summary – but **no files are written**.

#### 3. Override prompt for dated files

```bash
acsync replicate /home/user/Documents /media/backup/Documents --override_question
```

During the run, when a file in the destination is older than the source, `acsync` will present information about how much dated the file is and ask for confirmation if you really want to override.

#### 4. Restore from backup

```bash
acsync replicate /media/backup/Documents /home/user/Documents --back
```

The same logic runs but source and destination are swapped.

#### 5. Using include/exclude lists

Create `.acsync_includes` in `/home/user/Documents`:

```
src/
README.md
```

Create `.acsync_excludes`:

```
target
.tmp
```

Now run:

```bash
acsync replicate /home/user/Documents /media/backup/Documents
```

Only files in `src/` and the `README.md` will be copied; the `target/` directory and any `.tmp` files will be skipped.

---

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
