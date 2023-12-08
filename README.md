## LC3100 Simulator

created by Justin Schilleman (jas21ba)

This Rust program simulates a computer running the LC3100 architecture divised in class.

---

### Getting Rust up and running

1. Download and install the Rust compiler from [the Rust website](https://www.rust-lang.org/en-US/downloads.html).
2. Make sure to add the Rust binaries to your PATH if not done so automatically.

---

### Usage

In order to use the simulator, you must provide the path of the machine code as a command line argument.

There are 2 approaches to running the program:

1. Building the program using `cargo build` which will create a binary within the `target/` folder. You can then run the binary as
   follows:
   ```bash
   $ cd target
   $ ./cda3100_lc_sim <PATH>
   ```
2. Running the program directly through `cargo` with `$ cargo run <PATH>` while in the root directory of the project. This option will build binary and run it directly without needing to enter another command to execute it.

---

### Test cases

For this project I created 3 test cases that (in my opinion) adequately test the ability of my simulator. The test cases can be found within the `tests` directory of the project. Within this directory is the provided `LC3101a.c` file which is used to assemble the test cases. The `assembler` file within that is a provided binary for Linux x86-64 (if you are using another platform you can either compile the assembler youtself, or use the provided machine code). The `assembly/` directory holds all of the test cases' source code. The `machine_code/` directory holds all of the assembled machine code.

#### Test case explanations

1. `jas21ba_test1`: Loads the values 10, 5, and 30 from memory. It will add 5 to 10 until it reaches 30. Upon reaching 30, it will store the final answer (30) into the memory address that the initial value (10) came from, and then halt. This program also demonstrates the ability for I-type instructions to properly parse a negative 2's complement offset.
2. `jas21ba_test2`: This test case simply loads the numbers 5 and -1 into registers from memory, and performs the `add` and `nand` operations on the 2 numbers.
3. `jas21ba_test3`: This test case calcuates the 15th digit of the fibbonaci sequence. Once calculated, the number will be stored into memory. The digit of the fibbonaci sequence being calculated can be altered by changing the `n` label's value in memory to whatever digit you wish (WARNING: potential for the program to panic if the integer overflows).
4. `jas21ba_test4`: This test simply showcases the abilities of the cache. `jas21ba_test5` is the same program, however 'performance' varies differently based on cache settings.
---

### Packages used

The only dependecy that this project has besides any `std` library features from Rust itself was a package named [CLAP](https://docs.rs/clap/latest/clap/) (Command Line Argument Parser). I used this package to simplify the process of parsing arguments from the command line, and to automatically generate a help page within the CLI using `<EXEC> -h` or `<EXEC> --help`.
