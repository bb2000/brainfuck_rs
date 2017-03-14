mod brainfuck;

use std::env;
use std::io;
use std::io::{BufReader, Read, Write};
use std::fs::File;

fn main() {
    let program_filename = env::args().nth(1).unwrap();
    let file = File::open(program_filename).unwrap();

    let program : Vec<u8> = file
                    .bytes()
                    .map(|x| x.unwrap())
                    .collect();

    let mut input_buf = String::new();
    print!("Enter input: ");
    io::stdout().flush();
    io::stdin().read_line(&mut input_buf);

    let mut input : Vec<u8> = input_buf.into_bytes();
    input.pop();

    let mut brainfuck = brainfuck::Brainfuck::new(program, input);
    brainfuck.run();

    println!("");
}
