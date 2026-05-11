use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    AmountTooLow = 3,
    SlippageExceeded = 4,
    InsufficientLiquidity = 5,
    InvalidPool = 6,
    SwapFailed = 7,
    Unauthorized = 8,
    MathOverflow = 9,
    InvalidRoute = 10,
}
