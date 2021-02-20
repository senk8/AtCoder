mod math;
mod data_structure;
mod algorithm;

use std::{collections::VecDeque, usize};

use proconio::input;

type Vz = Vec<i128>;

macro_rules! at {
    (vec:exp,idx:exp) => {
        $vec[$idx as usize]
    };
}

pub fn bfs()
{
    use std::collections::VecDeque;
    let dx:Vec<usize> = vec![0,1,0,1];
    let dy:Vec<usize> = vec![0,0,1,1];
    let mut que=VecDeque::new();
    let mut distance = vec![vec![-1;10];10];
    que.push_back((0,0));
    distance[0][0]=0;

    while let Some(v)=que.pop_front()
    {
       for i in 0..4
       {
           let x=v.0+dx[i];
           let y=v.1+dy[i];
           if x<0 || 50<=x {
               continue;
           }
           if y<0 || 50<=y {
               continue;
           }

           if distance[x][y]==-1
           {
               que.push_back((x,y));
               distance[x][y]=distance[v.0][v.1]+1;
           }
       }
    }
}


fn main() {
    input!{
        a:usize,
        b:usize,
    }

    dbg!(a)+dbg!(b);
    ()
}

