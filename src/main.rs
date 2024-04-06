use std::env::args;
use std::fs::{read_to_string};
use std::path::PathBuf;
use crate::hxx_ir1::from_hxx::hxx_to_ir1;
use crate::hxx_ir1::to_ir2::ir1_to_ir2;

mod common;
mod hxx_ir1;
mod ir2;
mod ir3;

fn main() {
  let fpath = PathBuf::from(args().nth(1).unwrap());
  let stdlib = read_to_string("support/builtins.hx").expect("File read failed");
  let file = read_to_string(&fpath).expect("File read failed");
  let srcs = vec![
    ("internal:builtins.hx".to_owned(), stdlib),
    (fpath.file_name().unwrap().to_string_lossy().to_string(), file)
  ];
  let ir1s = srcs.into_iter().map(|(filename, v)| {
    hxx_to_ir1(&filename, &v)
      .map_err(|v| {
        eprintln!("{}", v);
        panic!("Compilation failed (HXX->IR1 stage)");
      })
      .unwrap()
  }).collect::<Vec<_>>();
  let ir2 = ir1_to_ir2(&ir1s)
    .map_err(|v| {
      eprintln!("{}", v);
      panic!("Compilation failed (IR1->IR2 stage)");
    })
    .unwrap();
  println!("{:#?}", ir2);
}
