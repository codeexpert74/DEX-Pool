// Data structures

#[account]
#[derive(Default)]
pub struct Pair {
    
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub lp_total_supply: u64,
    pub fee_bps: u16, // 0.3% = 30 bps
}
