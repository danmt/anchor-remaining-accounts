use anchor_lang::prelude::*;

declare_id!("DLjexKmTr6f9ZqLnYzvEVWzfH7b6dpFPjgdn5P7t2UeM");

#[program]
pub mod sandbox {
    use super::*;

    pub fn create_account(ctx: Context<CreateAccount>) -> ProgramResult {
        ctx.accounts.my_account.optional_account = None;
        Ok(())
    }

    pub fn create_account_with_remaining_account(ctx: Context<CreateAccount>) -> ProgramResult {
        let remaining_account = get_first_remaining_account(ctx.remaining_accounts)?;

        ctx.accounts.my_account.optional_account = Some(remaining_account.key());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAccount<'info> {
    #[account(init, space = 100, payer = authority)]
    my_account: Account<'info, MyAccount>,
    authority: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateAccountWithRemainingAccount<'info> {
    #[account(mut)]
    my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount { 
    optional_account: Option<Pubkey>
}


fn get_remaining_account_by_index<'info>(remaining_accounts: &[AccountInfo<'info>], index: usize) -> std::result::Result<Account<'info, MyAccount>, ProgramError> {
    let maybe_account: Option<&AccountInfo> = remaining_accounts.get(index);
    let maybe_decoded_account: Option<std::result::Result<Account<MyAccount>, ProgramError>> = maybe_account.map(Account::try_from);
    let remaining_account = match maybe_decoded_account {
        Some(Ok(account)) => account,
        Some(Err(_)) => return Err(ErrorCode::WrongFirstAccount.into()),
        None => return Err(ErrorCode::MissingFirstAccount.into()),
    };

    Ok(remaining_account)
}

fn get_first_remaining_account<'info>(remaining_accounts: &[AccountInfo<'info>])  -> std::result::Result<Account<'info, MyAccount>, ProgramError> {
    let remaining_account = get_remaining_account_by_index(remaining_accounts, 0)?;
    Ok(remaining_account)
}


#[error]
pub enum ErrorCode {
    #[msg("Missing First Account")]
    MissingFirstAccount,
    #[msg("Wrong First Account")]
    WrongFirstAccount,
}