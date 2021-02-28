pub mod math {
    pub fn safe_mod(mut x:i64,m:i64) -> i64 {
        x %= m;
        if x < 0 {
            x += m;
        }
        x
    }

    pub struct Barrett{
        _m:u32,
        im:u64,
    }

    impl Barrett {
         fn new(m:u32) -> Self{
            Self {
                _m:m,
                im:std::u64::MAX/m as u64 + 1,
            }
        }
                
        fn umod(& self) -> u32 {
           self._m
        }

        fn mul(& self,a:u32,b:u32) -> u32 {
            let mut z = a as u64;
            z *= b as u64;
            let x = ((z as u128 * self.im as u128) >> 64) as u64;
            let mut v = (z - x * self._m as u64) as u32;
            if self._m <= v {
                v += self._m;
            }
            v
        }

        fn pow_mod_constexpr(x:i64, mut n:i64,m:i32) -> i64 {
            if m == 1 {
                return 0;
            }
            let _m = m as u32;
            let mut r = 1u64;
            let mut y= safe_mod(x as i64,m as i64);
            while n != 0 {
                if n & 1 == 1{
                    r = (r * y as u64) % _m as u64;
                    y = (y * y) % _m as i64;
                    n >>= 1;
                }
            }
            r as i64
        }
        
        fn is_prime_const(n:i32) -> bool {
            if n <= 1 {
                return false;
            }
            if n == 2 || n == 7 || n == 61 {
                return true;
            }
            if n % 2 == 0 {
                return false;
            }
            let mut d = n - 1;
            while d % 2 == 0 {
                d /= 2;
            }
            let bases = [2,7,61];
            for a in bases.iter() {
                let mut t = d as i64;
                let mut y = pow_mod_constexpr(*a as i64,t as i64,n as i32);
                while t != n as i64 - 1  && y != 1 && y != n - 1 {
                    y = y * y % n as i64;
                }
            }
            true
        }
    }
}

fn main() {

}
