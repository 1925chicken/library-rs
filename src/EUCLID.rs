use std::ops::{Rem,Not,Mul,Div};
use std::cmp::Ord;
use std::marker::Sized;
fn Euclid <Mytype<S>,Mytype<T>> (mut x:S,mut y:T) -> S 
where
    T:std::fmt::Debug + Ord + Copy + Rem + ConvertTo<S> + Zero + Sized,
    S:std::fmt::Debug + Ord + Copy + Rem + Sized + Zero,
{   
    let mut Y:S = y.convert();
    let mut r:S = y.convert();
    if x < Y {
        std::mem::swap(&mut x,&mut Y);
    }
    while r != zero()  {
        r = x % Y;
        x = Y;
        Y = r;
    }
    x
}

impl<S> Rem<S> for S 
where
    S:std::fmt::Debug + Ord + Copy + Rem + Sized + Zero
    {
        fn rem(lhs: S,rhs: S){
            lhs % rhs
        }
    }


trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

pub trait Zero:Sized + Rem{
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}
pub fn zero<S>() -> S
where
    S:Zero,
{
    S::zero()
}
impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 {*self as i64}
}

fn main(){
    let mut x = 10;
    let mut y = 3;
    println!("{}",Euclid(x,y));
}
