#[allow(dead_code)]
fn gcd<T>(mut x:T,mut y:T) -> T
where
    T:std::cmp::Ord + Copy + std::ops::Rem<Output=T> + Zero + std::marker::Sized + 
    std::ops::Mul<Output=T> + std::ops::MulAssign + MinusOne + std::ops::Div<Output=T> + std::ops::DivAssign,
{
    if x < T::zero() {
        x *= T::minusone();
    }
    if y < T::zero() {
        y *= T::minusone();
    }
    if x < y{
        std::mem::swap(&mut x,&mut y);
    }
    let mut r = y;
    while !T::is_zero(&r) {
        r = x % y;
        x = y;
        y = r;
    }
    x
}
#[allow(dead_code)]
fn lcm<T>(x:T,y:T) -> T 
where
    T:std::cmp::Ord + Copy + std::ops::Rem<Output=T> + Zero + std::marker::Sized + 
    std::ops::Mul<Output=T> + std::ops::MulAssign + MinusOne + std::ops::Div<Output=T> + std::ops::DivAssign,
{
    x / gcd(x,y) * y
}
trait Zero:std::ops::Rem<Output=Self> + std::marker::Sized {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}
trait MinusOne:std::ops::Mul<Output=Self> + std::marker::Sized {
    fn minusone() -> Self;
}

macro_rules! impl_zero {
    ($($t:ty),*)=> {
        $(
            impl Zero for $t {
                fn zero() -> Self{
                    0
                }
                fn is_zero(&self) -> bool {
                    *self == 0
                }
            }
        )*   
    };
}

macro_rules! impl_minusone {
    ($($t:ty),*)=> {
        $(
        impl MinusOne for $t {
            fn minusone() -> Self {
                -1
            }
        }
        )*
    };
}
impl_zero! {u64,usize,i64,i32,u32,isize}
impl_minusone! {i64,i32,isize}

fn main(){}
