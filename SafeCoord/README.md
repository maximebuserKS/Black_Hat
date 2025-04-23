## Rust Installation and Project Setup Guide
First, you need to install Rust 
### Step 1: Install Rust

1. Open a terminal or command prompt.
2. Run the following command to install Rust using `rustup` (the Rust toolchain installer):

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

3. Follow the on-screen instructions to complete the installation.
4. After installation, ensure that the `cargo` command is available by running:

    ```sh
    cargo --version
    ```

    This should print the version of Cargo, Rust's package manager.

### Step 2: Create a New Rust Project

1. Open a terminal or command prompt.
2. Navigate to the directory where you want to create your new project.
3. Run the following command to create a new Rust project:

    ```sh
    cargo new my_project
    ```

    Replace `my_project` with the desired name of your project.

4. Navigate into the newly created project directory:

    ```sh
    cd my_project
    ```

### Step 3: Compile the Rust Project

1. Inside the project directory, run the following command to compile the project:

    ```sh
    cargo build
    ```

    This will compile your project and create an executable in the `target/debug` directory.

### Step 4: Run the Rust Project

1. After compiling the project, run the following command to execute the compiled binary:

    ```sh
    cargo run
    ```

    This will compile (if necessary) and run your project.

### Additional Commands

- To compile the project in release mode (optimized for performance), use:

    ```sh
    cargo build --release
    ```

    The executable will be created in the `target/release` directory.

- To run tests for your project, use:

    ```sh
    cargo test
    ```

### Remarks

You have now installed Rust, created a new project, compiled it, and run it. For more information on Rust, visit the [official Rust documentation](https://www.rust-lang.org/learn).


## Mini-Audit

 


Go inside the SafeCoord folder 


```sh
 cargo run  
```