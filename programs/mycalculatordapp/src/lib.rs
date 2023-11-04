use anchor_lang::prelude::*;

declare_id!("5qKAXAmKPxTaeYhMyKKB3vsbGNPSpvXK2nB83WJirC19");

#[program]
pub mod mycalculatordapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
