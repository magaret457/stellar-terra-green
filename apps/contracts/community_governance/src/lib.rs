//! # CommunityGovernance
//!
//! On-chain governance for SolarCert cooperatives.
//! Members submit proposals; admin executes approved ones.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone)]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
    Executed,
}

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub proposer: Address,
    pub title: String,
    pub description: String,
    pub votes_for: u32,
    pub votes_against: u32,
    pub status: ProposalStatus,
}

#[contracttype]
pub enum DataKey {
    Admin,
    ProposalCount,
    Proposal(u32),
    Voted(u32, Address),
}

#[contract]
pub struct CommunityGovernance;

#[contractimpl]
impl CommunityGovernance {
    pub fn initialize(env: Env, admin: Address) {
        assert!(!env.storage().instance().has(&DataKey::Admin), "already initialized");
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::ProposalCount, &0u32);
    }

    pub fn propose(env: Env, proposer: Address, title: String, description: String) -> u32 {
        proposer.require_auth();
        let count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0);
        let id = count + 1;

        let proposal = Proposal {
            id,
            proposer,
            title,
            description,
            votes_for: 0,
            votes_against: 0,
            status: ProposalStatus::Active,
        };

        env.storage().instance().set(&DataKey::Proposal(id), &proposal);
        env.storage().instance().set(&DataKey::ProposalCount, &id);
        id
    }

    pub fn vote(env: Env, voter: Address, proposal_id: u32, approve: bool) {
        voter.require_auth();

        assert!(
            !env.storage()
                .instance()
                .has(&DataKey::Voted(proposal_id, voter.clone())),
            "already voted"
        );

        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .expect("proposal not found");

        assert!(
            matches!(proposal.status, ProposalStatus::Active),
            "proposal not active"
        );

        if approve {
            proposal.votes_for += 1;
        } else {
            proposal.votes_against += 1;
        }

        env.storage()
            .instance()
            .set(&DataKey::Voted(proposal_id, voter), &true);
        env.storage()
            .instance()
            .set(&DataKey::Proposal(proposal_id), &proposal);
    }

    pub fn finalize(env: Env, proposal_id: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .expect("proposal not found");

        proposal.status = if proposal.votes_for > proposal.votes_against {
            ProposalStatus::Approved
        } else {
            ProposalStatus::Rejected
        };

        env.storage()
            .instance()
            .set(&DataKey::Proposal(proposal_id), &proposal);
    }

    pub fn get_proposal(env: Env, proposal_id: u32) -> Proposal {
        env.storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .expect("proposal not found")
    }

    pub fn proposal_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, Address, CommunityGovernanceClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let contract_id = env.register(CommunityGovernance, ());
        let client = CommunityGovernanceClient::new(&env, &contract_id);
        client.initialize(&admin);
        (env, admin, client)
    }

    #[test]
    fn test_propose_and_vote() {
        let (env, admin, client) = setup();
        let member = Address::generate(&env);

        let id = client.propose(
            &member,
            &String::from_str(&env, "Add new meter"),
            &String::from_str(&env, "Install meter at location X"),
        );
        assert_eq!(id, 1);

        client.vote(&member, &1, &true);
        client.finalize(&admin, &1);

        let proposal = client.get_proposal(&1);
        assert!(matches!(proposal.status, ProposalStatus::Approved));
    }

    #[test]
    fn test_rejected_proposal() {
        let (env, admin, client) = setup();
        let member = Address::generate(&env);

        client.propose(
            &member,
            &String::from_str(&env, "Bad idea"),
            &String::from_str(&env, "This should be rejected"),
        );

        client.vote(&member, &1, &false);
        client.finalize(&admin, &1);

        let proposal = client.get_proposal(&1);
        assert!(matches!(proposal.status, ProposalStatus::Rejected));
    }

    #[test]
    #[should_panic(expected = "already voted")]
    fn test_double_vote_panics() {
        let (env, _admin, client) = setup();
        let member = Address::generate(&env);

        client.propose(
            &member,
            &String::from_str(&env, "Test"),
            &String::from_str(&env, "Test"),
        );
        client.vote(&member, &1, &true);
        client.vote(&member, &1, &true); // should panic
    }
}
