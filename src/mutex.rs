use disjoint_sets::UnionFind;

pub fn compute_mutex_watershed_clustering(
    num_labels: usize,
    edges: &[(u32, u32, f64)],
    mutex_edges: &[(u32, u32, f64)]) -> UnionFind<u32> {

    let mut uf: UnionFind<u32> = UnionFind::new(num_labels);

    let num_edges = edges.len();
    let num_mutex = mutex_edges.len();

    let mut mutexes: Vec<Vec<u32>> = (0..num_labels).map(|_| Vec::new()).collect();
    
    let mut indices: Vec<usize> = (0..(num_edges + num_mutex)).collect();

    // helper to select edge weight
    let _ew = |i: usize| -> f64 {
        if i < num_edges {
            edges[i].2
        } else {
            mutex_edges[i - num_edges].2
        }
    };
    indices.sort_unstable_by(|i1, i2| _ew(*i1).partial_cmp(&_ew(*i2)).unwrap());
    indices.reverse();

    for edge_id in indices {
        let is_mutex_edge = edge_id >= num_edges;
        let actual_edge_id = if is_mutex_edge { edge_id - num_edges } else { edge_id };
        let (from, to, w) = if is_mutex_edge { mutex_edges[actual_edge_id] } else { edges[actual_edge_id] };

        if w.is_nan() { continue; }
        
        let (from_r, to_r) = (uf.find(from), uf.find(to));

        if from_r == to_r { continue; }
        if check_mutex(&mutexes, from_r as usize, to_r as usize) { continue; }

        if is_mutex_edge { insert_mutex_for_two_representatives(&mut mutexes, from_r as usize, to_r as usize, actual_edge_id as u32) }
        else {
            uf.union(from_r, to_r);
            if uf.find(from_r) == to_r { merge_mutexes(&mut mutexes[..], from_r as usize, to_r as usize); }
            else { merge_mutexes(&mut mutexes[..], to_r as usize, from_r as usize); }
        }
    }
        

            // for (edgeId in indices) {

            //     if (isMutexEdge)
            //         mutexStorage.insertMutex(rootFrom, rootTo, actualEdgeId.toLong())
            //     else {
            //         uf.join(rootFrom, rootTo)

            //         // Always only have mutexes for true roots!
            //         // If the newly assigned root == rootTo, merge into rootTo,
            //         // merge into rootFrom, otherwise
            //         if (uf.findRoot(rootFrom) == rootTo)
            //             mutexStorage.mergeMutexes(rootFrom, rootTo)
            //         else
            //             mutexStorage.mergeMutexes(rootTo, rootFrom)
            //     }
            // }

    // return uf

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
    let (l_from, l_into) = if r_from < r_into {
        let (l1, l2) = mutexes.split_at_mut(r_into);
        (&mut l1[r_from], &mut l2[0])
    } else {
        let (l2, l1) = mutexes.split_at_mut(r_from);
        (&mut l2[0], &mut l1[r_into])
    };
    

    let mut i_from = 0;
    let mut i_into = 0;
    while i_from < l_from.len() && i_into < l_into.len() {
        if l_from[i_from] < l_into[i_into] { l_into.insert(i_into, l_from[i_from]); i_from += 1 }
        else { i_into += 1; }
    }

    l_into.extend_from_slice(&l_from[i_from..]);
    mutexes[r_from] = Vec::with_capacity(0);
}

