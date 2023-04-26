use std::io;
use std::collections::HashSet;
fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("error!");
    let _n:u32=n.trim().parse().expect("not a number");
    
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("error");
    let vc: Vec<&str> = buf.trim().split_whitespace().collect();
    
    let mut hs = HashSet::new();
    for element in vc {
        hs.insert(element);
    }
    let n=hs.len();
    println!("{n}");
}
