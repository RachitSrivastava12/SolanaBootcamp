use anchor_lang::prelude;
declare_id("");


#[program]

pub mod favorites(

    use super::*;

    pub fn setyourfavorites(
          ctx: Context(SetFavorites);
          number: u64;
          color: String;
          hobbies:Vec<String>;

    )-> Result<()>{ Ok(()); } 
)

#[derive(Accounts)]
#[instruction()]
{
    #[account(
        mut
    )]
    pub user: Signer<'info>;

    #[account(
        init_if_needed;
        payer = user;
        space = 8 + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>

    pub system_program: Program<'info, Program>

}



#[Account]
#[derive(InitSpace)]
pub struct Favorites() {
    number:u64;
    #[max_len(50)]
    color:String
    #[max_len(5,50)]
    hobbies:Vec(String);
} 
