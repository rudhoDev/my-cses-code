use std::io;
//use proconio::input;
fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("0");
    let mut t:u32=t.trim().parse().expect("0");
    while t!=0 {
        //input!{
        //    a:u32,
        //    b:u32,
        //    }
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("error");
        let vc: Vec<&str> = buf.trim().split_whitespace().collect();
        let a:u32 = vc[0].parse().unwrap();
        let b:u32 = vc[1].parse().unwrap();
        if a>2*b || b>2*a {
            println!("NO");
        }
        else if (a%3==1) && (b%3!=2) {
            println!("NO");
        } else if (a%3==2) && (b%3!=1) {
            println!("NO");
        } else if (a%3==0) && (b%3!=0) {
            println!("NO");
        } else {
            println!("YES")
        }
        t=t-1;
    }
}
