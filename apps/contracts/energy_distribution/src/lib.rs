//! # EnergyDistribution
//!
//! Allocates energy certificates to cooperative members proportionally
//! based on their participation (meter contribution share).

#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, map, Address, Env, Map, Vec,
};

#[contracttype]
pub enum DataKey {
    Admin,
    TokenContract,
    Shares,
}

#[contract]
pub struct EnergyDistribution;

#[contractimpl]
impl EnergyDistribution {
    pub fn initialize(env: Env, admin: Address, token_contract: Address) {
        assert!(!env.storage().instance().has(&DataKey::Admin), "already initialized");
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenContract, &token_contract);
        env.storage()
            .instance()
            .set(&DataKey::Shares, &map![&env] as &Map<Address, u32>);
    }

    /// Set participation shares for members. Shares are relative weights (e.g. 50 + 50 = equal split).
    pub fn set_shares(env: Env, members: Vec<Address>, shares: Vec<u32>) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        assert_eq!(members.len(), shares.len(), "length mismatch");

        let mut share_map: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&DataKey::Shares)
            .unwrap_or(map![&env]);

        for i in 0..members.len() {
            share_map.set(members.get(i).unwrap(), shares.get(i).unwrap());
        }
        env.storage().instance().set(&DataKey::Shares, &share_map);
    }

    /// Distribute `total_amount` tokens proportionally to all members with shares.
    pub fn distribute(env: Env, total_amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let share_map: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&DataKey::Shares)
            .unwrap_or(map![&env]);

        let total_shares: u32 = share_map.values().iter().sum();
        assert!(total_shares > 0, "no shares set");

        let token: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenContract)
            .unwrap();

        for (member, share) in share_map.iter() {
            let member_amount = total_amount * (share as i128) / (total_shares as i128);
            if member_amount > 0 {
                let client = energy_token::EnergyTokenClient::new(&env, &token);
                client.transfer(&admin, &member, &member_amount);
            }
        }
    }

    pub fn get_share(env: Env, member: Address) -> u32 {
        let share_map: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&DataKey::Shares)
            .unwrap_or(map![&env]);
        share_map.get(member).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, vec, Env};

    #[test]
    fn test_set_and_get_shares() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let contract_id = env.register(EnergyDistribution, ());
        let client = EnergyDistributionClient::new(&env, &contract_id);
        client.initialize(&admin, &token);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        client.set_shares(&admin, &vec![&env, alice.clone(), bob.clone()], &vec![&env, 60u32, 40u32]);

        assert_eq!(client.get_share(&alice), 60);
        assert_eq!(client.get_share(&bob), 40);
    }
}
