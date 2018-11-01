#![feature(dbg_macro)]

extern crate pdflib;
use pdflib::PDF;
use std::{fs};
use std::io::{
    BufReader, Read,
};

fn main() {
    let mut buf: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(fs::File::open("pdf/PRML1-20.pdf").expect("File does not exist."));

    reader.read_to_end(&mut buf).expect("read_to_end error");

    let pdf = match PDF::new(buf) {
        Ok(pdf) => pdf,
        Err(err) => {
            println!("pdf new error: {}", err);
            panic!(); 
        }
    };

    dbg!(&pdf);
}
