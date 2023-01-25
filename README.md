<h1>Compression</h1>
This program is a command-line utility for compressing files using the Gzip format.

Requirements
Rust programming language
Usage
To use this program, you'll need to have Rust programming language installed on your system. You can check if rust is installed by running the following command in the terminal

  rustc --version

Once you have rust installed, you can run this program by following these steps:

Open the terminal and navigate to the directory where the program's source code is located.

Compile the program using the following command:

    cargo build --release
    
Run the program using the following command:

    cargo run --release <source> <target>
    
Replace <source> with the path to the source file you want to compress and <target> with the path where you want to save the compressed file.

The program will compress the source file and create a new compressed file at the target location. It will also display the source and target file sizes, along with the time taken to compress the file.

If you want to build the program and create the binary you can use the following command:

  cargo build --release

The binary will be located in the target/release directory.

You can then run the binary like this:

  ./target/release/compression <source> <target>

Alternatively, if you want to build the program without creating a binary you can use the following command:

  cargo run --release <source> <target>

This will run the program directly.

Make sure that you have the right permissions to read and write in the source and target file directories.
