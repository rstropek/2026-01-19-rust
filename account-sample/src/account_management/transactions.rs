/// Represents a transaction that can be applied to bank accounts.
///
/// # Variants
///
/// * `CashDeposit` - Deposits cash into an account
/// * `Transfer` - Transfers money between two accounts
///
/// # Examples
///
/// ```
/// use accounts::account_management::transactions::Transaction;
///
/// // Create a cash deposit transaction
/// let deposit = Transaction::CashDeposit {
///     target_account: 1001,
///     amount: 250.0,
/// };
///
/// // Create a transfer transaction
/// let transfer = Transaction::Transfer {
///     source_account: 1001,
///     target_account: 1002,
///     amount: 100.0,
/// };
/// ```
pub enum Transaction {
    /// A cash deposit into a target account.
    ///
    /// # Fields
    ///
    /// * `target_account` - The account number to deposit into
    /// * `amount` - The amount to deposit (must be positive)
    CashDeposit {
        target_account: u32,
        amount: f32,
    },
    /// A transfer between two accounts.
    ///
    /// # Fields
    ///
    /// * `source_account` - The account number to transfer from
    /// * `target_account` - The account number to transfer to
    /// * `amount` - The amount to transfer (must be positive)
    #[allow(dead_code)]
    Transfer {
        source_account: u32,
        target_account: u32,
        amount: f32,
    },
}