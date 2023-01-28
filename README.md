<h1>Compression</h1>
This program is a command-line utility for compressing files using the Gzip format.

<h2>Requirements</h2>

Rust programming language

<h2>Usage</h2>
<ul>
<li>To use this program, you'll need to have Rust programming language installed on your system. You can check if rust is installed by running the following command in the terminal</li>

  rustc --version

Once you have rust installed, you can run this program by following these steps:

<li>Open the terminal and navigate to the directory where the program's source code is located.</li>

<li>Compile the program using the following command:</li>

    cargo build --release
    
<li>Run the program using the following command:</li>

    cargo run --release <source> <target>
    
<li>Replace <source> with the path to the source file you want to compress and <target> with the path where you want to save the compressed file.</li>

<li>The program will compress the source file and create a new compressed file at the target location. It will also display the source and target file sizes, along with the time taken to compress the file.</li>

<li>If you want to build the program and create the binary you can use the following command:</li>

  cargo build --release

<li>The binary will be located in the target/release directory.</li>

<li>You can then run the binary like this:</li>

  ./target/release/compression <source> <target>

<li>Alternatively, if you want to build the program without creating a binary you can use the following command:</li>

  cargo run --release <source> <target>

<li>This will run the program directly.</li>

<li>Make sure that you have the right permissions to read and write in the source and target file directories.</li>
</ul>
