use std::prelude::v1::*;

use sgx_tunittest::*;

use super::*;

fn test_assert_matches_panics() {
    should_panic!(assert_matches_panics());
}

pub fn do_rsgx_test() {
    rsgx_unit_tests!(
        assert_matches_works,
        matches_works,
        test_assert_matches_panics
    );
}
