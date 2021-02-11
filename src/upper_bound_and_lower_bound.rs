#[allow(dead_code)]
fn upper_bound<T>(f:&Vec<T>,criteria:T) -> usize //criteriaより大きい要素が最初にどこに現れるかを返す
where
    T:std::cmp::Ord,
{
    let las = f.len();
    let mut fir = 0;
    let mut length = las;
    while length != 0{
        let half = length / 2;
        let mut mid = fir;
        mid += half;
        if criteria >= f[mid] {
            length -= half + 1;
            mid += 1;
            fir = mid;
        }else{
            length = half;
        }
    }
    fir
}
#[allow(dead_code)]
fn lower_bound<T>(f:&Vec<T> , criteria:T) -> usize //criteria以上の要素が最初にどこに現れるかを返す
where
    T:std::cmp::Ord,
{
    let las = f.len();
    let mut fir = 0;
    let mut length = las;
    while length != 0 {
        let half = length / 2;
        let mut mid = fir;
        mid += half;
        if f[mid] < criteria {
            length -= half + 1;
            mid += 1;
            fir = mid;
        }else {
            length = half;
        }
    }
    fir
}
