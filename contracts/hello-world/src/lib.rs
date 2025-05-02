#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, String, symbol_short, Vec, log};

#[contracttype]
#[derive(Clone)]
pub struct Plot {
    pub id: u64,
    pub owner: Address,
    pub crop_type: String,
    pub planted_at: u64,
}

#[contracttype]
pub enum GardenKey {
    Plot(u64),
    Count,
}

#[contract]
pub struct CommunityGarden;

#[contractimpl]
impl CommunityGarden {
    pub fn claim_plot(env: Env, owner: Address, crop_type: String) -> u64 {
        owner.require_auth();

        let mut count: u64 = env.storage().instance().get(&GardenKey::Count).unwrap_or(0);
        count += 1;

        let plot = Plot {
            id: count,
            owner: owner.clone(),
            crop_type,
            planted_at: env.ledger().timestamp(),
        };

        env.storage().instance().set(&GardenKey::Plot(count), &plot);
        env.storage().instance().set(&GardenKey::Count, &count);

        log!(&env, "Plot {} claimed by {}", count, owner);
        count
    }

    pub fn get_plot(env: Env, plot_id: u64) -> Plot {
        env.storage().instance().get(&GardenKey::Plot(plot_id)).expect("Plot not found")
    }

    pub fn get_plot_count(env: Env) -> u64 {
        env.storage().instance().get(&GardenKey::Count).unwrap_or(0)
    }
}
