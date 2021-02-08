use std::io::*;
use std::str::FromStr;
struct LCA{
    depth:Vec<u32>,
    dig:i32,
    cost:Vec<Vec<i32>>,
    costs:Vec<i32>,
    g:Vec<Vec<usize>>,
    doubling:Vec<Vec<Option<usize>>>
}

impl LCA{
    fn new(_n:usize) -> Self{
        let dep:Vec<u32> = vec![0;_n];
        let  c:Vec<i32> = vec![0;_n];
        let  _c:Vec<Vec<i32>> = vec![vec![];_n];
        let  _g:Vec<Vec<usize>> = vec![vec![];_n];
        let  digit:usize = 64;
        let  doub:Vec<Vec<Option<usize>>> = vec![vec![None;_n];digit];
        LCA{
            depth:dep,
            costs:c,
            cost:_c,
            doubling:doub,
            dig:digit as i32,
            g:_g
        }
    }

    fn addedge(&mut self,u:usize,v:usize,c:i32){//コスト指定がなければc = 0を渡す
        self.g[u].push(v);
        self.g[v].push(u);
        self.cost[u].push(c);
        self.cost[v].push(c);
    }

    fn dfs(&mut self,now:usize,par:Option<usize>,d:u32,c:i32){
        self.doubling[0][now] = par;
        self.depth[now] = d;
        self.costs[now] = c;
        for i in 0..self.g[now].len(){
            if Some(self.g[now][i]) != par {
                self.dfs(self.g[now][i],Some(now),d + 1, c + self.cost[now][i]);
            }
        }
    }

    fn construct(&mut self){
        self.dfs(0,None,0,0);
        for i in 0..(self.dig as usize - 1){
            for j in 0..self.doubling[i].len(){
                if self.doubling[i][j].is_some(){
                    self.doubling[i + 1][j] = self.doubling[i][self.doubling[i][j].unwrap()];
               }
            }
         }
    }

    fn answer_to_query(&mut self, mut u:usize,mut v:usize) -> i32{
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u,&mut v);
        }
        for i in 0..self.dig as usize {
            if (((self.depth[v] - self.depth[u]) >> i) & 1) > 0 {
                v = self.doubling[i][v].unwrap();
            }
        }
        if u == v {
           return u as i32;
        }
        for i in (0..self.dig as usize).rev(){
            if self.doubling[i][u] != self.doubling[i][v]{
                u = self.doubling[i][u].unwrap();
                v = self.doubling[i][v].unwrap();
            }
        }
        self.doubling[0][u].unwrap() as i32
    }

   /* fn length(&mut self,u:usize,v:usize) -> i32 {
        let f = self.answer_to_query(u,v) as usize;
        self.depth[u] + self.depth[v] - 2 * self.depth[f]
    }*/

    fn dist(&mut self, u:usize,v:usize)-> i32 {
        let f = self.answer_to_query(u,v) as usize;
        self.costs[u] + self.costs[v] - 2 * self.costs[f]
    }
    
   /* fn is_on_path(&mut self,u:usize,v:usize,x:usize) -> bool {
        self.length(u,x) + self.length(v,x) == self.length(u,v)
    }*/
}
fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}
fn main(){
    let n:usize = read();
    let mut abc:Vec<Vec<i32>> = vec![vec![0;3];n];
    let mut lca = LCA::new(n);
    for i in 0..n - 1{
        abc[i] = (0..3).map(|_| read()).collect();
        lca.addedge(abc[i][0] as usize - 1, abc[i][1] as usize - 1, abc[i][2]);
    }
    lca.construct();
   let q:usize = read();
    let mut _q:Vec<Vec<usize>> = vec![vec![0;2];q];
    for i in 0..q{
        _q[i] = (0..2).map(|_| read()).collect();
    }
    for i in 0..q{
       println!("{}",lca.dist(_q[i][0] - 1,_q[i][1] - 1));
    }
}
