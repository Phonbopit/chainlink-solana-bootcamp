use anchor_lang::prelude::*;

declare_id!("2HZrGbQpkWmUtj4Ed2k7XeJtkb9LWsBXsADwAD8C945u");

#[program]
pub mod gm_anchor {
    use super::*;
    pub fn execute(ctx: Context<Execute>, name: String) -> ProgramResult {
        let gm_account = &mut ctx.accounts.gm_account;

        gm_account.name = name;
        gm_account.counter = 1;
        msg!("GM {}", gm_account.name);
        Ok(())
    }

    pub fn greeting(ctx: Context<GreetingCount>, name: String) -> ProgramResult {
        let gm_account: &mut Account<GreetingAccount> = &mut ctx.accounts.greeting_account;

        gm_account.name = name;
        gm_account.counter += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub gm_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GreetingCount<'info> {
    #[account(mut)]
    pub greeting_account: Account<'info, GreetingAccount>,
    pub user: Signer<'info>,
}

#[account]
#[derive(Debug)]
pub struct GreetingAccount {
    pub name: String,
    pub counter: u64,
}
