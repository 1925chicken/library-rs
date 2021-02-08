
use std::cmp::*;
use std::collections::*;
use std::io::*;
use std::str::FromStr;
fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}
fn gcd(mut x:i64,mut y:i64) -> i64 {
    let mut r = 1;
    if x < y{
        std::mem::swap(&mut x,&mut y);
    }
    while r != 0 {
        r = x % y;
        x = y;
        y = r;
    }
    x
}

fn lcm(x:i64,y:i64) -> i64{
    let div = gcd(x,y);
    x / div * y
}

fn main(){
    let a:i64 = read();
    let b:i64 = read();
    println!("{} {}" gcd(a,b),lcm(a,b));
}