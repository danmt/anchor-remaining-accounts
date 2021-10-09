use anchor_lang::prelude::*;

declare_id!("ATAxWUNPqhcMwRHsALbdjB2ZPRn82JfJ1uvb6jSoad3A");

#[program]
pub mod basic {
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

type MaybeDecodedAccount<'info> = std::result::Result<Account<'info, MyAccount>, ProgramError>;

fn get_remaining_account_by_index<'info>(remaining_accounts: &[AccountInfo<'info>], index: usize) -> MaybeDecodedAccount<'info> {
    let maybe_account: Option<&AccountInfo> = remaining_accounts.get(index);
    let maybe_decoded_account: Option<MaybeDecodedAccount<'info>> = maybe_account.map(Account::try_from);
    match maybe_decoded_account {
        Some(Err(_)) => return Err(ErrorCode::WrongFirstAccount.into()),
        None => return Err(ErrorCode::MissingFirstAccount.into()),
        Some(account) => account
    }
}

fn get_first_remaining_account<'info>(remaining_accounts: &[AccountInfo<'info>])  -> MaybeDecodedAccount<'info> {
    get_remaining_account_by_index(remaining_accounts, 0)
}


#[error]
pub enum ErrorCode {
    #[msg("Missing First Account")]
    MissingFirstAccount,
    #[msg("Wrong First Account")]
    WrongFirstAccount,
}