# pybites_rust

## Exercise downloader
Exercise downloader for https://rustplatform.com/

### Quickstart

- install the exercise downloader, open a terminal and run:
    ```shell
    cargo install pybites-rust-download
    ```
- `cd` to the directory where you want to save the exercises
- run the downloader (free exercises only):
    ```shell
    pybites-rust-download
    ```
- to download **all** exercises (requires premium), set your API key:
    ```shell
    PYBITES_API_KEY=your-api-key-here pybites-rust-download
    ```
    Or export it in your shell profile so you don't have to pass it every time:
    ```shell
    export PYBITES_API_KEY=your-api-key-here
    ```
    You can find your API key on your [profile page](https://rustplatform.com/profile/).

### Compile it manually

Maybe you want to have a look at the code and make some changes.

- clone the repo and `cd` to the main directory of the project
- compile the downloader
    ```shell
    cd exercise_downloader && \
        cargo build --release
    ```
- cd back to the project main directory
    ```shell
    cd ..
    ```
- run the downloader from the project main directory
    ```shell
    ./exercise_downloader/target/release/pybites-rust-download
    ```
- the downloader will create `exercises` in the current directory

### Makefile

Alternatively, use the Makefile. Be aware that this will create `exercises` in the project main directory, where the `Makefile` is located.

```shell
make download-exercises
```

### Examples

<details><summary>(Open to see some output examples ...)</summary>

Using `cargo` to install from crates.io.

```shell
➜ cargo install pybites-rust-download
  Installing pybites-rust-download v0.1.4

(...)

   Compiling pybites-rust-download v0.1.4
    Finished `release` profile [optimized] target(s) in 15.61s
  Installing /my/home/.cargo/bin/pybites-rust-download
   Installed package `pybites-rust-download v0.1.4` (executable `pybites-rust-download`)

➜
```

Using `make` to compile and execute the exercise downloader.

```shell
➜ make download-exercises
make build-executable && \
exercise_downloader/target/release/pybites-rust-download && \
echo ... all done
make[1]: Entering directory '/my/home/github/pybites_rust'
cd exercise_downloader && \
cargo build --release
    Finished `release` profile [optimized] target(s) in 0.06s
make[1]: Leaving directory '/my/home/github/pybites_rust'
Downloading the exercises from Pybites Rust (rustplatform.com) ✅
'exercises' will be created in the current directory (/my/home/github/pybites_rust/exercises)
21 exercises found!

"Strings and Slices" ✅
"URL Query Parameter Parser" ✅
"Hello Rustacean" ✅
"Vectors and Vec" ✅
"Variables and Mutability" ✅
"Json Serialization" ✅
"Simple Calculations" ✅
"Working with Enums" ✅
"Vowel Counter" ✅
"Using Structs in Rust" ✅
"Fibonacci Sequence" ✅
"Primitive Types" ✅
"Basic Tokenizer" ✅
"Reverse a String" ✅
"Variable Assigment and Mutability" ✅
"Ownership and Borrowing" ✅
"Scopes and Shadowing" ✅
"Function Return Values" ✅
"Result Handling" ✅
"Basic Struct" ✅
"Control Flow" ✅
... all done
```

</details>

### Credits

Originally created by [Giuseppe Cunsolo](https://github.com/markgreene74).
