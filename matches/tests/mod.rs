use std::prelude::v1::*;

use testings::*;

use super::*;

fn test_assert_matches_panics() {
    should_panic!(
        assert_matches_panics(),
        "assertion failed: `Some(\"-AB\")` does not match "
    );
}

pub fn do_rsgx_tests() -> usize {
    run_tests!(
        assert_matches_works,
        matches_works,
        test_assert_matches_panics
    )
}
