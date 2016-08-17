use std::io::prelude::*;
use std::fs::File;

pub fn glitch(filename : &str, pattern : &str) {
    println!("Glitching {} with pattern '{}'", filename, pattern);
    println!("generating a file as  a debug...");
    let mut f = File::create(filename).unwrap();
    f.write_all(b"hello, world\n").unwrap();
    f.write(&[1; 1]).unwrap();
    f.write_all(b"damn, hello, world").unwrap();
}

pub fn find(filename : &str) {
    println!("Find non-printables in {}", filename);
    let mut f = File::open(filename).unwrap();
    //let mut buf : Vec<u8> = Vec::with_capacity(65546);
    let mut buf = [0; 65536];
    let mut pos = 0;
    loop {
        let r = f.read(&mut buf);
        match r {
            Ok(s) if s == 0 => {
                return
            }
            Ok(s) => {
                // println!("Read {} bytes", s);
                //println!(">{:?}", &buf[1..s]);
                let line = String::from_utf8_lossy(&buf[0..s]);
                println!("{}", line);
                for (i, c) in line.chars().enumerate() {
                    //print!("{}>{},", c, c as u32);
                    if (c as u32) < 7 {
                        print!("offset:{} invalid:", pos+i); //, c as u32)
                        for d in c.escape_default() {
                            print!("{}", d)
                        }
                        println!("")
                    }
                }
                pos += s;

            }
            Err(e) => {
                println!("Error: {}", e);
                return
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
