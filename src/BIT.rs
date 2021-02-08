use proconio::*;
struct Bit {
    n:usize,
    bit:Vec<i64>
}
impl Bit {
    fn new(_n:usize) -> Self {
        let vec:Vec<i64> = vec![0;_n + 1];
        Bit {
            bit:vec,
            n:_n
        }
    }
    fn add(&mut self,mut id:usize,x:i64){
        id += 1;
        while id <= self.n {
            self.bit[id] += x;
            let i = id as i64;
            id += (i & (-i)) as usize;
        }
    }
    //sum of [0,i)
    fn sum(&mut self, mut id:usize) -> i64 {
        let mut s:i64 = 0;
        let mut i = id as i64;
        while i > 0 {
            id = i as usize;
            s += self.bit[id];
            i -= i & (-i);
        }
        s
    }
    //sum of [a,b);
    fn segment_sum(&mut self,a:usize,b:usize) -> i64{
       self.sum(b) - self.sum(a)
    }
}
//AC code of Shift and Inversion(ABC190-F)
fn main(){
    input!{
        n:usize,
        a:[i64;n]
    }
    let mut ans = 0;
    let mut bit = Bit::new(n);
    for i in 0..n{
        ans += i as i64 - bit.sum(a[i] as usize);
        bit.add(a[i] as usize,1);
    }
    for i in 0..n{
        println!("{}",ans);
        ans += n as i64  - 1 - a[i];
        ans -= a[i];
    }
}
