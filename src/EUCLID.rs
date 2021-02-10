extern crate num_integer;
macro_rules! FromInteger{
    ($from:expr,$x:ty,$y:ty) => {
        impl From<$x> for $y {
            fn from($from:$x) -> $y {
                $from as $y
            }
        }
    };
}
macro_rules! ConvertTo {
    ($x:expr,$tt:ty,$t:ty) => {
        FromInteger!($x,$tt,$t)
    };
}
use std::ops::{Rem,Not,Mul,Div};
use std::cmp::Ord;
use std::marker::Sized;
fn Euclid <S,T> (mut x:S,mut y:T) -> S 
where
    T:std::fmt::Debug + Ord + Copy + Rem + ConvertTo<S> + num_integer,
    S:std::fmt::Debug + Ord + Copy + Rem + Zero + num_integer,
{  
    let mut Y = ConvertTo!(y,T,S);
    if x < y {
        std::mem::swap(&mut x,&mut Y);
    }
    let mut r = ConvertTo!(1,T,S);
    let z = ConvertTo!(0,T,S);
    while r != z{
        let r = ConvertTo!(x % Y,T,S);
        x = Y;
        Y = r;
    }
    x
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
