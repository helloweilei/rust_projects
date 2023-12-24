use std::{fs, fs::File, io::Read, io::Error};

fn main() {
    print!("method 1:\n {}", read_file("cargo.lock"));
    print!("method 2:\n {}", read_file2("cargo.lock"));
    print!("method 3:\n {}", read_file3("cargo.lock").unwrap());
    print!("method 4:\n {}", read_file4("cargo.lock").unwrap());
}

fn read_file(filename: &str) -> String {
  let f = File::open(filename);
  let mut file = match f {
      Ok(file) => file,
      Err(err) => panic!("open file failed: {}", err),
  };

  let mut s = String::new();
  return match file.read_to_string(&mut s) {
    Ok(_) => s,
    Err(err) => panic!("read file failed: {}", err),
  };
}

fn read_file2(filename: &str) -> String {
  let mut f = File::open(filename).unwrap(); // or .expect("error message")
  let mut s = String::new();
  f.read_to_string(&mut s).expect("read file failed");
  s
}

fn read_file3(filename: &str) -> Result<String, Error> {
  let mut s = String::new();
  File::open(filename)?.read_to_string(&mut s)?;
  Ok(s)
}

fn read_file4(filename: &str) -> Result<String, Error> {
  fs::read_to_string(filename)
}
