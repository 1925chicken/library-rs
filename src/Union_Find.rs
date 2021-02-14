use std::mem::*;
struct UnionFind {
    par: Vec<i64>,
}
#[allow(dead_code)] 
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let par = (0..n).map(|_| -1).collect();
        Self{par}
    }
    pub fn root(&mut self, x: usize) -> i64 {
      if self.par[x] < 0{
          x as i64
      }else{
          self.par[x] = self.root(self.par[x] as usize);
          self.par[x]
      }
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
 
    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut _x: i64 = self.root(x);
        let mut _y: i64 = self.root(y);
        if _x == _y {
            return false;
        }
        if self.par[_x as usize] > self.par[_y as usize] {
            swap(&mut _x, &mut _y);
        }
        self.par[_x as usize] += self.par[_y as usize];
        self.par[_y as usize] = _x as i64;
         return true;
    }
    pub fn size(&mut self, x: usize) -> i64 {
        (self.par[x]) as i64 * -1
    }
}
