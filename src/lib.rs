extern crate disjoint_sets;

pub mod mutex;

#[cfg(test)]
mod tests {

    #[test]
    fn mutex_ws() {
        let uf = super::mutex::compute_mutex_watershed_clustering(
            3,
            &vec![(0, 1, 1.0), (1, 2, 0.0)][..],
            &vec![(0, 2, 0.9)][..],
        );
        for i in 0..3 {
            println!("oke? {} -> {}", i, uf.find(i));
        }
    }
}

