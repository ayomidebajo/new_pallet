use crate::{mock::*, *};
use frame_support::{assert_noop, assert_ok, traits::Currency, BoundedVec};
use frame_system::Origin;
use sp_core::ConstU32;

#[test]
fn it_works() {
    new_test_ext().execute_with(|| assert!(true));
}

// type BalanceError = pallet_balances::Error<Test>;
// type AssetError = pallet_assets::Error<Test>;
pub const FIRST_POST_ID: u32 = 1;
pub const MAX_COMMENTS: u32 = 2000;
pub const TITLE_MAX_TEXT_LIMIT: u32 = 1000;
pub const BODY_MAX_TEXT_LIMIT: u32 = 10000;
pub const COMMENT_MAX_TEXT_LIMIT: u32 = 1000;
pub const TEXT_LIMIT: u32 = 1000;
pub const MIN_TEXT_CONTENT: u32 = 1;
pub const MAX_TEXT_CONTENT: u32 = 10000;

// helper fn for creating tokens
fn create_tokens<T: crate::Config>() {
    Balances::make_free_balance_be(&1, 100);

    assert!(News::create_token(RuntimeOrigin::signed(1), 8, 10).is_ok());
}

// helper fn for creating tokens and transferring tokens
fn transfer_tokens<T: crate::Config>() {
    create_tokens::<T>();

    Balances::make_free_balance_be(&2, 1);
    assert_ok!(News::transfer_tokens(RuntimeOrigin::signed(1), 8, 2, 5));
}

#[test]
fn token_creation_works() {
    new_test_ext().execute_with(|| {
        assert!(TokenCreators::<Test>::get(1).is_none());

        <Test as crate::Config>::Currency::make_free_balance_be(&1, 100);
        // Balances::make_free_balance_be(&1, 100);
        assert_ok!(News::create_token(RuntimeOrigin::signed(1), 6, 10));
        assert_eq!(Balances::reserved_balance(&1), 100);
        assert!(TokenCreators::<Test>::get(1).is_some());
    });
}

#[test]
fn token_creation_cant_work_without_balance() {
    new_test_ext().execute_with(|| {
        assert!(TokenCreators::<Test>::get(1).is_none());

        assert!(News::create_token(RuntimeOrigin::signed(1), 8, 1000).is_err());

        assert!(TokenCreators::<Test>::get(1).is_none());
    });
}

#[test]
fn token_transfer_works() {
    new_test_ext().execute_with(|| {
        assert!(TokenCreators::<Test>::get(1).is_none());

        Balances::make_free_balance_be(&1, 100);

        assert!(News::create_token(RuntimeOrigin::signed(1), 8, 10).is_ok());

        Balances::make_free_balance_be(&2, 1);
        assert_ok!(News::transfer_tokens(RuntimeOrigin::signed(1), 8, 2, 5));
    });
}

#[test]
fn create_post_works() {
    new_test_ext().execute_with(|| {
        create_tokens::<Test>();
        assert!(TokenCreators::<Test>::get(1).is_some());
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_ok!(News::create_post(
            RuntimeOrigin::signed(1),
            title,
            url,
            body
        ));
    });
}

#[test]
fn create_post_doesnt_work_for_non_token_creator() {
    new_test_ext().execute_with(|| {
        // create_tokens::<Test>();
        // assert!(TokenCreators::<Test>::get(1).is_some());
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_noop!(
            News::create_post(RuntimeOrigin::signed(1), title, url, body),
            Error::<Test>::InvalidSigner
        );
    });
}

#[test]
fn create_comment_works() {
    new_test_ext().execute_with(|| {
        // create token and transfer
        transfer_tokens::<Test>();
        // assert token creator
        assert!(TokenCreators::<Test>::get(1).is_some());
        // create post
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_ok!(News::create_post(
            RuntimeOrigin::signed(1),
            title,
            url,
            body
        ));

        // create comment
        let body: BoundedVec<u32, ConstU32<TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];

        let post_id = PostsStore::<Test>::iter_keys().next().unwrap();
        assert_ok!(News::create_comment(
            RuntimeOrigin::signed(2),
            post_id,
            body
        ));
    });
}

#[test]
fn create_comment_doesnt_work_for_non_token_holder() {
    new_test_ext().execute_with(|| {
        // create_tokens::<Test>();
        // assert!(TokenCreators::<Test>::get(1).is_some());
        // create token and transfer
        transfer_tokens::<Test>();
        // assert token creator
        assert!(TokenCreators::<Test>::get(1).is_some());
        // create post
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_ok!(News::create_post(
            RuntimeOrigin::signed(1),
            title,
            url,
            body
        ));

        // create comment
        let body: BoundedVec<u32, ConstU32<TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];

        let post_id = PostsStore::<Test>::iter_keys().next().unwrap();
        assert_noop!(
            News::create_comment(RuntimeOrigin::signed(3), post_id, body),
            Error::<Test>::InvalidSigner
        );
    });
}

#[test]
fn upvote_post_works() {
    new_test_ext().execute_with(|| {
        // creates and transfer tokens to account => 2
        transfer_tokens::<Test>();
        assert!(TokenCreators::<Test>::get(1).is_some());
        assert!(TokenHolders::<Test>::get(2).is_some());
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_ok!(News::create_post(
            RuntimeOrigin::signed(1),
            title,
            url,
            body
        ));

        let post_id = PostsStore::<Test>::iter_keys().next().unwrap();

        assert_ok!(News::upvote_post(RuntimeOrigin::signed(1), post_id));
    })
}

#[test]
fn upvote_post_doesnt_work_for_non_token_creators() {
    new_test_ext().execute_with(|| {
        // creates and transfer tokens to account => 2
        transfer_tokens::<Test>();
        assert!(TokenCreators::<Test>::get(1).is_some());
        assert!(TokenHolders::<Test>::get(2).is_some());
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_ok!(News::create_post(
            RuntimeOrigin::signed(1),
            title,
            url,
            body
        ));

        let post_id = PostsStore::<Test>::iter_keys().next().unwrap();

        assert_noop!(
            News::upvote_post(RuntimeOrigin::signed(3), post_id),
            Error::<Test>::InvalidSigner
        );
    })
}

#[test]
fn upvote_comment_works() {
    new_test_ext().execute_with(|| {
        // create token and transfer
        transfer_tokens::<Test>();
        // assert token creator
        assert!(TokenCreators::<Test>::get(1).is_some());
        // create post
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_ok!(News::create_post(
            RuntimeOrigin::signed(1),
            title,
            url,
            body
        ));

        let post_id = PostsStore::<Test>::iter_keys().next().unwrap();

        // create comment
        let body: BoundedVec<u32, ConstU32<TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];

        assert_ok!(News::create_comment(
            RuntimeOrigin::signed(2),
            post_id,
            body
        ));

        let comment_id = CommentStore::<Test>::iter_keys().next().unwrap();

        assert_ok!(News::upvote_comment(RuntimeOrigin::signed(2), comment_id));
    })
}

#[test]
fn upvote_comment_doesnt_work_for_non_token_holders() {
    new_test_ext().execute_with(|| {
        // create token and transfer
        transfer_tokens::<Test>();
        // assert token creator
        assert!(TokenCreators::<Test>::get(1).is_some());
        // create post
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_ok!(News::create_post(
            RuntimeOrigin::signed(1),
            title,
            url,
            body
        ));

        let post_id = PostsStore::<Test>::iter_keys().next().unwrap();

        // create comment
        let body: BoundedVec<u32, ConstU32<TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];

        assert_ok!(News::create_comment(
            RuntimeOrigin::signed(2),
            post_id,
            body
        ));

        let comment_id = CommentStore::<Test>::iter_keys().next().unwrap();

        assert_noop!(
            News::upvote_comment(RuntimeOrigin::signed(4), comment_id),
            Error::<Test>::InvalidSigner
        );
    })
}

#[test]
fn update_post_works() {
    new_test_ext().execute_with(|| {
        create_tokens::<Test>();
        assert!(TokenCreators::<Test>::get(1).is_some());
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_ok!(News::create_post(
            RuntimeOrigin::signed(1),
            title,
            url,
            body
        ));

        let post_id = PostsStore::<Test>::iter_keys().next().unwrap();
        let new_content: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 102];

        assert_ok!(News::update_post(
            RuntimeOrigin::signed(1),
            post_id,
            new_content
        ));
    });
}

#[test]
fn update_post_doesnt_work_for_non_token_creator() {
    new_test_ext().execute_with(|| {
        create_tokens::<Test>();
        assert!(TokenCreators::<Test>::get(1).is_some());
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_ok!(News::create_post(
            RuntimeOrigin::signed(1),
            title,
            url,
            body
        ));

        let post_id = PostsStore::<Test>::iter_keys().next().unwrap();
        let new_content: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 102];

        assert_noop!(
            News::update_post(RuntimeOrigin::signed(2), post_id, new_content),
            Error::<Test>::InvalidSigner
        );
    });
}

#[test]
fn update_comment_works() {
    new_test_ext().execute_with(|| {
        transfer_tokens::<Test>();
        assert!(TokenCreators::<Test>::get(1).is_some());
        assert!(TokenHolders::<Test>::get(2).is_some());

        // create post
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_ok!(News::create_post(
            RuntimeOrigin::signed(1),
            title,
            url,
            body
        ));

        let post_id = PostsStore::<Test>::iter_keys().next().unwrap();

        // create comment
        let body: BoundedVec<u32, ConstU32<TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];

        assert_ok!(News::create_comment(
            RuntimeOrigin::signed(2),
            post_id,
            body
        ));

        // update comment
        let comment_id = CommentStore::<Test>::iter_keys().next().unwrap();
        let new_content: BoundedVec<u32, ConstU32<TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 102];

        assert_ok!(News::update_comment(
            RuntimeOrigin::signed(2),
            comment_id,
            new_content
        ));
    });
}

#[test]
fn update_comment_doesnt_work_for_non_token_holder() {
    new_test_ext().execute_with(|| {
        transfer_tokens::<Test>();
        assert!(TokenCreators::<Test>::get(1).is_some());
        assert!(TokenHolders::<Test>::get(2).is_some());

        // create post
        let title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 6, 6];
        let url: BoundedVec<u32, ConstU32<TEXT_LIMIT>> = frame_support::bounded_vec![3, 4, 6, 6];
        let body: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];
        assert_ok!(News::create_post(
            RuntimeOrigin::signed(1),
            title,
            url,
            body
        ));

        let post_id = PostsStore::<Test>::iter_keys().next().unwrap();

        // create comment
        let body: BoundedVec<u32, ConstU32<TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 6];

        assert_ok!(News::create_comment(
            RuntimeOrigin::signed(2),
            post_id,
            body
        ));

        // update comment
        let comment_id = CommentStore::<Test>::iter_keys().next().unwrap();
        let new_content: BoundedVec<u32, ConstU32<TEXT_LIMIT>> =
            frame_support::bounded_vec![3, 4, 100, 200, 101, 100, 103, 6, 102];

        assert_noop!(
            News::update_comment(RuntimeOrigin::signed(1), comment_id, new_content),
            Error::<Test>::InvalidSigner
        );
    });
}
