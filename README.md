# Smart Contract Functions
```rust
// account_id - Must be a valid near account
pub fn get_did(
  &self, account_id: AccountId
) -> Option<String>
```

```rust
// account_id - Must be a valid near account
pub fn has_did(
  &self,
  account_id: AccountId
) -> bool
```

```rust
// account_id - Must be a valid near account
// The signer wallet must be an approved source
pub fn put_did(
  &mut self,
  account_id: AccountId,
  did: String
) -> bool
```

```rust
// old_account_id - Must be a valid near account present in the contract
// new_account_id - Must be a valid near account not present in the contract
// The signer wallet must be an approved source
pub fn transfer_did(
  &mut self,
  old_account_id: AccountId,
  new_account_id: AccountId
) -> bool
```

```rust
// source - Must be a valid near account not present in the contract
// The signer wallet must be the deployer of the contract
pub fn add_source(
  &mut self,
  source: AccountId,
  source_label: String
)
```

```rust
// source - Must be a valid near account present in the contract
// The signer wallet must be the deployer of the contract
pub fn remove_source(
  &mut self,
  source: AccountId
)
```

```rust
pub fn retrieve_definition(
  &self,
  alias: String
) -> Option<String>
```

```rust
pub fn has_alias_definition(
  &self,
  alias: String
) -> bool
```

```rust
pub fn store_definition(
  &mut self,
  alias: String,
  definition: String
) -> bool
```

# Run tests

`cargo test -- --nocapture`

# Compile

`cargo build --target wasm32-unknown-unknown --release`

# Deploy (regular)

```
near login
near deploy --wasmFile target/wasm32-unknown-unknown/release/coreto_did.wasm --accountId YOUR_ACCOUNT_HERE
```

# Call

```
near call YOUR_ACCOUNT_HERE METHOD_NAME METHOD_ARGUMENTS --accountId YOUR_ACCOUNT_HERE
near view YOUR_ACCOUNT_HERE GETTER_METHOD_NAME METHOD_ARGUMENTS --accountId YOUR_ACCOUNT_HERE
```

# Deploy (dev account)
```
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/coreto_did.wasm --helperUrl https://near-contract-helper.onrender.com
source neardev/dev-account.env
echo $CONTRACT_NAME
```

# Call
```
near call $CONTRACT_NAME METHOD_NAME METHOD_ARGUMENTS --accountId $CONTRACT_NAME
near view $CONTRACT_NAME GETTER_METHOD_NAME METHOD_ARGUMENTS --accountId $CONTRACT_NAME
```
