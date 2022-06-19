# emporium

## Overview

Discord daily bot that uses fungible tokens for coins and non fungible tokens for the shop on the IC

## Commands

### `register <principal id>`

- user registers their principal id for their discord name

### `daily`

- users can build a streak
- can call this method once per 18 hrs

### `work`

- users can work every 1 hr
- the closer to hr between calls can net more tokens
- need to define a curve for that

### `shop`

- display items for sale

### `buy <item>`

- purchase item in shop
- this triggers a nft mint via dip721v2 canister

## Flow

![flowchart](https://user-images.githubusercontent.com/8976745/173971159-3f5bcb99-d714-4326-b8b0-69794daacebc.png)

- bot receives command, calls respective method on emporium

### Canisters:

#### Emporium

- Main canister.
- Implements dip20 interface and holds token state
- holds user data state and grants rewards to local token state
-

#### Rewards

- Dip721 canister

Options:

- shop purchases are minted to users
- shop items are minted before hand under the emporium canister id, and transferred to users
