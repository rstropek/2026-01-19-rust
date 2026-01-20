pub enum Transaction {
    CashDeposit {
        target_account: u32,
        amount: f32,
    },
    #[allow(dead_code)]
    Transfer {
        source_account: u32,
        target_account: u32,
        amount: f32,
    },
}