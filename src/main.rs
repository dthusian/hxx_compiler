use std::env::args;
use std::fs::{read_to_string};
use std::path::PathBuf;
use crate::hxx::hxx_to_ir1;
use crate::ir1::to_ir2::ir1_to_ir2;
use crate::ir2::builtins::default_builtins;

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
      panic!("Compilation failed (HXX->IR1 stage)");
    })
    .unwrap();
  let ir2 = ir1_to_ir2(&[ir1], &default_builtins())
    .map_err(|v| {
      eprintln!("{}", v);
      panic!("Compilation failed (IR1->IR2 stage)");
    })
    .unwrap();
  println!("{:#?}", ir2);
}
