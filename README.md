# jesty

A fast, interactive CLI tool for selecting and running [Jest](https://jestjs.io/) tests in your project.
Pick test files with fuzzy search, pass any Jest flags, and easily rerun your last test run.

<img width="1244" height="850" alt="image" src="https://github.com/user-attachments/assets/5d2ddbd7-d936-4d95-a701-89961f78ab0a" />

## Features

- **Fuzzy-pick** Jest test files interactively
- **Pass any Jest CLI flags** directly (e.g. `--watch`, `-u`)
- **Rerun** your last test run with a single command

## Installation

1. **Clone and build:**
   ```sh
   git clone https://github.com/James-Kni/jesty.git
   cd jesty
   cargo build --release
   ```

2. **(Optional) Install globally:**
   ```sh
   cargo install --path .
   ```

## Usage

### Pick and run tests

```sh
jesty [JEST_FLAGS...]
```

- Launches an interactive picker for test files matching `*.test.{js,jsx,ts,tsx}`.
- All arguments are passed directly to Jest.

**Examples:**
```sh
jesty --watch
jesty -u --coverage
jesty -w 2
```

### Rerun last test run

```sh
jesty rerun
```

- Instantly reruns the last test files and flags you used.

## How it works

- Uses [`skim`](https://github.com/lotabout/skim) for interactive fuzzy file selection.
- Caches your last run (flags and files) in your OS cache directory.
- Passes all CLI arguments (except `rerun`) directly to Jest via `npx jest`.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (for building)
- [Node.js](https://nodejs.org/) and `npx` (for running Jest)
- [`bat`](https://github.com/sharkdp/bat) (optional, for file previews with syntax highlighting; if not installed, `cat` will be used)

## Acknowledgements

- [skim](https://github.com/lotabout/skim) for fuzzy selection
- [ignore](https://docs.rs/ignore/) for fast file walking
- [anyhow](https://docs.rs/anyhow/) for ergonomic error handling
