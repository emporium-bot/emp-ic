# shopbot

## Overview

Discord daily coin bot that uses fungible and non fungible tokens on the IC

## Commands

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
