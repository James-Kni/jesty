# jesty

A fast, interactive CLI tool for selecting and running [Jest](https://jestjs.io/) tests in your project.
Pick test files with fuzzy search, pass any Jest flags, and rerun your last test run.

<img width="1244" height="850" alt="image" src="https://github.com/user-attachments/assets/5d2ddbd7-d936-4d95-a701-89961f78ab0a" />

## Features

- Interactive fuzzy picker for Jest test files
- Pass through normal Jest args (`--watch`, `-u`, etc.)
- `rerun` command for repeating your last run

## Installation

### Prebuilt binary

Download from the [Releases](https://github.com/James-Kni/jesty/releases) tab, then either:
- put it somewhere in your `PATH`
- run it from wherever you want!

### Build from source

```sh
git clone https://github.com/James-Kni/jesty.git
cd jesty
cargo build --release
```

Optionally install with:

```sh
cargo install --path .
```

## Usage

### Pick and run

```sh
jesty [JEST_FLAGS...]
```

- Opens a picker for files matching `*.test.{js,jsx,ts,tsx}`.
- All additional args are passed to Jest.

Examples:
```sh
jesty --watch
jesty -u --coverage
jesty -w 2
```

### Rerun last test run

```sh
jesty rerun
```

- Runs the same test files & flags from your previous command.

## How it works

- Uses [`skim`](https://github.com/lotabout/skim) for interactive fuzzy file picker.
- Caches your last run (flags and files) in your OS cache directory.
- Passes all CLI arguments (except `rerun`) directly to Jest via `npx jest`.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (for building)
- [Node.js](https://nodejs.org/) and `npx` (for running Jest)
- [`bat`](https://github.com/sharkdp/bat) (optional, for file previews with syntax highlighting; if not installed, `cat` will be used)

## Acknowledgements

- [skim](https://github.com/lotabout/skim) for fuzzy selection
- [ignore](https://docs.rs/ignore/) for fast file walking
