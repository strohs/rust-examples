
#[derive(Debug)]
struct Elem {
    c: u8,
}

#[derive(Debug)]
struct Grid {
    g: Vec<Vec<Elem>>,
}

impl Grid {

    fn neighbor_indices(&self, r: usize, c: usize) -> Vec<(usize,usize)> {
        let mut ndxs = vec![];
        let max_rows = self.g.len();
        let max_cols = self.g[0].len();
        let rstart = if r <= 1 { 0 } else { r - 1 };
        let cstart = if c <= 1 { 0 } else { c - 1 };
        for nr in rstart..=(r + 1) {
            for nc in cstart..=(c + 1) {
                // push all the cells located around index: r,c  into the return vector
                if nr < max_rows && nc <= max_cols && (nr != r || nc != c) {
                    ndxs.push((nr, nc));
                }
            }
        }
        ndxs
    }

    fn neighbors(&mut self, r: usize, c: usize) -> Vec<&mut Elem> {
        let neighbor_ndxs = self.neighbor_indices(r,c);
        let mut neighbors = vec![];
        for (ri, row) in self.g.iter_mut().enumerate() {
            for (ci,elem) in row.iter_mut().enumerate() {
                if neighbor_ndxs.contains(&(ri,ci) ) {
                    neighbors.push(elem);
                }
            }
        }
        neighbors
    }

}

fn main() {
    let mut grid = Grid { g: vec![vec![Elem {c:0}, Elem {c:1}, Elem {c:2} ],
                                  vec![Elem {c:10}, Elem {c:11}, Elem {c:12} ],
                                  vec![Elem {c:20}, Elem {c:21}, Elem {c:22} ]] };
    
    println!("{:?}", &grid.g );

    let nbs = grid.neighbors(0,0);
    dbg!(nbs);

}