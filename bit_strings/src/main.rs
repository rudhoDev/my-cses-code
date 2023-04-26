use::std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("failed to get input");
    let n : u32 = n.trim().parse().expect("cannot convert");
    solve(n);
}

fn solve(mut var : u32){
    const MOUD : u32 = 1_000_000_007;
    let mut ans: u32 = 1;
    while var!=0 {
        ans=((ans%MOUD)*2)%MOUD;
        var=var-1;
    }
    println!("{}",ans);
}