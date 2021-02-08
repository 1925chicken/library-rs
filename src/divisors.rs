fn enum_divisors(n:i64) -> Vec<i64> {
    let mut res:Vec<i64> = Vec::new();
    let mut i = 1;
    while i * i <= n{
        if n % i == 0{
            let f = n / i;
            if f != i{
                res.push(f);
            }
            res.push(i);
        }
        i += 1;
    }
    res.sort();
    res
}

fn main(){
    println!("{:?}",enum_divisors(100));
}
