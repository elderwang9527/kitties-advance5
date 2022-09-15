use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_success() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
		assert_noop!(KittiesModule::transfer(Origin::signed(2), 0, 1), Error::<Test>::NotOwner);
	});
}

#[test]
fn create_failed_token_not_enough() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		// assert_ok!(KittiesModule::create(Origin::signed(11)));
		// assert_eq!(KittiesModule::next_kitty_id(), 1);
		assert_noop!(KittiesModule::create(Origin::signed(11)), Error::<Test>::TokenNotEnough);
	});
}

#[test]
fn transfer_success() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
	    assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::transfer(Origin::signed(1), 0, 2));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
	});
}


#[test]
fn transfer_failed_not_owner() {
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = 1;
		let account_id_2: u64 = 2;
		let account_id_3: u64 = 3;
		let kitty_id = 0u32;
		// 创建Kitty
		assert_ok!(KittiesModule::create(Origin::signed(account_id_1)));
		assert_noop!(
			KittiesModule::transfer(Origin::signed(account_id_2), kitty_id, account_id_3),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn breed_success() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;

		let kitty_id_1 = 0u32;
		let kitty_id_2 = 1u32;
		let kitty_id_3: u32 = 2u32;

		// 创建Kitty
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		// 繁殖
		assert_ok!(KittiesModule::breed(Origin::signed(account_id), kitty_id_1, kitty_id_2));
		// 检查拥有者
		assert_eq!(KittiesModule::kitty_owner(kitty_id_3), Some(account_id));
	});
}

#[test]
fn breed_failed_token_not_enough() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;

		let kitty_id_1 = 0u32;
		let kitty_id_2 = 1u32;

		// 创建Kitty
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		// 繁殖
		assert_noop!(
			(KittiesModule::breed(Origin::signed(account_id), kitty_id_1, kitty_id_2)),
			Error::<Test>::TokenNotEnough
		);
	});
}


#[test]
fn sell_success() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::sell(Origin::signed(1),0,Some(1000)));
	});
}

#[test]
fn sell_failed_not_owner() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(KittiesModule::sell(Origin::signed(2),0,Some(1000)),Error::<Test>::NotOwner);
	});
}

#[test]
fn buy_success() {
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = 1;
		let account_id_2: u64 = 2;
		let kitty_id = 0u32;
		let price = 1000u128;
		// 创建Kitty
		assert_ok!(KittiesModule::create(Origin::signed(account_id_1)));
		// 检查拥有者
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id_1));

		assert_ok!(KittiesModule::sell(Origin::signed(account_id_1), kitty_id, Some(price)));

		assert_ok!(KittiesModule::buy(Origin::signed(account_id_2), kitty_id));
		// 检查拥有者
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id_2));
	});
}

#[test]
fn buy_failed_token_not_enough() {
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = 1;
		let account_id_2: u64 = 12;
		let kitty_id = 0u32;
		let price = 1000u128;
		// 创建Kitty
		assert_ok!(KittiesModule::create(Origin::signed(account_id_1)));
		// 检查拥有者
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id_1));

		assert_ok!(KittiesModule::sell(Origin::signed(account_id_1), kitty_id, Some(price)));

		assert_noop!(KittiesModule::buy(Origin::signed(account_id_2), kitty_id),Error::<Test>::TokenNotEnough);
	});
}

#[test]
fn buy_failed_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = 1;
		let account_id_2: u64 = 12;
		let kitty_id = 0u32;
		let price = 1000u128;
		// 创建Kitty
		assert_ok!(KittiesModule::create(Origin::signed(account_id_1)));
		// 检查拥有者
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id_1));

		assert_ok!(KittiesModule::sell(Origin::signed(account_id_1), kitty_id, Some(price)));

		assert_noop!(KittiesModule::buy(Origin::signed(account_id_2), 2),Error::<Test>::InvalidKittyId);
	});
}

#[test]
fn buy_failed_buy_self() {
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = 1;
		let kitty_id = 0u32;
		let price = 1000u128;
		// 创建Kitty
		assert_ok!(KittiesModule::create(Origin::signed(account_id_1)));
		// 检查拥有者
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id_1));

		assert_ok!(KittiesModule::sell(Origin::signed(account_id_1), kitty_id, Some(price)));

		assert_noop!(KittiesModule::buy(Origin::signed(account_id_1), kitty_id),Error::<Test>::NoBuySelf);
	});
}


#[test]
fn buy_failed_not_for_sale() {
	new_test_ext().execute_with(|| {
		let account_id_1: u64 = 1;
		let account_id_2: u64 = 2;
		let kitty_id = 0u32;
		// 创建Kitty
		assert_ok!(KittiesModule::create(Origin::signed(account_id_1)));
		// 检查拥有者
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id_1));

		assert_noop!(KittiesModule::buy(Origin::signed(account_id_2), kitty_id),Error::<Test>::NotForSale);
	});
}
