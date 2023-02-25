#![cfg_attr(not(feature = "std"), no_std)]
pub mod mock;
pub mod test;

pub use pallet::*;
pub use scale_info;
// use sp_runtime::traits::;

pub const FIRST_POST_ID: u32 = 1;
pub const MAX_COMMENTS: u32 = 2000;
pub const TITLE_MAX_TEXT_LIMIT: u32 = 1000;
pub const BODY_MAX_TEXT_LIMIT: u32 = 10000;
pub const COMMENT_MAX_TEXT_LIMIT: u32 = 1000;
pub const TEXT_LIMIT: u32 = 1000;
pub const MIN_TEXT_CONTENT: u32 = 1;
pub const MAX_TEXT_CONTENT: u32 = 10000;

#[frame_support::pallet]
pub mod pallet {
    // use core::cmp::Ordering;

    // use frame_support::sp_runtime::traits::StaticLookup;
    use super::*;

    use frame_support::{
        dispatch::DispatchError,
        pallet_prelude::{ValueQuery, *},
        sp_runtime::traits::{Hash, StaticLookup},
        traits::{Currency, LockableCurrency, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;

    pub type BalanceOf<T> = <<T as pallet::Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;
    pub type PostId = u32;

    // type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_assets::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>
            + ReservableCurrency<Self::AccountId>
            + LockableCurrency<Self::AccountId>;

        type Bond: Get<BalanceOf<Self>>;
    }

    #[pallet::type_value]
    pub fn DefaultForNextPostId() -> PostId {
        FIRST_POST_ID
    }

    /// The next post id.
    #[pallet::storage]
    #[pallet::getter(fn next_post_id)]
    pub type NextPostId<T: Config> = StorageValue<_, PostId, ValueQuery, DefaultForNextPostId>;

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Eq, PartialEq, Clone, Default)]
    #[scale_info(skip_type_params(T))]
    pub struct Posts<T: Config> {
        user_address: T::AccountId,
        posttitle: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>>,
        posturl: BoundedVec<u32, ConstU32<TEXT_LIMIT>>,
        posttext: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>>,
        postvotes: u32,
        postcomments: u32,
        postask: bool,
        posthide: bool,
        postblocked: bool,
        token_id: T::AssetId,
        id: PostId,
        // pub created: WhoAndWhenOf<T>,
        // comments:
    }

    // create voting mechanisms for blocking posts, this votes can be made by token holders
    // token_id check assets pallet.

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Eq, PartialEq, Clone, Default)]
    #[scale_info(skip_type_params(T))]
    pub struct PostVotes<T: Config> {
        user_address: T::AccountId,
        post_id: T::Hash,
        upvote: bool,
        votecount: u32,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Eq, PartialEq, Clone, Default)]
    #[scale_info(skip_type_params(T))]
    pub struct PostComments<T: Config> {
        post_id: T::Hash,
        user_address: T::AccountId,
        comment_text: BoundedVec<u32, ConstU32<COMMENT_MAX_TEXT_LIMIT>>,
        comment_votes: u32,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Eq, PartialEq, Clone, Default)]
    #[scale_info(skip_type_params(T))]
    pub struct CommentVotes<T: Config> {
        comment_id: T::Hash,
        user_address: T::AccountId,
        upvote: bool,
        votecount: u32,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Eq, PartialEq, Clone, Default)]
    #[scale_info(skip_type_params(T))]
    pub struct TokenCreatorsConfig<T: Config> {
        tokens_balance: T::Balance,
        id: T::AssetId,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Debug, Eq, PartialEq, Clone, Default)]
    #[scale_info(skip_type_params(T))]
    pub struct TokenHoldersConfig<T: Config> {
        tokens_balance: T::Balance,
        id: T::AssetId,
    }

    #[pallet::storage]
    #[pallet::getter(fn token_creators)]
    pub(super) type TokenCreators<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, TokenCreatorsConfig<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn token_holders)]
    pub(super) type TokenHolders<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, TokenHoldersConfig<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn posts)]
    pub(super) type PostsStore<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, Posts<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn comments)]
    pub(super) type CommentStore<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, PostComments<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn commentsvotes)]
    pub(super) type CommentVotesStore<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, CommentVotes<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn postvotes)]
    pub(super) type PostVotesStore<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, PostVotes<T>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        PostCreated {
            title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>>,
            user_address: T::AccountId,
            token_id: T::AssetId,
        },
        PostEdited {
            content: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>>,
            user_address: T::AccountId,
        },
        PostCommentCreated {
            comment_text: BoundedVec<u32, ConstU32<COMMENT_MAX_TEXT_LIMIT>>,
            post_id: T::Hash,
            user_id: T::AccountId,
            token_id: T::AssetId,
        },
        PostCommentEdited {
            comment_text: BoundedVec<u32, frame_support::traits::ConstU32<1000>>,
            comment_id: T::Hash,
            user_id: T::AccountId,
        },
        UpvotePost {
            post_id: T::Hash,
            vote_count: u32,
        },
        UpvoteComment {
            comment_id: T::Hash,
            votecount: u32,
        },
        CreateTokens {
            user_id: T::AccountId,
            token_id: T::AssetId,
        },
        TransferTokens {
            user_id: T::AccountId,
            token_id: T::AssetId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        InvalidSigner,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        PostNotEnoughBytes,
        PostTooManyBytes,
        PostCommentNotEnoughBytes,
        PostCommentTooManyBytes,
        CantVoteTwice,
        PostNotFound,
        TipperIsAuthor,
        MaxCommentsDepthsReached,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn create_token(
            origin: OriginFor<T>,
            id: T::AssetId,
            amount: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;
            let admin = T::Lookup::unlookup(who.clone());

            // create token
            let created_assets = pallet_assets::Pallet::<T>::create(
                origin.clone(),
                id.clone().into(),
                admin.clone(),
                amount,
            );

            // mint it after creation into the token creator
            match pallet_assets::Pallet::<T>::mint(origin.clone(), id.clone().into(), admin, amount)
            {
                Ok(_) => {
                    match created_assets {
                        Ok(val) => {
                            let bal = pallet_assets::Pallet::<T>::balance(id.clone(), who.clone());

                            // dbg!(bal.clone());

                            let token_creator = TokenCreatorsConfig {
                                tokens_balance: bal,
                                id: id.clone(),
                            };

                            TokenCreators::<T>::insert(who.clone(), token_creator);
                            Self::deposit_event(Event::CreateTokens {
                                user_id: who,
                                token_id: id,
                            });
                            return Ok(val);
                            // return Ok(());
                            // Ok(());
                        }
                        Err(e) => return Err(e),
                    };
                    // return Ok(());
                }
                Err(e) => return Err(e),
            };
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn transfer_tokens(
            origin: OriginFor<T>,
            id: T::AssetId,
            account_id: T::AccountId,
            amount: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;
            let _admin = T::Lookup::unlookup(who.clone());
            let target = T::Lookup::unlookup(account_id.clone());

            let transfer_assets = pallet_assets::Pallet::<T>::transfer(
                origin.clone(),
                id.clone().into(),
                target,
                amount.clone(),
            );

            match transfer_assets {
                Ok(_) => {
                    let bal = pallet_assets::Pallet::<T>::balance(id.clone(), who.clone());

                    let token_holder: TokenHoldersConfig<T> = TokenHoldersConfig {
                        tokens_balance: bal,
                        id,
                    };
                    TokenHolders::<T>::insert(account_id, token_holder);
                    Self::deposit_event(Event::TransferTokens {
                        user_id: who,
                        token_id: id,
                    });
                    Ok(())
                }
                Err(e) => Err(e),
            }

            // Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn create_post(
            origin: OriginFor<T>,
            title: BoundedVec<u32, ConstU32<TITLE_MAX_TEXT_LIMIT>>,
            posturl: BoundedVec<u32, ConstU32<TEXT_LIMIT>>,
            content: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            // check if post is valid
            ensure!(
                (content.len() as u32) > MIN_TEXT_CONTENT,
                <Error<T>>::PostNotEnoughBytes
            );

            ensure!(
                (content.len() as u32) < MAX_TEXT_CONTENT,
                <Error<T>>::PostTooManyBytes
            );

            // check the signer is a token creator
            ensure!(
                <TokenCreators<T>>::get(who.clone()).is_some(),
                <Error<T>>::InvalidSigner
            );

            // create a new id, this is a counter that should increase by 1
            let id = Self::next_post_id();

            match <TokenCreators<T>>::get(who.clone()) {
                Some(val) => {
                    // post creation
                    let post: Posts<T> = Posts {
                        user_address: who.clone(),
                        posttitle: title.clone(),
                        posturl,
                        posttext: content.clone(),
                        postvotes: 0,
                        postcomments: 0,
                        postask: false,
                        posthide: false,
                        postblocked: false,
                        token_id: val.id.clone(),
                        id,
                    };

                    // post id
                    let post_id = T::Hashing::hash_of(&post);

                    PostsStore::insert(post_id, post);
                    NextPostId::<T>::mutate(|n| {
                        *n += 1;
                    });
                    Self::deposit_event(Event::PostCreated {
                        title,
                        user_address: who.clone(),
                        token_id: val.id,
                    });
                    return Ok(());
                }
                None => return Err(DispatchError::BadOrigin),
            }
        }

        #[pallet::call_index(3)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn create_comment(
            origin: OriginFor<T>,
            post_id: T::Hash,
            comment_text: BoundedVec<u32, ConstU32<COMMENT_MAX_TEXT_LIMIT>>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            // check the signer is a token holder
            ensure!(
                <TokenHolders<T>>::get(who.clone()).is_some(),
                <Error<T>>::InvalidSigner
            );

            // ensure post exists
            ensure!(
                <PostsStore<T>>::contains_key(post_id.clone()),
                <Error<T>>::PostNotFound
            );

            // check if comment is valid
            ensure!(
                (comment_text.len() as u32) > MIN_TEXT_CONTENT,
                <Error<T>>::PostNotEnoughBytes
            );

            ensure!(
                (comment_text.len() as u32) < MAX_TEXT_CONTENT,
                <Error<T>>::PostTooManyBytes
            );

            // check if the user creating comment is a token holder, then create comment
            match <TokenHolders<T>>::get(who.clone()) {
                Some(val) => {
                    // post creation
                    let create_comment = PostComments::<T> {
                        post_id,
                        user_address: who.clone(),
                        comment_text: comment_text.clone(),
                        comment_votes: 0,
                    };

                    // post id
                    let comment_id = T::Hashing::hash_of(&create_comment);

                    // push to storage
                    CommentStore::insert(comment_id, create_comment);

                    // emit event
                    Self::deposit_event(Event::PostCommentCreated {
                        comment_text,
                        post_id,
                        user_id: who,
                        token_id: val.id,
                    });
                    return Ok(());
                }
                None => return Err(DispatchError::BadOrigin),
            }
        }

        #[pallet::call_index(4)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn upvote_post(origin: OriginFor<T>, post_id: T::Hash) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // check if post exists
            ensure!(
                <PostsStore<T>>::get(post_id.clone()).is_some(),
                <Error<T>>::PostNotFound
            );

            // check the signer is a token holder
            ensure!(
                <TokenCreators<T>>::get(who.clone()).is_some(),
                <Error<T>>::InvalidSigner
            );

            // create vote
            let new_vote = PostVotes::<T> {
                user_address: who.clone(),
                post_id: post_id.clone(),
                upvote: true,
                votecount: 1,
            };

            // if post exists, retreive post and mutate the vote count
            PostsStore::<T>::mutate(post_id, |maybe_value: &mut Option<Posts<T>>| {
                if let Some(value) = maybe_value {
                    value.postvotes += 1;
                    // emit event
                    Self::deposit_event(Event::UpvotePost {
                        post_id,
                        vote_count: value.postvotes,
                    });
                }
            });

            // store vote
            let vote_id = T::Hashing::hash_of(&new_vote);
            <PostVotesStore<T>>::insert(vote_id, new_vote);

            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn upvote_comment(origin: OriginFor<T>, comment_id: T::Hash) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // check the signer is a token holder
            ensure!(
                <TokenHolders<T>>::get(who.clone()).is_some(),
                <Error<T>>::InvalidSigner
            );

            // enure comment exists
            ensure!(
                <CommentStore<T>>::contains_key(comment_id.clone()),
                <Error<T>>::PostNotFound
            );
            // ensure vote doesn't exist
            ensure!(
                !<CommentVotesStore<T>>::contains_key(comment_id.clone()),
                <Error<T>>::CantVoteTwice
            );

            // create vote
            let new_vote = CommentVotes::<T> {
                user_address: who.clone(),
                comment_id,
                votecount: 1,
                upvote: true,
            };

            // if comment exists, retreive comment and mutate the vote count
            CommentStore::<T>::mutate(comment_id, |maybe_value| {
                if let Some(value) = maybe_value {
                    value.comment_votes += new_vote.votecount;

                    // emit event
                    Self::deposit_event(Event::UpvoteComment {
                        comment_id,
                        votecount: value.comment_votes,
                    });
                }
            });

            // store vote
            let vote_id = T::Hashing::hash_of(&new_vote);
            <CommentVotesStore<T>>::insert(vote_id, new_vote);

            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn update_post(
            origin: OriginFor<T>,
            post_id: T::Hash,
            content: BoundedVec<u64, ConstU32<BODY_MAX_TEXT_LIMIT>>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            // check if post exists
            ensure!(
                <PostsStore<T>>::get(post_id.clone()).is_some(),
                <Error<T>>::PostNotFound
            );

            // check the signer is a token creator
            ensure!(
                <TokenCreators<T>>::get(who.clone()).is_some(),
                <Error<T>>::InvalidSigner
            );

            // check if postt is valid
            ensure!(
                (content.len() as u32) > MIN_TEXT_CONTENT,
                <Error<T>>::PostNotEnoughBytes
            );

            ensure!(
                (content.len() as u32) < MAX_TEXT_CONTENT,
                <Error<T>>::PostTooManyBytes
            );

            // check if post is blocked
            let old_post = <PostsStore<T>>::get(post_id.clone()).expect("Post not found!");

            if old_post.postblocked {
                return Err(DispatchError::Unavailable);
            }

            PostsStore::<T>::mutate(post_id, |maybe_value: &mut Option<Posts<T>>| {
                if let Some(value) = maybe_value {
                    value.posttext = content.clone();
                }
            });

            // emit event
            Self::deposit_event(Event::PostEdited {
                content,
                user_address: who.clone(),
            });

            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::call_index(7)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn update_comment(
            origin: OriginFor<T>,
            comment_id: T::Hash,
            content: BoundedVec<u32, frame_support::traits::ConstU32<1000>>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            // check if comment exists
            ensure!(
                <CommentStore<T>>::get(comment_id.clone()).is_some(),
                <Error<T>>::PostNotFound
            );

            // check the signer is a token holder
            ensure!(
                <TokenHolders<T>>::get(who.clone()).is_some(),
                <Error<T>>::InvalidSigner
            );

            // check if comment is valid
            ensure!(
                (content.len() as u32) > MIN_TEXT_CONTENT,
                <Error<T>>::PostNotEnoughBytes
            );

            ensure!(
                (content.len() as u32) < MAX_TEXT_CONTENT,
                <Error<T>>::PostTooManyBytes
            );

            CommentStore::<T>::mutate(comment_id, |maybe_value| {
                if let Some(value) = maybe_value {
                    value.comment_text = content.clone();
                }
            });

            // emit event
            Self::deposit_event(Event::PostCommentEdited {
                comment_text: content,
                comment_id,
                user_id: who.clone(),
            });

            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }
    }
}
