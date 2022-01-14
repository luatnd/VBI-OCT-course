// In tests.rs, start by importing the dependencies you'll need from lib.rs using super:
use super::*;

use crate::{mock::*, Error};
use frame_support::{
	assert_noop, assert_ok,

	sp_runtime::traits::Hash, // support T::Hashing
};


// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
// 		// Dispatch a signed extrinsic.
// 		assert_ok!(ProofOfExistence::do_something(Origin::signed(1), 42));
// 		// Read pallet storage and assert an expected result.
// 		assert_eq!(ProofOfExistence::something(), Some(42));
// 	});
// }
//
// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(ProofOfExistence::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }

// #[test]
// fn gen_id_should_return_a_value() {
// 	new_test_ext().execute_with(|| {
// 		// Dispatch a signed extrinsic.
// 		let s = ProofOfExistence::gen_unique_id();
// 		assert_ne!(s, Hash::hash(&b"N/A"[..]) as Hash);
// 	});
// }

#[test]
fn it_works_for_normal_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(ProofOfExistence::create_claim(Origin::signed(0), Some(b"this is phone number".to_vec()), Some(b"this is phone number".to_vec())));
		assert_ok!(ProofOfExistence::create_claim(Origin::signed(1), None, Some(b"this is phone number".to_vec())));
		assert_ok!(ProofOfExistence::create_claim(Origin::signed(1), Some(b"this is phone number".to_vec()), None));
	});
}

