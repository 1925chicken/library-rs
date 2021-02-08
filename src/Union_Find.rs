struct union_find{
    par:Vec<i64>
}
impl union_find {
    fn new(n:usize) -> Self {
        let mut vec:Vec<i64> = vec![-1;n];
        union_find {
            par:vec
        }
    }
    fn root<T>(&mut self, x:T) -> T{
        if self.par[x] < 0{
            x
        }
        else{
            par[x] = self.root(par[x]);
        }
    }
    fn same<T>(&mut self, x:T, y:T) -> bool {
        self.root(x) == self.root(y);
    }
    fn unite<T>(&mut self, &mut x:T, &mut y:T) -> bool
    {
        x = self.root(x);
        y = self.root(y);
        if x == y {
            false
        }
        if self.par[x] > self.par[y]{
            std::mem::swap(&mut x, &mut y);
        }
            self.par[x] += self.par[y];
            self.par[y] = x;
            true
    }
    fn size<T> (x:T) -> i64{
        -par[root(x)]
    }
}

use std::io::*;
use std::str::FromStr;
use std::collections::*;
use std::cmp::*;


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
fn main(){
    let n: usize = read();
    let q: usize = read();
    let utf = union_find::new(n);
    for i in 0..q {
        let a:Vec<i64> = (0..3).map(|_| read()).collect();
        if a[0] == 0 {
            utf::unite(a[1] - 1, a[2] - 1);
        }else{
            if utf::same(a[1] - 1, a[2] - 1){
                println!("Yes");
            }else{
                println!("No");
            }
        }
    }
}