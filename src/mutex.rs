use disjoint_sets::UnionFind;
use log::debug;

pub fn compute_mutex_watershed_clustering(
    num_labels: usize,
    edges: &[(u32, u32, f64, bool)]) -> UnionFind<u32> {
    compute_mutex_watershed_clustering_with_callback(num_labels, edges, |_| {})
}

pub fn compute_mutex_watershed_clustering_with_callback<F>(
    num_labels: usize,
    edges: &[(u32, u32, f64, bool)],
    callback: F) -> UnionFind<u32>
where F: Fn(&UnionFind<u32>) -> () {

    let num_edges = edges.len();
    let mut indices: Vec<usize> = (0..num_edges).collect();

    debug!("Sorting {} indices", indices.len());
    indices.sort_unstable_by(|i1, i2| edges[*i2].2.partial_cmp(&edges[*i1].2).unwrap());

    let data = |idx: &usize| &edges[*idx];
    let index_edges = IndexArrayIter::new(&data, indices.iter());

    mutex_watershed_mst_cut_iter_with_callback(num_labels, index_edges, callback)

}

pub trait Edge {
    fn from(&self) -> u32;
    fn to(&self) -> u32;
    fn is_mutex_edge(&self) -> bool;
}

struct IndexArrayIter<'a, T: 'a, F: 'a + Fn(&usize) -> &'a T, I: Iterator<Item = &'a usize>> {
    data: &'a F,
    index_iterator: I
}

impl <'a, T: 'a, F: 'a + Fn(&usize) -> &'a T, I: Iterator<Item = &'a usize>> IndexArrayIter<'a, T, F, I> {
    pub fn new(data: &'a F, indices: I) -> Self {
        Self{
            data: data,
            index_iterator: indices
        }
    }
}

impl <'a, T: 'a, F: 'a + Fn(&usize) -> &'a T, I: Iterator<Item = &'a usize>> Iterator for IndexArrayIter<'a, T, F, I> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.index_iterator.next().map(self.data)
    }
}

impl Edge for (u32, u32, f64, bool) {
    fn from(&self) -> u32 {
        self.0
    }
    fn to(&self) -> u32 {
        self.1
    }
    fn is_mutex_edge(&self) -> bool {
        self.3
    }
}

impl Edge for (u32, u32, bool) {
    fn from(&self) -> u32 {
        self.0
    }
    fn to(&self) -> u32 {
        self.1
    }
    fn is_mutex_edge(&self) -> bool {
        self.2
    }
}

pub fn mutex_watershed_mst_cut_with_callback<F>(
    num_labels: usize,
    sorted_edges: &[impl Edge],
    callback: F) -> UnionFind<u32>
where F: Fn(&UnionFind<u32>) -> () {
    mutex_watershed_mst_cut_iter_with_callback(num_labels, sorted_edges.iter(), callback)
}

pub fn mutex_watershed_mst_cut_iter_with_callback<'a, E: 'a + Edge, I: Iterator<Item = &'a E>, F>(
    num_labels: usize,
    sorted_edges: I,
    callback: F) -> UnionFind<u32>
where F: Fn(&UnionFind<u32>) -> () {
    let mut uf: UnionFind<u32> = UnionFind::new(num_labels);
    let mut mutexes: Vec<Vec<u32>> = (0..num_labels).map(|_| Vec::new()).collect();

    for (edge_id, edge) in sorted_edges.enumerate() {
        let from = edge.from();
        let to = edge.to();
        let is_mutex_edge = edge.is_mutex_edge();
        let (from_r, to_r) = (uf.find(from), uf.find(to));

        if from_r == to_r { continue; }
        if check_mutex(&mutexes, from_r as usize, to_r as usize) { continue; }

        if is_mutex_edge { insert_mutex_for_two_representatives(&mut mutexes, from_r as usize, to_r as usize, edge_id as u32) }
        else {
            uf.union(from_r, to_r);
            if uf.find(from_r) == to_r { merge_mutexes(&mut mutexes[..], from_r as usize, to_r as usize); }
            else { merge_mutexes(&mut mutexes[..], to_r as usize, from_r as usize); }
        }
        callback(&uf)
    }
        

    uf
}

fn check_mutex(mutexes: &[Vec<u32>], r_one: usize, r_two: usize) -> bool {
    let l_one = &mutexes[r_one];
    let l_two = &mutexes[r_two];
    let mut i_one = 0;
    let mut i_two = 0;
    while i_one < l_one.len() && i_two < l_two.len() {
        if      l_one[i_one] < l_two[i_two] { i_one += 1; }
        else if l_two[i_two] < l_one[i_one] { i_two += 1; }
        else                                { return true; }
    }
    false
}

fn insert_mutex(mutexes: &mut [Vec<u32>], r: usize, edge_id: u32) {
    let index = mutexes[r].binary_search(&edge_id);
    if index.is_err() { mutexes[r].insert(index.unwrap_err(), edge_id); }
}

fn insert_mutex_for_two_representatives(mutexes: &mut [Vec<u32>], r_one: usize, r_two: usize, edge_id: u32) {
    insert_mutex(mutexes, r_one, edge_id);
    insert_mutex(mutexes, r_two, edge_id);
}

fn merge_mutexes(mutexes: &mut [Vec<u32>], r_from: usize, r_into: usize) {

    if r_from == r_into { return; }
    if mutexes[r_from].len() == 0 { return; }
    if mutexes[r_into].len() == 0 { mutexes.swap(r_from, r_into); return; }


    // this does not work, unfortunately:
    // let mut l_from = &mut mutexes[r_from];
    // let mut l_into = &mut mutexes[r_into];

    // try to get two mutable references of elements of the same slices
    // this seems rather complicated but compiles:
    // why do l_from, l_into not need to be mut?
    // let (l_from, l_into) = if r_from < r_into {
    //     let (l1, l2) = mutexes.split_at_mut(r_into);
    //     (&mut l1[r_from], &mut l2[0])
    // } else {
    //     let (l1, l2) = mutexes.split_at_mut(r_from);
    //     (&mut l2[0], &mut l1[r_into])
    // };

    // possibly the best solution to create my own function that uses unsafe block:
    let (l_from, l_into) = get_two_mutable_references(&mut mutexes[..], r_from, r_into);
    

    let mut i_from = 0;
    let mut i_into = 0;
    while i_from < l_from.len() && i_into < l_into.len() {
        if l_from[i_from] < l_into[i_into] { l_into.insert(i_into, l_from[i_from]); i_from += 1 }
        else { i_into += 1; }
    }

    l_into.extend_from_slice(&l_from[i_from..]);
    mutexes[r_from] = Vec::with_capacity(0);
}

fn get_two_mutable_references<T>(slice: &mut [T], index1: usize, index2: usize) -> (&mut T, &mut T) {
    let p1: *mut T = &mut slice[index1];
    let p2: *mut T = &mut slice[index2];
    unsafe {
        return (&mut *p1, &mut *p2);
    }
}

