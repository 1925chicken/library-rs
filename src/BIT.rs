use proconio::*;
struct BIT {
    n:usize,
    Bit:Vec<i64>
}
impl BIT {
    fn new(_n:usize) -> Self {
        let mut vec:Vec<i64> = vec![0;_n + 1];
        BIT {
            Bit:vec,
            n:_n
        }
    }
    fn add(&mut self,mut id:usize,x:i64){
        id += 1;
        while id <= self.n {
            self.Bit[id] += x;
            let mut I = id as i64;
            id += (I & (-I)) as usize;
        }
    }
    //sum of [0,i)
    fn sum(&mut self, mut id:usize) -> i64 {
        let mut s:i64 = 0;
        let mut I = id as i64;
        while I > 0 {
            id = I as usize;
            s += self.Bit[id];
            I -= I & (-I);
        }
        s
    }
    //sum of [a,b);
    fn segment_sum(&mut self, mut a:usize,mut b:usize) -> i64{
       self.sum(mut b) - self.sum(mut a)
    }
}
//AC code of Shift and Inversion(ABC190-F)
fn main(){
    input!{
        n:usize,
        a:[i64;n]
    }
    let mut ans = 0;
    let mut bit = BIT::new(n);
    for i in 0..n{
        ans += i as i64 - bit.sum(mut(a[i] as usize));
        let mut I = i;
        bit.add(mut (a[i] as usize),1);
    }
    for i in 0..n{
        println!("{}",ans);
        ans += (n as i64  - 1 - a[i]);
        ans -= (a[i]);
    }
}
