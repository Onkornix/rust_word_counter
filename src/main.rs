use std::collections::HashMap;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub mod words;



#[derive(Parser)]
struct Args {
    /// The pattern to look for
    input: std::path::PathBuf,
    /// The path to the file to read
    output: String
}

fn main() {
    let args = Args::parse();

    let mut new_map = words::Words {
        ungrouped_map: HashMap::new(),
        grouped_map: HashMap::new(),
        values_that_exist: Vec::new(),
    };

    if let Ok(lines) = read_line(args.input) {
        for line in lines.flatten() {
            new_map.update_ungrouped_map(line);
            
        }
    } 
    new_map.create_grouped_map();
    new_map.write_to_file(&args.output);
    
}

fn read_line<P>(filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}