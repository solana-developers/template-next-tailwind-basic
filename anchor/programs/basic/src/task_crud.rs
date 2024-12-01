use anchor_lang::prelude::*;


pub fn create_todo(
    ctx: Context<CreateEntry>,
    title: String,
    description: String,
) -> Result<()> {
    msg!("Todo Created!");
    msg!("Title: {}", title);
    msg!("Description: {}", description);

    let todo_entry = &mut ctx.accounts.todo_entry;
    todo_entry.owner = ctx.accounts.owner.key();
    todo_entry.title = title;
    todo_entry.description = description;

    Ok(())
}

pub fn update_todo(
    ctx: Context<UpdateEntry>,
    new_title: String,
    new_description: String,
) -> Result<()> {

    // output for client
    msg!("Todo Updated!");
    msg!("Title: {}", title);
    msg!("Description: {}", description);

    let todo_entry = &mut ctx.accounts.todo_entry;
    todo_entry.title = new_title;
    todo_entry.description = new_description;

    Ok(())
}

pub fn delete_todo(
    ctx: Context<DeleteTodo>
) -> Result<()> {

    let todo_entry = &mut ctx.accounts.todo_entry;

    // ensure the owner is authorized to delete
    require!(
        todo_entry.owner == ctx.accounts.owner.key(),
        CustomError::Unauthorized
    );

    // reset the specified todo_entry account data
    todo_entry.title.clear();
    todo_entry.description.clear();
    todo_entry.deleted = true; // mark as deleted

    // logging the action
    msg!("Todo entry data cleared. Title: {}", todo_entry.title);

    Ok(())
}



#[account]
pub struct Todo {
    pub owner: Pubkey,
    pub title: String,
    pub description: String,
    pub deleted: bool,
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateTodo<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
        payer = owner,
        space = 8 + 32 + 4 + title.len() + 4 + description.len()
    )]
    pub todo_entry: Account<'info, Todo>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct UpdateEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
        realloc = 8 + 32 + 4 + title.len() + 4 + description.len(), // using realloc to resize the account in runtime
        realloc::payer = owner, 
        realloc::zero = true, 
    )]
    pub todo_entry: Account<'info, Todo>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    #[account( 
        mut, 
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
    )]
    pub todo_entry: Account<'info, Todo>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// error handling
#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to delete this entry.")]
    Unauthorized,
}
