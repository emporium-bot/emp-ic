.PHONY: init candid build local cap-local start-replica stop-replica test format lint clean dfx-clean

LOCAL_CUSTODIAN_PRINCIPAL = $(shell dfx identity get-principal)
CAP_ID ?= $(shell dfx canister id cap-router)

cap-local:
	# Verifying cap... $(shell [ -z "$(CAP_ID)" ] && dfx deploy cap-router)
	@echo "cap canister id: $(CAP_ID)"

init:
	git submodule update --init --recursive
	cargo check

candid:
	cargo run > candid/emporium.did

build: candid start-replica
	dfx canister create emporium
	dfx build emporium

local: candid start-replica cap-local
	dfx deploy emporium --argument '(opt record{custodians=opt vec{principal"$(LOCAL_CUSTODIAN_PRINCIPAL)"}; nft_canister=null; cap=opt principal"$(CAP_ID)"})'

start-replica:
	dfx ping local || dfx start --clean --background

stop-replica:
	dfx stop

format:
	cargo fmt --all

lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings -D clippy::all

clean: clean-dfx
	cargo clean

clean-dfx: stop-replica
	rm -rf .dfx cap/.dfx