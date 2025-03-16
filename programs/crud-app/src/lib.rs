use std::str;

use anchor_lang::prelude::*;

declare_id!("35NwwgDmNzoJECYNLMsYvidvUHSZU6YpuDEMZ9yjBBX7");

#[program]
pub mod crud_app {
    use super::*;
    pub fn create_journal_entry(
        ctx: Context<CreateEntry>,
        title: String,
        message: String,
    ) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = ctx.accounts.owner.key();
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }

    pub fn update_journal_entry(
        ctx: Context<UpdateEntry>,
        _title: String,
        message: String
    ) -> Result<()>{
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.message = message;

        Ok(())
    }

    pub fn delete_journal_entry(
        _ctx: Context<DeleteEntry>,
        _title: String,
    ) -> Result<()>{
        // delete logic already handled in DeletEentry data structure
        Ok(()) 
    }

    pub fn find_journal_entry(
        ctx: Context<FindEntry>,
        _title: String
    ) -> Result<()> {
        let journal_entry = &ctx.accounts.journal_entry;
        msg!("title is {}", journal_entry.title);
        msg!("message is {}", journal_entry.message);
        msg!("owner is {}", journal_entry.owner);

        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct FindEntry<'info>{
    // #[account()] no need for this because  account used to add contraints & validation rules for account as we are only reading the data so we can skip this
    pub owner: Signer<'info>,

    #[account(
        seeds=[title.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    // pub system_program: Program<'info, System> only require when we init or realloc the state
}


#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info>{
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        close = owner // this is logically handle delte operation if you run this instruction this will close the account and owner can only delete
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    // pub system_program: Program<'info, System> // realloc require system program object

}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateEntry<'info>{
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + JournalEntryState::INIT_SPACE, //realloc calculate the updating space so new space will be charged accordingly even return if less space taken
        realloc::payer = owner,
        realloc::zero = true // original space calculation back to zero and recalculate
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    pub system_program: Program<'info, System> // realloc require system program object
}


//in this createentry we need to pass all accounts that needs in instruction
#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init, // initialize the account on the chain
        payer = owner,
        seeds =[title.as_bytes(), owner.key().as_ref()], // it is used to define pda for account that is like a private key in db
        bump, //any time when we add seeds we need to add bump
        space = 8 + JournalEntryState::INIT_SPACE // 8 is anchor discriminated always have to add
    )]
    pub journal_entry: Account<'info, JournalEntryState>,

    pub system_program: Program<'info, System>, // init require system program
}

#[account] // State --> data that store on chain
#[derive(InitSpace)] // calculate data on state and based on the size we have to pay on chain
pub struct JournalEntryState {
    pub owner: Pubkey,
    #[max_len(50)] // restrict the size so we can calculate the size on chain
    pub title: String,
    #[max_len(200)]
    pub message: String,
}
