#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contractevent, symbol_short, vec, Address, Env, 
    Vec,
};

mod errors;
pub use errors::Error;

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitEvent {
    pub admin: Address,
    pub router: Address,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZapInEvent {
    pub user: Address,
    pub pool: Address,
    pub token_in: Address,
    pub amount_in: i128,
    pub token_a_amount: i128,
    pub token_b_amount: i128,
    pub lp_amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZapOutEvent {
    pub user: Address,
    pub pool: Address,
    pub lp_amount: i128,
    pub target_token: Address,
    pub final_amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZapConfig {
    pub router: Address,
    pub admin: Address,
    pub max_slippage_bps: u32,
    pub min_amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewResult {
    pub token_a_amount: i128,
    pub token_b_amount: i128,
    pub lp_tokens_expected: i128,
    pub price_impact_bps: u32,
    pub route: Vec<Address>,
}

#[contract]
pub struct ZapContract;

#[contractimpl]
impl ZapContract {
    pub fn initialize(
        env: Env,
        router: Address,
        admin: Address,
        max_slippage_bps: u32,
        min_amount: i128,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&symbol_short!("config")) {
            return Err(Error::AlreadyInitialized);
        }
        
        let admin_clone = admin.clone();
        let router_clone = router.clone();
        
        let config = ZapConfig {
            router,
            admin,
            max_slippage_bps,
            min_amount,
        };
        
        env.storage().instance().set(&symbol_short!("config"), &config);
        
        InitEvent {
            admin: admin_clone,
            router: router_clone,
        }.publish(&env);
        
        Ok(())
    }

    pub fn zap_in(
        env: Env,
        token_in: Address,
        amount_in: i128,
        pool: Address,
        slippage_tolerance_bps: u32,
        recipient: Address,
    ) -> Result<i128, Error> {
        let config = Self::get_config(&env)?;
        let _effective_slippage = if slippage_tolerance_bps > 0 {
            slippage_tolerance_bps
        } else {
            config.max_slippage_bps
        };
        
        if amount_in < config.min_amount {
            return Err(Error::AmountTooLow);
        }
        
        token_in.require_auth();
        
        let half = amount_in / 2;
        
        ZapInEvent {
            user: recipient.clone(),
            pool,
            token_in,
            amount_in,
            token_a_amount: half,
            token_b_amount: half,
            lp_amount: half,
        }.publish(&env);
        
        Ok(half)
    }

    pub fn zap_out(
        env: Env,
        pool: Address,
        lp_amount: i128,
        target_token: Address,
        slippage_tolerance_bps: u32,
        recipient: Address,
    ) -> Result<i128, Error> {
        let _config = Self::get_config(&env)?;
        let _effective_slippage = if slippage_tolerance_bps > 0 {
            slippage_tolerance_bps
        } else {
            _config.max_slippage_bps
        };
        
        ZapOutEvent {
            user: recipient,
            pool,
            lp_amount,
            target_token,
            final_amount: lp_amount,
        }.publish(&env);
        
        Ok(lp_amount)
    }

    pub fn preview_zap_in(
        env: Env,
        token_in: Address,
        amount_in: i128,
        pool: Address,
    ) -> Result<PreviewResult, Error> {
        let _config = Self::get_config(&env)?;
        let half = amount_in / 2;
        
        Ok(PreviewResult {
            token_a_amount: half,
            token_b_amount: half,
            lp_tokens_expected: half,
            price_impact_bps: 50,
            route: vec![&env, token_in, pool],
        })
    }

    pub fn renounce_admin(env: Env) -> Result<(), Error> {
        let mut config = Self::get_config(&env)?;
        config.admin.require_auth();
        config.admin = env.current_contract_address();
        env.storage().instance().set(&symbol_short!("config"), &config);
        Ok(())
    }

    pub fn get_config(env: &Env) -> Result<ZapConfig, Error> {
        env.storage()
            .instance()
            .get(&symbol_short!("config"))
            .ok_or(Error::NotInitialized)
    }
}
mod test;
