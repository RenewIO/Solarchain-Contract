#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, Env, String};

#[contracttype]
pub enum DataKey {
    CampaignCounter,
    CampaignOwner(u32),
    CampaignGoal(u32),
    CampaignBalance(u32),
    CampaignDescription(u32),
}

pub struct SolarChainContract;

#[contractimpl]
impl SolarChainContract {
    pub fn init(env: Env) {
        if !env.storage().has(&DataKey::CampaignCounter) {
            env.storage().set(&DataKey::CampaignCounter, &0u32);
        }
    }

    pub fn create_campaign(env: Env, owner: Address, goal: i128, description: String) -> u32 {
        let counter: u32 = env.storage().get(&DataKey::CampaignCounter).unwrap_or(0u32);
        let campaign_id = counter + 1;
        env.storage().set(&DataKey::CampaignCounter, &campaign_id);
        env.storage().set(&DataKey::CampaignOwner(campaign_id), &owner);
        env.storage().set(&DataKey::CampaignGoal(campaign_id), &goal);
        env.storage().set(&DataKey::CampaignBalance(campaign_id), &0i128);
        env.storage().set(&DataKey::CampaignDescription(campaign_id), &description);
        campaign_id
    }

    pub fn donate(env: Env, campaign_id: u32, amount: i128) {
        let balance: i128 = env.storage().get(&DataKey::CampaignBalance(campaign_id)).unwrap_or(0i128);
        let new_balance = balance + amount;
        env.storage().set(&DataKey::CampaignBalance(campaign_id), &new_balance);
    }

    pub fn get_campaign_balance(env: Env, campaign_id: u32) -> i128 {
        env.storage().get(&DataKey::CampaignBalance(campaign_id)).unwrap_or(0i128)
    }

    pub fn get_campaign_goal(env: Env, campaign_id: u32) -> i128 {
        env.storage().get(&DataKey::CampaignGoal(campaign_id)).unwrap_or(0i128)
    }

    pub fn get_campaign_description(env: Env, campaign_id: u32) -> String {
        env.storage()
            .get(&DataKey::CampaignDescription(campaign_id))
            .unwrap_or(String::new(&env))
    }
}
