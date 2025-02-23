use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("YourProgramID");

#[program]
pub mod savings {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        let savings_account = &mut ctx.accounts.savings_account;
        savings_account.balance = amount;
        savings_account.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let savings_account = &mut ctx.accounts.savings_account;
        savings_account.balance += amount;
        savings_account.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let savings_account = &mut ctx.accounts.savings_account;

        if savings_account.balance < amount {
            return Err(ErrorCode::InsufficientBalance.into());
        }

        savings_account.balance -= amount;
        savings_account.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
        let savings_account = &mut ctx.accounts.savings_account;
        let reward_rate: f64 = 0.05; // 5% annual rewards
        let time_passed = (Clock::get()?.unix_timestamp - savings_account.last_updated) as f64;
        let annual_seconds = 60.0 * 60.0 * 24.0 * 365.0;
        let rewards = (savings_account.balance as f64) * reward_rate * (time_passed / annual_seconds);

        savings_account.balance = (savings_account.balance as f64 + rewards) as u64;
        savings_account.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn reinvest(ctx: Context<Reinvest>) -> Result<()> {
        let savings_account = &mut ctx.accounts.savings_account;
        let reward_rate: f64 = 0.05; // 5% annual rewards
        let time_passed = (Clock::get()?.unix_timestamp - savings_account.last_updated) as f64;
        let annual_seconds = 60.0 * 60.0 * 24.0 * 365.0;
        let rewards = (savings_account.balance as f64) * reward_rate * (time_passed / annual_seconds);

        savings_account.balance = (savings_account.balance as f64 + rewards) as u64;
        savings_account.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[account]
pub struct SavingsAccount {
    pub owner: Pubkey,
    pub balance: u64,
    pub last_updated: i64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8)] // 8 bytes for discriminator, 32 for pubkey, 8 for balance and last updated timestamp
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Reinvest<'info> {
    #[account(mut)]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance to withdraw.")]
    InsufficientBalance,
}
