## LC3100 Simulator

by Justin Schilleman

This Rust program simulates a computer running the LC3100 architecture divised in class.

---

### Getting Rust up and running

1. Download and install the Rust compiler from [the Rust website](https://www.rust-lang.org/en-US/downloads.html).
2. Make sure to add the Rust binaries to your PATH if not done so automatically.

---

### Usage

In order to use the simulator, you must already have assembled machine code ready to be used.

There are 2 approaches to running the program:

1. Building the program using `cargo build` which will create a binary within the `target/` folder. You can then run the binary as
   follows:
   ```bash
   $ cd target
   $ ./cda3100_lc_sim <MACHINE CODE FILE PATH>
   ```
2. Running the program directly through `cargo` with `cargo run <MACHINE CODE FILE PATH>`. This option will build binary and run
   it directly without needing to enter another command to execute it.

---

### Packages used

The only dependecy that this project has besides any `std` library features from Rust itself was a package named [CLAP](https://docs.rs/clap/latest/clap/)(Command Line Argument Parser). I used this package to simplify the process of parsing arguments from the command line, and to automatically generate a help page within the CLI using `<EXEC> -h` or `<EXEC> --help`.
