use std::io;

fn main(){
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("no input");
    let n :u32 = n.trim().parse().expect("not a number");
    solve(n);
}

fn solve(a:u32){
    const MAX_LMT :u32 =1_000_000_000;
    let mut ans=0;
    let mut val=5;
    while val<MAX_LMT {
        let cnt=a/val;
        ans=ans+cnt;
        val=val*5;
    }
    println!("{}",ans);
}