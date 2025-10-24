use anchor_lang::prelude::*;

declare_id!("FLWc42CeaZmCvhy8GUnukYgcSMpF73hkqwqkg8p5BL9h"); 

#[program]
pub mod visitor_counter {
    use super::*;

    // Initialize the visitor counter
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = *ctx.accounts.authority.key;
        counter.count = 0;
        Ok(())
    }

    // Increment the visitor counter
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_add(1).ok_or(ErrorCode::CounterOverflow)?;
        emit!(CounterIncremented {
            new_count: counter.count,
            visitor: *ctx.accounts.visitor.key,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init, 
        payer = authority, 
        space = 8 + 32 + 8)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub visitor: Signer<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}

#[event]
pub struct CounterIncremented {
    pub new_count: u64,
    pub visitor: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Counter Overflowed")]
    CounterOverflow,
}
