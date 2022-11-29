use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{
    env,
    log,
    require,
    near_bindgen,
    AccountId,
    BorshStorageKey,
    assert_self
};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKeys {
    RegistryKey,
    AllAliasesKey,
    SourcesKey,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CoretoDID {
    registry: LookupMap<AccountId, String>,
    mapped_definitions: LookupMap<String, String>,
    sources: LookupMap<AccountId, String>,
}

impl Default for CoretoDID {
    fn default() -> Self {
        Self {
            registry: LookupMap::new(StorageKeys::RegistryKey),
            mapped_definitions: LookupMap::new(StorageKeys::AllAliasesKey),
            sources: LookupMap::new(StorageKeys::SourcesKey),
        }
    }
}

#[near_bindgen]
impl CoretoDID {
    // DID
    pub fn get_did(&self, account_id: AccountId) -> Option<String> {
        require!(env::is_valid_account_id(account_id.as_bytes()), "Not a valid NEAR account");

        return self.registry.get(&account_id);
    }

    pub fn has_did(&self, account_id: AccountId) -> bool {
        return self.registry.contains_key(&account_id);
    }

    pub fn put_did(&mut self, account_id: AccountId, did: String) -> bool {
		require!(
			self.sources.contains_key(&env::signer_account_id()),
			"Invalid signer wallet."
		);

        self.registry.insert(&account_id, &did);
        log!("Registered DID {} - {}", account_id, did);

        return true;
    }

    pub fn transfer_did(&mut self, old_account_id: AccountId, new_account_id: AccountId) -> bool {
		require!(
			self.sources.contains_key(&env::signer_account_id()),
			"Invalid signer wallet."
		);

        require!(
			self.registry.contains_key(&old_account_id),
			"DID not found."
		);

        require!(
			!self.registry.contains_key(&new_account_id),
			"AccountId already have DID associated."
		);

        let did = self.registry.get(&old_account_id).unwrap();

        self.registry.remove(&old_account_id);
        self.registry.insert(&new_account_id, &did);

        log!("Transferred DID {} from {} tp {}", did, old_account_id, new_account_id);

        return true;
    }

    // Sources
	pub fn add_source(&mut self, source: AccountId, source_label: String) {
		assert_self();
		require!(
			!self.sources.contains_key(&source),
			"Source already exists."
		);

		self.sources.insert(&source, &source_label);
	}

	pub fn remove_source(&mut self, source: AccountId) {
		assert_self();
		require!(
			self.sources.contains_key(&source),
			"Source not found."
		);

		self.sources.remove(&source);
	}

    // Alias - Definition Tracking
    pub fn retrieve_definition(&self, alias: String) -> Option<String> {
		require!(self.mapped_definitions.contains_key(&alias), "Definition not found for this alias.");

        return self.mapped_definitions.get(&alias);
    }

    pub fn has_alias_definition(&self, alias: String) -> bool {
        return self.mapped_definitions.contains_key(&alias);
    }

    pub fn store_definition(&mut self, alias: String, definition: String) -> bool {
        require!(!self.mapped_definitions.contains_key(&alias), "Definition already exists.");

        self.mapped_definitions.insert(&alias, &definition);
        log!("Stored definition for alias {} - {}", alias, definition);

        return true;
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};

    use super::*;

    // Allows for modifying the environment of the mocked blockchain
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    #[should_panic(expected = r#"Invalid signer wallet."#)]
    fn transfer_did_no_sources() {
        let context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoDID::default();
        contract.transfer_did(
            accounts(2),
            accounts(3),
        );
    }

    #[test]
    #[should_panic(expected = r#"DID not found."#)]
    fn transfer_non_existent_did() {
        let mut context = get_context(accounts(0));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoDID::default();

        contract.add_source(
            accounts(1),
            "main_source".to_string(),
        );

        testing_env!(
            context
                .signer_account_id(accounts(1))
                .build()
        );

        contract.transfer_did(
            "francis.near".parse().unwrap(),
            accounts(2),
        );
    }

    #[test]
    #[should_panic(expected = r#"AccountId already have DID associated."#)]
    fn transfer_did_to_already_associated_account_id() {
        let mut context = get_context(accounts(0));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoDID::default();

        contract.add_source(
            accounts(1),
            "main_source".to_string(),
        );

        testing_env!(
            context
                .signer_account_id(accounts(1))
                .build()
        );

        let mocked_did2 = "did:mocked:accounts(2)";
        contract.put_did(
            accounts(2),
            mocked_did2.to_string()
        );
        assert_eq!(
            mocked_did2.to_string(),
            contract.get_did(accounts(2)).unwrap()
        );

        let mocked_did3 = "did:mocked:accounts(3)";
        contract.put_did(
            accounts(3),
            mocked_did3.to_string()
        );
        assert_eq!(
            mocked_did3.to_string(),
            contract.get_did(accounts(3)).unwrap()
        );

        contract.transfer_did(
            accounts(2),
            accounts(3),
        );
    }

    #[test]
    fn transfer_did() {
        let mut context = get_context(accounts(0));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoDID::default();

        contract.add_source(
            accounts(1),
            "main_source".to_string(),
        );

        testing_env!(
            context
                .signer_account_id(accounts(1))
                .build()
        );

        let mocked_did = "did:mocked:accounts(2)";
        contract.put_did(
            accounts(2),
            mocked_did.to_string()
        );
        assert_eq!(
            mocked_did.to_string(),
            contract.get_did(accounts(2)).unwrap()
        );

        contract.transfer_did(
            accounts(2),
            accounts(3),
        );
        assert_eq!(
            mocked_did.to_string(),
            contract.get_did(accounts(3)).unwrap()
        );
        assert_eq!(false, contract.has_did(accounts(2)));
    }

    #[test]
    #[should_panic(expected = r#"Invalid signer wallet."#)]
    fn put_did_no_sources() {
        let context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoDID::default();
        let mocked_did = "did:mocked:accounts(2)";
        contract.put_did(
            accounts(2),
            mocked_did.to_string()
        );
    }
    #[test]
    fn put_get_did() {
        let mut context = get_context(accounts(0));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoDID::default();

        contract.add_source(
            accounts(1),
            "main_source".to_string(),
        );

        testing_env!(
            context
                .signer_account_id(accounts(1))
                .build()
        );

        let mocked_did = "did:mocked:accounts(2)";
        contract.put_did(
            accounts(2),
            mocked_did.to_string()
        );
        assert_eq!(
            mocked_did.to_string(),
            contract.get_did(accounts(2)).unwrap()
        );
    }

    #[test]
    fn has_did() {
        let mut context = get_context(accounts(0));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        let mut contract = CoretoDID::default();

        contract.add_source(
            accounts(1),
            "main_source".to_string(),
        );

        testing_env!(
            context
                .signer_account_id(accounts(1))
                .build()
        );

        let mocked_did = "did:mocked:accounts(2)";
        contract.put_did(
            accounts(2),
            mocked_did.to_string()
        );
        assert_eq!(true, contract.has_did(accounts(2)));

        assert_eq!(false, contract.has_did("francis.near".parse().unwrap()));
    }

    #[test]
    fn get_nonexistent_did() {
        let contract = CoretoDID::default();
        assert_eq!(None, contract.get_did("francis.near".parse().unwrap()));
    }

    #[test]
    #[should_panic(expected = r#"Definition not found for this alias."#)]
    fn retrieve_nonexistent_definition() {
        let contract = CoretoDID::default();
        assert_eq!(None, contract.retrieve_definition("non_existing_alias".parse().unwrap()));
    }

    #[test]
    fn has_alias_definition() {
        let mut context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        // Set the testing environment for the subsequent calls
        testing_env!(
            context
                .predecessor_account_id(accounts(1))
                .build()
        );


        let mut contract = CoretoDID::default();

        let mocked_alias = "mocked_alias";
        let mocked_definition = "mocked_definition";
        contract.store_definition(mocked_alias.to_string(), mocked_definition.to_string());
        assert_eq!(true, contract.has_alias_definition(mocked_alias.to_string()));

        assert_eq!(false, contract.has_did("non_existing_alias".parse().unwrap()));
    }

    #[test]
    fn store_retrieve_definition() {
        let mut context = get_context(accounts(1));
        // Initialize the mocked blockchain
        testing_env!(context.build());

        // Set the testing environment for the subsequent calls
        testing_env!(
            context
                .predecessor_account_id(accounts(1))
                .build()
        );

        let mut contract = CoretoDID::default();
        let mocked_alias = "mocked_alias";
        let mocked_definition = "mocked_definition";
        contract.store_definition(mocked_alias.to_string(), mocked_definition.to_string());
        assert_eq!(
            mocked_definition.to_string(),
            contract.retrieve_definition(mocked_alias.to_string()).unwrap()
        );
    }
}
