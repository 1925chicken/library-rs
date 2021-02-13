pub mod segment_tree {//基本的には整数型で使うことを想定しています(最悪C++に逃げましょう)
    use std::ops::*;
    fn e<T>() -> T 
    where
        T:Eq + Copy + Add<Output=T> + AddAssign + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign +
          BitXor<Output=T> + BitXorAssign<T> + Mul<Output=T> + MulAssign + Identity,
    {
        T::identity()
    }
    fn op<T>(x:T,y:T) -> T 
    where
        T:Eq + Copy + Add<Output=T> + AddAssign + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign +
          BitXor<Output=T> + BitXorAssign<T> + Mul<Output=T> + MulAssign,
    {
        x ^ y
    }
    pub trait Identity {
        fn identity() -> Self;
    }
 
    macro_rules! impl_identity{
        ($($t:ty),*) => {
            $(
                impl Identity for $t {
                    fn identity() -> Self {
                        0
                    }
                }
            )*
        };
    }
    
 
impl_identity!{i64,usize,u64,i32,u32,isize}
 
    #[derive(Debug)]
    pub struct SegmentTree<T> {
        node:Vec<T>,
        cnt:usize,
        sz:usize,
    }
    impl<T> SegmentTree<T> 
    where
        T:Eq + Copy + Add<Output=T> + AddAssign + BitAnd<Output=T> + BitAndAssign + BitOr<Output=T> + BitOrAssign +
          BitXor<Output=T> + BitXorAssign<T> + Mul<Output=T> + MulAssign + Identity,
    {
        pub fn new(_n:usize) -> Self{
            let mut _cnt = 0;
            while (1usize << _cnt) < _n {
                _cnt += 1;
            }
            let size = 1 << _cnt;
            Self {
                sz:size,
                node:vec![e();size * 2],
                cnt:_cnt,
            }
        }
 
        pub fn set(&mut self,_i:usize,x:T) 
        {   
            let mut i  = _i;
            i += self.sz;
            self.node[i] = x;
            for bit in 1..=self.cnt {
               self.update(i >> bit);
            } 
        }
 
        pub fn get(&mut self,i:usize) -> T {
            self.node[i + self.sz]
        }
        
        pub fn prod(&mut self,_l:usize,_r:usize) -> T {
            let mut resl = e();
            let mut l = _l + self.sz;
            let mut r = _r + self.sz;
            let mut resr = e();
            while l < r {
                if l & 1 == 1 {
                    resl = op(resl,self.node[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    resr = op(resr,self.node[r]);
                }
                l >>= 1;
                r >>= 1;
            }
            op(resl,resr)
        }
 
        pub fn update (&mut self,k:usize)
        {
            self.node[k] = op(self.node[2 * k],self.node[2 * k + 1]);
        }
        
        pub fn all_prod(&mut self) -> T{
            self.node[1]
        }
    }
}
