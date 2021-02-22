extern crate disjoint_sets;

pub mod mutex;

#[cfg(test)]
mod tests {

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
}

