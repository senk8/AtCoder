struct UnionFind {
    par:Vec<usize>,
    ranks:Vec<usize>
}

impl UnionFind{
    pub fn new(n:usize)->UnionFind{
        UnionFind {
            par:(0..n).collect::<Vec<usize>>(),
            ranks:vec![1;n]
        }
    }

    pub fn root(&mut self,x:usize)->usize{
        if self.par[x]==x {
            x
        }else{
            let px = self.par[x];
            self.par[x] = self.root(px);
            self.par[x]
        }
    }

    pub fn unite(&mut self,x:usize,y:usize)->bool{
        let rx = self.root(x);
        let ry = self.root(y);

        if rx==ry{
            false
        }else{
            let (small,large) = if self.ranks[rx] < self.ranks[ry] {
                (rx,ry)
            }else{
                (ry,rx)
            };

            self.par[small]=large;
            self.ranks[large]+=self.ranks[small];
            self.ranks[small]=0;

            true
        }
    }

    pub fn same(&mut self,x:usize,y:usize)->bool{
        let rx = self.root(x);
        let ry = self.root(y);
        rx==ry
    }
}