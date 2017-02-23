type Index2d = (usize, usize);
type BoolMap = Vec<Vec<bool>>;
type RectangleBfs<'a> = Bfs<'a, Index2d, BoolMap, RectangleGraph>;

struct RectangleGraph {
    width: usize,
    height: usize,
    vertices: BoolMap,
}

impl RectangleGraph {
    fn new<N: Copy, F>(vect: &Vec<Vec<N>>, closure: F) -> RectangleGraph
        where F: Fn(N) -> bool {
        let width = vect[0].len();
        let height = vect.len();
        let mut vertices: BoolMap = vec![Vec::with_capacity(height); width];
        for y in 0..height {
             for x in 0..width {
                 vertices[x].push(closure(vect[y][x]));  // Swap x, y to make width 1st, height 2nd
            }
        }
        RectangleGraph{width: width, height: height, vertices: vertices}
    }
    #[inline]
    fn bstate(&self, x: usize, y: usize) -> bool {  // Panic-dangerous!
        self.vertices[x][y]
    }
    #[inline]
    fn ustate(&self, x: usize, y: usize) -> usize {  // Panic-dangerous!
        self.vertices[x][y] as usize
    }
    #[inline]
    fn degrees(&self, node: Index2d) -> Index2d { // Active and non-active
        let mut cnt: [usize; 2] = [0, 0];
        if node.0 + 1 != self.width {
            cnt[self.ustate(node.0 + 1, node.1)] += 1;
        }
        if node.1 + 1 != self.height {
            cnt[self.ustate(node.0, node.1 + 1)] += 1;
        }
        if node.0 != 0 {
            cnt[self.ustate(node.0 - 1, node.1)] += 1;
        }
        if node.1 != 0 {
            cnt[self.ustate(node.0, node.1 - 1)] += 1;
        }
        (cnt[1], cnt[0])
    }
    /// Correct usage:
    /// let comps: Vec<Vec<Index2d>>;
    /// let g: RectangleGraph = RectangleGraph::new(&lab, |x: char| x == '.');
    /// { comps = g.components(); }  // Note the curly brackets!!
    #[inline]
    fn components(&self) -> Vec<Vec<Index2d>> {
        RectangleBfs::new(&self).components()
    }
}

trait Graph<N: PartialEq> {
    fn neighbours(&self, node: N) -> Vec<N>;
    fn has_edge(&self, node1: N, node2: N) -> bool {
        for i in self.neighbours(node1){
            if i == node2 {
                return true;
            }
        }
        false
    }
}

impl Graph<Index2d> for RectangleGraph {
    fn neighbours(&self, node: Index2d) -> Vec<Index2d>{
        let mut ans : Vec<Index2d> = Vec::with_capacity(4);
        if node.0 + 1 != self.width && self.bstate(node.0 + 1, node.1){
            ans.push((node.0 + 1, node.1))
        }
        if node.1 + 1 != self.height && self.bstate(node.0, node.1 + 1){
            ans.push((node.0, node.1 + 1))
        }
        if node.0 != 0 && self.bstate(node.0 - 1, node.1){
            ans.push((node.0 - 1, node.1))
        }
        if node.1 != 0 && self.bstate(node.0, node.1 - 1){
            ans.push((node.0, node.1 - 1))
        }
        ans
    }
}

trait VisitMap<N>{
    /// Mark `a` as visited.
    ///
    /// Return **true** if this is the first visist, false otherwise.
    fn visit(&mut self, a: N) -> bool;

    /// Return whether `a` has been visited before.
    fn is_visited(&self, a: &N) -> bool;
}

impl VisitMap<Index2d> for BoolMap {
    #[inline]
    fn visit(&mut self, a: Index2d) -> bool {
        let ans = self[a.0][a.1];
        self[a.0][a.1] = true;
        ans
    }
    #[inline]
    fn is_visited(&self, a: &Index2d) -> bool {
        self[a.0][a.1]
    }

}

struct Bfs<'a, N, VM, G: 'a> {
    queue: VecDeque<N>,
    discovered: VM,
    graph: &'a G,
}

impl<'a> RectangleBfs<'a> {
    #[inline]
    fn new(graph: &'a RectangleGraph) -> RectangleBfs<'a> {
        Bfs{queue:      VecDeque::new(),
            discovered: vec![vec![false; MAXN]; MAXN],
            graph:      graph}
    }
    #[inline]
    fn spawn(&mut self, node: Index2d) {
        self.queue.push_back(node)
    }
    fn components(&mut self) -> Vec<Vec<Index2d>> {
        let mut ans: Vec<Vec<Index2d>> = Vec::new();
        self.queue.clear();
        for x in 0..self.graph.width {
            for y in 0..self.graph.height {
                if self.graph.vertices[x][y] && !self.discovered.visit((x, y)) {
                    self.spawn((x, y));
                    ans.push(self.collect());
                }
            }
        }
        ans
    }
}

impl<'a, N, VM, G> Iterator for Bfs<'a, N, VM, G>
    where N: Copy + PartialEq,
          VM: VisitMap<N>,
          G: Graph<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        while let Some(node) = self.queue.pop_front() {
            for succ in self.graph.neighbours(node) {
                if !self.discovered.visit(succ) {
                    self.queue.push_back(succ);
                }
            }
            return Some(node);
        }
        None
    }
}