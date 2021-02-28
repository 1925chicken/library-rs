#[allow(dead_code)]
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

        fn mul(&mut self,a:u32,b:u32) -> u32 {
            let mut z = a as u64;
            z *= b as u64;
            let x = ((z as u128 * self.im as u128) >> 64) as u64;
            let mut v = (z - x * self._m as u64) as u32;
            if self._m <= v {
                v += self._m;
            }
            v
        }

        pub fn pow_mod_constexpr(x:i64, mut n:i64,m:i32) -> i64 {
            if m == 1 {
                return 0;
            }
            let _m = m as u32;
            let mut r = 1u64;
            let mut y = safe_mod(x as i64,m as i64);
            while n != 0 {
                if n & 1 == 1{
                    r = (r as u64 * y as u64) % _m as u64;
                    y = (y as u32 * y as u32) as i64 % _m as i64;
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
                let t = d as i64;
                let mut y = Barrett::pow_mod_constexpr(*a as i64,t as i64,n as i32);
                while t != n as i64 - 1  && y != 1 && y != n as i64 - 1 {
                    y = y * y % n as i64;
                }
            }
            true
        }

        fn inv_gcd(mut a:i64,b:i64) -> (i64,i64) {
            a = safe_mod(a,b);
            if a == 0 {
                return (b,0);
            }

            let (mut s,mut t) = (b,a);
            let (mut m0,mut m1) = (0,1);

            while t > 0 {
                let u = s / t;
                s -= t * u;
                m0 -= m1 * u;

                let mut tmp = s;
                s = t;
                t = tmp;
                tmp = m0;
                m0 = m1;
                m1 = tmp;
            }
            if m0 < 0 {
                m0 += b / s;
            }
            (s,m0)
        }
        fn primitive_root_constexpr(m:i32) -> i32 {
            if m == 2 {
                return 1;
            }
            if m == 167772161 {
                return 3;
            }
            if m == 469762049 {
                return 3;
            }
            if m == 754974421 {
                return 11;
            }
            if m == 998244353 {
                return 3;
            }

            let mut divs = vec![0;20];
            let mut cnt = 1;
            let mut x = (m - 1) / 2;
            while x % 2 == 0 {
                x /= 2;
            }
            let mut i = 3;
            while i * i <= x {
                if x % i == 0 {
                    divs[cnt] = i;
                    cnt += 1;
                    while x % i == 0 {
                        x /= i;
                    }
                }
            i += 2;
            }

            if x > 1 {
                divs[cnt] = x;
                cnt += 1;
            }
            let mut g = 2;
            loop {
                let mut ok = true;
                for i in 0..cnt {
                    if Barrett::pow_mod_constexpr(g,(m as i64- 1) / divs[i] as i64,m) == 1 {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    return g as i32;
                }
                g += 1;
            }
            
        }
    

        fn floor_sum_unsigned(mut n:u64,mut m:u64,mut a:u64,mut b:u64) -> u64 {
            let mut ans = 0;
            loop {
                if a >= m {
                    ans += n * (n - 1) / 2 * (a / m);
                    a %= m;
                }
                if b >= m {
                    ans += n * (b / m);
                    b %= m;
                }
                let y_max = a * n + b;
                if y_max < m {
                    break;
                }
                n = y_max / m;
                b = y_max % m;
                std::mem::swap(&mut m,&mut a);
            }
            ans
        }
    }

    fn pow_mod(x:i64,mut n:i64,m:i32) -> i64 {
        assert!(0 <= n && 1 <= m);
        if m == 1 {
            return 0;
        }
        let mut bt = Barrett::new(m as u32);
        let mut r = 1;
        let mut y = safe_mod(x,m as i64) as u64;
        while n != 0 {
            if n & 1 == 1 {
                r = bt.mul(r as u32,y as u32);
            }
            y = bt.mul(y as u32,y as u32) as u64;
            n >>= 1;
        }
        r as i64
    }

    fn inv_mod(x:i64,m:i64) -> i64 {
        assert!(1 <= m);
        let z = Barrett::inv_gcd(x,m);
        assert!(z.0 == 1);
        z.0
    }

    fn crt(r:&Vec<i64>,m:&Vec<i64>) -> (i64,i64) {
        assert!(r.len() == m.len());
        let n = r.len();
        let (mut r0,mut m0) = (0,1);
        for i in 0..n {
            assert!(1 <= m[i]);
            let mut r1 = safe_mod(r[i],m[i]);
            let mut m1 = m[i];
            if m0 < m1 {
                std::mem::swap(&mut r0,&mut r1);
                std::mem::swap(&mut m0,&mut m1);
            }
            if m0 % m1 == 0 {
                if r0 % m1 != r1 {
                    return (0,0);
                }
                continue;
            }


            let (g,im) = Barrett::inv_gcd(m0,m1);
            let u1 = m1 / g;

            if (r1 - r0) % g != 0{
                return (0,0);
            }

            let x = (r1 - r0) / g % u1 * im % u1;

            r0 += x * m0;
            m0 *= u1;
            if r0 < 0 {
                r0 += m0;
            }
        }
        (r0,m0)
    }

    fn floor_sum(n:i64,m:i64,mut a:i64,mut b:i64) -> i64 {
        assert!(0 <= n && n < (1i64<<32));
        assert!(1 <= m && m < (1i64<<32));
        let mut ans = 0;
        if a < 0 {
            let a2 = safe_mod(a,m) as u64;
            ans -= (1u64 * n as u64 * (n as u64 - 1) / 2 *((a2 - a as u64) / m as u64)) as i64;
            a = a2 as i64;
        }
        if b < 0 {
            let b2 = safe_mod(b,m) as u64;
            ans -= (1u64 * n as u64 * ((b2 - b as u64) / m as u64)) as i64;
            b = b2 as i64;
        }
        ans += Barrett::floor_sum_unsigned(n as u64,m as u64,a as u64,b as u64) as i64;
        ans
    }
}

fn main() {

}
