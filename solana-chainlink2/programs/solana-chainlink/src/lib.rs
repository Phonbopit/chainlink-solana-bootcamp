use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

use chainlink_solana as chainlink;

declare_id!("9GE1ZiqMLXccgJK73vTpBizCFB1Q35P5rCgsHxMVm6or");

#[account]
pub struct Decimal {
    pub value: i128,
    pub decimals: u32,
}

impl Decimal {
    pub fn new(value: i128, decimals: u32) -> Self {
        Decimal { value, decimals }
    }
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut scaled_val = self.value.to_string();
        if scaled_val.len() <= self.decimals as usize {
            scaled_val.insert_str(
                0,
                &vec!["0"; self.decimals as usize - scaled_val.len()].join(""),
            );
            scaled_val.insert_str(0, "0.");
        } else {
            scaled_val.insert(scaled_val.len() - self.decimals as usize, '.');
        }
        f.write_str(&scaled_val)
    }
}

#[program]
pub mod solana_chainlink {
    use super::*;
    pub fn execute(ctx: Context<Execute>) -> ProgramResult {
        let round = chainlink::latest_round_data(
            ctx.accounts.chainlink_program.to_account_info(),
            ctx.accounts.chainlink_feed.to_account_info(),
        )?;

        let description = chainlink::description(
            ctx.accounts.chainlink_program.to_account_info(),
            ctx.accounts.chainlink_feed.to_account_info(),
        )?;

        let decimals = chainlink::decimals(
            ctx.accounts.chainlink_program.to_account_info(),
            ctx.accounts.chainlink_feed.to_account_info(),
        )?;

        let round2 = chainlink::latest_round_data(
            ctx.accounts.chainlink_program.to_account_info(),
            ctx.accounts.chainlink_feed2.to_account_info(),
        )?;

        let description2 = chainlink::description(
            ctx.accounts.chainlink_program.to_account_info(),
            ctx.accounts.chainlink_feed2.to_account_info(),
        )?;

        let decimals2 = chainlink::decimals(
            ctx.accounts.chainlink_program.to_account_info(),
            ctx.accounts.chainlink_feed2.to_account_info(),
        )?;

        // Set the account value
        let decimal: &mut Account<Decimal> = &mut ctx.accounts.decimal;
        decimal.value = round.answer;
        decimal.decimals = u32::from(decimals);

        // Also print the value to the program output
        let decimal_print = Decimal::new(round.answer, u32::from(decimals));
        let decimal_print2 = Decimal::new(round2.answer, u32::from(decimals2));
        msg!("feed 1 {} price is {}", description, decimal_print);
        msg!("feed 2 {} price is {}", description2, decimal_print2);

        let new_price = decimal_print2.value as f64 / decimal_print.value as f64;

        let pair1 = description.split("/").next();
        let pair2 = description2.split("/").next();

        msg!("new price {}", new_price);
        msg!("final price of {:?}/{:?} is {}", pair1, pair2, new_price);

        // set to account for final feed/feed pair price.
        let decimal2: &mut Account<Decimal> = &mut ctx.accounts.decimal;
        // TODO: a bit tricky, will find the better solution later :)
        decimal2.value = (new_price * 100000000.0) as i128;
        decimal2.decimals = 8;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(init, payer = user, space = 100)]
    pub decimal: Account<'info, Decimal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub chainlink_feed: AccountInfo<'info>,
    pub chainlink_feed2: AccountInfo<'info>,
    pub chainlink_program: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}
