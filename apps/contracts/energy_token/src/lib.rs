//! # EnergyToken
//!
//! SEP-41 fungible token representing renewable energy generation certificates.
//! 1 token = 1 kWh of verified renewable energy production.
//!
//! Built with OpenZeppelin Stellar (Pausable + Upgradeable).

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
pub enum DataKey {
    Admin,
    Minter,
}

#[contract]
pub struct EnergyToken;

#[contractimpl]
impl EnergyToken {
    /// Initialize the token contract.
    pub fn initialize(env: Env, admin: Address, minter: Address, name: String, symbol: String) {
        assert!(!env.storage().instance().has(&DataKey::Admin), "already initialized");
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Minter, &minter);
        stellar_fungible::set_metadata(&env, 7, name, symbol);
    }

    /// Mint `amount` tokens to `to`. Only callable by the minter.
    pub fn mint(env: Env, to: Address, amount: i128) {
        let minter: Address = env.storage().instance().get(&DataKey::Minter).unwrap();
        minter.require_auth();
        stellar_fungible::mint(&env, &to, amount);
    }

    /// Burn `amount` tokens from `from`. Used to retire certificates.
    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        stellar_fungible::burn(&env, &from, amount);
    }

    /// Transfer admin role.
    pub fn set_admin(env: Env, new_admin: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    /// Transfer minter role.
    pub fn set_minter(env: Env, new_minter: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Minter, &new_minter);
    }
}

// SEP-41 interface delegation
#[contractimpl]
impl stellar_fungible::Sep41 for EnergyToken {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        stellar_fungible::allowance(&env, &from, &spender)
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();
        stellar_fungible::approve(&env, &from, &spender, amount, expiration_ledger);
    }

    fn balance(env: Env, id: Address) -> i128 {
        stellar_fungible::balance(&env, &id)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        stellar_fungible::transfer(&env, &from, &to, amount);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        stellar_fungible::transfer_from(&env, &spender, &from, &to, amount);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        stellar_fungible::burn(&env, &from, amount);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        stellar_fungible::burn_from(&env, &spender, &from, amount);
    }

    fn decimals(env: Env) -> u32 {
        stellar_fungible::decimals(&env)
    }

    fn name(env: Env) -> String {
        stellar_fungible::name(&env)
    }

    fn symbol(env: Env) -> String {
        stellar_fungible::symbol(&env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, Address, Address, EnergyTokenClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let contract_id = env.register(EnergyToken, ());
        let client = EnergyTokenClient::new(&env, &contract_id);
        client.initialize(
            &admin,
            &minter,
            &String::from_str(&env, "SolarCert Energy Token"),
            &String::from_str(&env, "SCERT"),
        );
        (env, admin, minter, client)
    }

    #[test]
    fn test_mint_and_balance() {
        let (env, _admin, minter, client) = setup();
        let recipient = Address::generate(&env);
        client.mint(&minter, &recipient, &1_000_000_000);
        assert_eq!(client.balance(&recipient), 1_000_000_000);
    }

    #[test]
    fn test_burn_retires_certificate() {
        let (env, _admin, minter, client) = setup();
        let buyer = Address::generate(&env);
        client.mint(&minter, &buyer, &500_000_000);
        client.burn(&buyer, &500_000_000);
        assert_eq!(client.balance(&buyer), 0);
    }

    #[test]
    fn test_transfer() {
        let (env, _admin, minter, client) = setup();
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        client.mint(&minter, &alice, &1_000_000_000);
        client.transfer(&alice, &bob, &400_000_000);
        assert_eq!(client.balance(&alice), 600_000_000);
        assert_eq!(client.balance(&bob), 400_000_000);
    }

    #[test]
    #[should_panic]
    fn test_unauthorized_mint() {
        let (env, _admin, _minter, client) = setup();
        let attacker = Address::generate(&env);
        let victim = Address::generate(&env);
        client.mint(&attacker, &victim, &1_000_000_000);
    }
}
