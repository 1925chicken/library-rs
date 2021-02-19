pub mod lowlink {
    pub struct LowLink {
        k:u32,
        list:Vec<Vec<u32>>,
        used_e:Vec<Vec<bool>>,
        ord:Vec<u32>,
        lowlink:Vec<u32>,
        pub bridge:Vec<(u32,u32)>,//橋
        pub art:Vec<u32>,//関節点
        used_v:Vec<bool>,
    }
    impl LowLink {
        pub fn new(g:Vec<(usize,usize)>,dir:bool,v:usize) -> Self{
            let mut tmp = vec![Vec::new();v];
            for (from,to) in g {
                tmp[from].push(to as u32);
                if !dir {
                    tmp[to].push(from as u32);
                }
            }
            Self {
                k:0,
                list:tmp,
                used_v:vec![false;v],
                used_e:vec![vec![false;v];v],
                ord:vec![0;v],
                lowlink:vec![0;v],
                bridge:Vec::new(),
                art:Vec::new(),
            }
        }

        pub fn dfs(&mut self,par:Option<usize>,now:usize) {
            self.used_v[now] = true;
            self.ord[now] = self.k;
            self.lowlink[now] = self.k;
            self.k += 1;
            let mut cnt = 0;
            let mut _art = false;
            for i in 0..self.list[now].len() {
                if !self.used_v[self.list[now][i] as usize] {
                    self.used_e[now][self.list[now][i] as usize] = true;
                    cnt += 1;
                    self.dfs(Some(now),self.list[now][i] as usize);
                    self.lowlink[now] = std::cmp::min(self.lowlink[now],self.lowlink[self.list[now][i] as usize]);
                    if Some(par) != None {
                        _art = true;
                    }
                    if self.ord[now] < self.lowlink[self.list[now][i] as usize] {
                        self.bridge.push((std::cmp::min(now as u32,self.list[now][i]), std::cmp::max(now as u32, self.list[now][i])));
                    }
                }else if !self.used_e[self.list[now][i] as usize][now] {
                        self.lowlink[now] = std::cmp::min(self.lowlink[now],self.ord[self.list[now][i] as usize]);
                    }
                }
            if Some(par) == None && cnt >= 2 {
                _art = true;
            }
            if _art {
                self.art.push(now as u32);
            }
        }
    }
}
