use std::collections::BinaryHeap;
use std::cmp::Reverse;
fn dijkstra(g:& Vec<Vec<(usize,u64)>>, dist:&mut Vec<u64>, start:usize) {
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0,start)));
    dist[start] = 0;
    while let Some(v) = pq.pop() {
        let (cost,now) = v.0;
        for (to,c) in &g[now] {
            if dist[now] < cost {
                continue;
            }
            if dist[*to] > *c + dist[now] {
                dist[*to] = dist[now] + *c;
                pq.push(Reverse((dist[*to],*to)));
            }
        }
    }
}

