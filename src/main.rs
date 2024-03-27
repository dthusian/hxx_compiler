use std::env::args;
use std::fs::{read_to_string};
use std::path::PathBuf;
use crate::hxx::hxx_to_ir1;

mod common;
mod hxx;
mod ir1;
mod ir2;
mod ir3;

fn main() {
  let fpath = PathBuf::from(args().nth(1).unwrap());
  let file = read_to_string(&fpath).expect("File read failed");
  let ir1 = hxx_to_ir1(&fpath.file_name().unwrap().to_string_lossy(), &file)
    .map_err(|v| {
      eprintln!("{}", v);
    })
    .unwrap();
  println!("{:#?}", ir1);
}
