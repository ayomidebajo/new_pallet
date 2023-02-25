# About this project

This project contains a custom pallet for creating news posts based on the following features.

- A user may list news records ✅
- A user may view individual news item with comments ✅
- The token creator may view individual news item for editing
- The token creator may create a new news record ✅
- The token creator may update news item details ✅
- A token holder may post a comment on a news record ✅
- A token holder may upvote a news item ✅
- A token holder may upvote a comment on a news record ✅

## Going through the news storages and Extrinsics

#### Storages

- ` pub(super) type TokenCreators<T: Config>` contains all token creators
- `pub(super) type TokenHolders<T: Config>` contains all token holders
- ` pub(super) type PostsStore<T: Config>` contains all post
- `pub(super) type CommentStore<T: Config>` contains all comments
- `pub(super) type CommentVotesStore<T: Config>` contains all votes casted for comments
- `pub(super) type PostVotesStore<T: Config>` contains all votes casted for posts

Note -> Each of these storages have `getters`

#### Extrinsics

- `pub fn create_token` creates tokens and mints them into the creator's account.
- `pub fn transfer_tokens` transfer tokens from the creator's account to a target
- `pub fn create_post` creates post (only by token creator)
- `pub fn create_comment` creates comment (only by token holder)
- `pub fn upvote_post` only by token creators
- `pub fn upvote_comment` only by token holders
- `pub fn update_post` only by token creators
- `pub fn update_comment` only by token holders

#### Tests and what they do

- `fn token_creation_works` tests for token creation
- `fn token_creation_cant_work_without_balance` test token creation can't work without a minimum balance that is also greater than the mint amount
- `fn token_transfer_works` transfer token works after token creation
- `fn create_post_works` create posts is only done by token creators
- `fn create_post_doesnt_work_for_non_token_creator` non token creators can't create post
- `fn create_comment_works` create commnets is only done by token holders
- `fn create_comment_doesnt_work_for_non_token_holder` non token holders can't create post
- `fn upvote_post_works` posts can be upvoted by token creators
- `fn upvote_comment_works` comments can be upvoted by token holders
- `fn upvote_post_doesnt_work_for_non_token_creators` non token creators can't upvote post
- `fn upvote_comment_doesnt_work_for_non_token_holders` non token holders can't upvote comment
- `fn update_post_works` posts can be updated by token creators
- `fn update_comment_works` comments can be updated by token holders
- `fn update_comment_doesnt_work_for_non_token_holder` non token holders can't update comment
- `fn update_post_doesnt_work_for_non_token_creator` non token creators can't update post

## Getting Started

Follow the steps below to get started with the Node Template, or get it up and running right from
your browser in just a few clicks using
the [Substrate Playground](https://docs.substrate.io/playground/) :hammer_and_wrench:

### Using Nix

Install [nix](https://nixos.org/) and optionally [direnv](https://github.com/direnv/direnv) and
[lorri](https://github.com/nix-community/lorri) for a fully plug and play experience for setting up
the development environment. To get all the correct dependencies activate direnv `direnv allow` and
lorri `lorri shell`.

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Tests

`cargo fix --lib -p news --tests`
