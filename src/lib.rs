extern crate disjoint_sets;

pub mod mutex;

#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use disjoint_sets::UnionFind;

    #[test]
    fn mutex_ws() {
        let uf = super::mutex::compute_mutex_watershed_clustering(
            3,
            &vec![
                (0, 1, 1.0, false),
                (1, 2, 2.0, false),
                (0, 2, 1.9, true),
            ][..]
        );
        assert_ne!(uf.find(0), uf.find(1));
        assert_ne!(uf.find(0), uf.find(2));
        assert_eq!(uf.find(1), uf.find(2));
    }

    const EDGES_BASE: [(u32, u32, f64, bool); 7] = [
        (0, 1, 1.0, false),
        (1, 2, 1.0, false),
        (2, 3, 1.0, false),
        (4, 5, 1.0, false),
        (5, 6, 1.0, false),
        (6, 7, 1.0, false),
        (6, 8, 1.0, false)
        // add this edge to make single cluste:
        // (3, 4, 1.1, false),
        // add this edge to make two clusters instead of one:
        // (3, 4, 1.1, true),
    ];

    #[test]
    fn mutex_ws_variable_edge() {
        mutex_ws_for_edges(
            &EDGES_BASE,
            &[vec![0, 1, 2, 3], vec![4, 5, 6, 7, 8]]);

        let mut edges_without_mutex = EDGES_BASE.to_vec();
        edges_without_mutex.push((3, 4, 1.1, false));
        mutex_ws_for_edges(
            &edges_without_mutex[..],
            &[vec![0, 1, 2, 3, 4, 5, 6, 7, 8]]);

        let mut edges_with_mutex = EDGES_BASE.to_vec();
        edges_with_mutex.push((3, 4, 1.1, true));
        mutex_ws_for_edges(
            &edges_with_mutex[..],
            &[vec![0, 1, 2, 3], vec![4, 5, 6, 7, 8]]);

        // add edge between 3 and 5, to try to get around mutex edge

        let mut edges_without_mutex = EDGES_BASE.to_vec();
        edges_without_mutex.push((3, 5, 0.9, false));
        mutex_ws_for_edges(
            &edges_without_mutex[..],
            &[vec![0, 1, 2, 3, 4, 5, 6, 7, 8]]);

        let mut edges_with_mutex = EDGES_BASE.to_vec();
        edges_with_mutex.push((3, 4, 1.1, true));
        // value for 3-5 has to be smaller than all values within true cluters;
        // otherwise, it is not guaranteed that clusters are sufficiently populated
        // for mutex 3-4 to block 3-5
        edges_with_mutex.push((3, 5, 0.9, false));
        mutex_ws_for_edges(
            &edges_with_mutex[..],
            &[vec![0, 1, 2, 3], vec![4, 5, 6, 7, 8]]);
        
    }

    fn mutex_ws_for_edges(
        edges: &[(u32, u32, f64, bool)],
        expected: &[Vec<u32>]
    ) {
        let uf = super::mutex::compute_mutex_watershed_clustering(9, &edges);
        let actual = group_by_smallest_element(&uf);
        assert_eq!(actual, expected);
    }

    fn group_by_smallest_element(uf: &UnionFind<u32>) -> Vec<Vec<u32>> {
        let mut hm: HashMap<u32, Vec<u32>> = HashMap::new();
        for id in 0..uf.len() {
            let id_u32 = id as u32;
            let root = uf.find(id_u32);
            if !hm.contains_key(&root) {
                hm.insert(root, vec![id_u32]);
            } else {
                hm.get_mut(&root).unwrap().push(id_u32);
            }
        }

        let mut clusters: Vec<Vec<u32>> = hm.values().cloned().collect();
        for cluster in clusters.iter_mut() { cluster.sort(); }
        clusters.sort_by(|c1, c2| c1[0].cmp(&c2[0]));

        clusters
    }

    
}

