//tells the rust compiler that the flate2 crate is being used in the code
extern crate flate2;

//imports various modules from flate2 crate and standard library
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
  
    //checks the number of command-line arguments passed to the program. If the number of arguments is not 3(i.e, source and target), it prints an error message to the error output and terminates the program
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

  //creates a new BufReader object by opening the file passed as the first command-line argument. 
    let mut input: BufReader<File> = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

  //creates a new file at the location specified by the second command-line argument and creates a File object that points to it.
    let output = File::create(args().nth(2).unwrap()).unwrap();

  //creates a new GzEncoder object, which will be used to compress the data read from the input file. 
    let mut encoder = GzEncoder::new(output, Compression::default());
  
  //creates a new Instant object, which will be used to measure the time elapsed during the compression process.
    let start = Instant::now();
  
//copies all the data from the input file to the output file, compressing it in the process.
    copy(&mut input, &mut encoder).unwrap();

//finishes the compression process and returns the output file.
    let output = encoder.finish().unwrap();
  
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len:{:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}
