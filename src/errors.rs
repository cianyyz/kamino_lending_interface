use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum KaminoLendingError {
    #[error("Market authority is invalid")]
    InvalidMarketAuthority = 6000,
    #[error("Market owner is invalid")]
    InvalidMarketOwner = 6001,
    #[error("Input account owner is not the program address")]
    InvalidAccountOwner = 6002,
    #[error("Input amount is invalid")]
    InvalidAmount = 6003,
    #[error("Input config value is invalid")]
    InvalidConfig = 6004,
    #[error("Input account must be a signer")]
    InvalidSigner = 6005,
    #[error("Invalid account input")]
    InvalidAccountInput = 6006,
    #[error("Math operation overflow")]
    MathOverflow = 6007,
    #[error("Insufficient liquidity available")]
    InsufficientLiquidity = 6008,
    #[error("Reserve state needs to be refreshed")]
    ReserveStale = 6009,
    #[error("Withdraw amount too small")]
    WithdrawTooSmall = 6010,
    #[error("Withdraw amount too large")]
    WithdrawTooLarge = 6011,
    #[error("Borrow amount too small to receive liquidity after fees")]
    BorrowTooSmall = 6012,
    #[error("Borrow amount too large for deposited collateral")]
    BorrowTooLarge = 6013,
    #[error("Repay amount too small to transfer liquidity")]
    RepayTooSmall = 6014,
    #[error("Liquidation amount too small to receive collateral")]
    LiquidationTooSmall = 6015,
    #[error("Cannot liquidate healthy obligations")]
    ObligationHealthy = 6016,
    #[error("Obligation state needs to be refreshed")]
    ObligationStale = 6017,
    #[error("Obligation reserve limit exceeded")]
    ObligationReserveLimit = 6018,
    #[error("Obligation owner is invalid")]
    InvalidObligationOwner = 6019,
    #[error("Obligation deposits are empty")]
    ObligationDepositsEmpty = 6020,
    #[error("Obligation borrows are empty")]
    ObligationBorrowsEmpty = 6021,
    #[error("Obligation deposits have zero value")]
    ObligationDepositsZero = 6022,
    #[error("Obligation borrows have zero value")]
    ObligationBorrowsZero = 6023,
    #[error("Invalid obligation collateral")]
    InvalidObligationCollateral = 6024,
    #[error("Invalid obligation liquidity")]
    InvalidObligationLiquidity = 6025,
    #[error("Obligation collateral is empty")]
    ObligationCollateralEmpty = 6026,
    #[error("Obligation liquidity is empty")]
    ObligationLiquidityEmpty = 6027,
    #[error("Interest rate is negative")]
    NegativeInterestRate = 6028,
    #[error("Input oracle config is invalid")]
    InvalidOracleConfig = 6029,
    #[error("Insufficient protocol fees to claim or no liquidity available")]
    InsufficientProtocolFeesToRedeem = 6030,
    #[error("No cpi flash borrows allowed")]
    FlashBorrowCpi = 6031,
    #[error("No corresponding repay found for flash borrow")]
    NoFlashRepayFound = 6032,
    #[error("Invalid repay found")]
    InvalidFlashRepay = 6033,
    #[error("No cpi flash repays allowed")]
    FlashRepayCpi = 6034,
    #[error("Multiple flash borrows not allowed in the same transaction")]
    MultipleFlashBorrows = 6035,
    #[error("Flash loans are disabled for this reserve")]
    FlashLoansDisabled = 6036,
    #[error("Switchboard error")]
    SwitchboardV2Error = 6037,
    #[error("Cannot deserialize the scope price account")]
    CouldNotDeserializeScope = 6038,
    #[error("Price too old")]
    PriceTooOld = 6039,
    #[error("Price too divergent from twap")]
    PriceTooDivergentFromTwap = 6040,
    #[error("Invalid twap price")]
    InvalidTwapPrice = 6041,
    #[error("Emergency mode is enabled")]
    GlobalEmergencyMode = 6042,
    #[error("Invalid lending market config")]
    InvalidFlag = 6043,
    #[error("Price is not valid")]
    PriceNotValid = 6044,
    #[error("Price is bigger than allowed by heuristic")]
    PriceIsBiggerThanHeuristic = 6045,
    #[error("Price lower than allowed by heuristic")]
    PriceIsLowerThanHeuristic = 6046,
    #[error("Price is zero")]
    PriceIsZero = 6047,
    #[error("Price confidence too wide")]
    PriceConfidenceTooWide = 6048,
    #[error("Conversion between integers failed")]
    IntegerOverflow = 6049,
    #[error("This reserve does not have a farm")]
    NoFarmForReserve = 6050,
    #[error("Wrong instruction at expected position")]
    IncorrectInstructionInPosition = 6051,
    #[error("No price found")]
    NoPriceFound = 6052,
    #[error(
        "Invalid Twap configuration: Twap is enabled but one of the enabled price doesn't have a twap"
    )]
    InvalidTwapConfig = 6053,
    #[error("Pyth price account does not match configuration")]
    InvalidPythPriceAccount = 6054,
    #[error("Switchboard account(s) do not match configuration")]
    InvalidSwitchboardAccount = 6055,
    #[error("Scope price account does not match configuration")]
    InvalidScopePriceAccount = 6056,
    #[error(
        "The obligation has one collateral with an LTV set to 0. Withdraw it before withdrawing other collaterals"
    )]
    ObligationCollateralLtvZero = 6057,
    #[error(
        "Seeds must be default pubkeys for tag 0, and mint addresses for tag 1 or 2"
    )]
    InvalidObligationSeedsValue = 6058,
    #[error("[DEPRECATED] Obligation id must be 0")]
    DeprecatedInvalidObligationId = 6059,
    #[error("Invalid borrow rate curve point")]
    InvalidBorrowRateCurvePoint = 6060,
    #[error("Invalid utilization rate")]
    InvalidUtilizationRate = 6061,
    #[error("Obligation hasn't been fully liquidated and debt cannot be socialized.")]
    CannotSocializeObligationWithCollateral = 6062,
    #[error("Obligation has no borrows or deposits.")]
    ObligationEmpty = 6063,
    #[error("Withdrawal cap is reached")]
    WithdrawalCapReached = 6064,
    #[error("The last interval start timestamp is greater than the current timestamp")]
    LastTimestampGreaterThanCurrent = 6065,
    #[error("The reward amount is less than the minimum acceptable received liquidity")]
    LiquidationRewardTooSmall = 6066,
    #[error("Isolated Asset Tier Violation")]
    IsolatedAssetTierViolation = 6067,
    #[error("The obligation's elevation group and the reserve's are not the same")]
    InconsistentElevationGroup = 6068,
    #[error(
        "The elevation group chosen for the reserve does not exist in the lending market"
    )]
    InvalidElevationGroup = 6069,
    #[error("The elevation group updated has wrong parameters set")]
    InvalidElevationGroupConfig = 6070,
    #[error(
        "The current obligation must have most or all its debt repaid before changing the elevation group"
    )]
    UnhealthyElevationGroupLtv = 6071,
    #[error(
        "Elevation group does not accept any new loans or any new borrows/withdrawals"
    )]
    ElevationGroupNewLoansDisabled = 6072,
    #[error("Reserve was deprecated, no longer usable")]
    ReserveDeprecated = 6073,
    #[error("Referrer account not initialized")]
    ReferrerAccountNotInitialized = 6074,
    #[error("Referrer account mint does not match the operation reserve mint")]
    ReferrerAccountMintMissmatch = 6075,
    #[error("Referrer account address is not a valid program address")]
    ReferrerAccountWrongAddress = 6076,
    #[error("Referrer account referrer does not match the owner referrer")]
    ReferrerAccountReferrerMissmatch = 6077,
    #[error("Referrer account missing for obligation with referrer")]
    ReferrerAccountMissing = 6078,
    #[error("Insufficient referral fees to claim or no liquidity available")]
    InsufficientReferralFeesToRedeem = 6079,
    #[error("CPI disabled for this instruction")]
    CpiDisabled = 6080,
    #[error("Referrer short_url is not ascii alphanumeric")]
    ShortUrlNotAsciiAlphanumeric = 6081,
    #[error("Reserve is marked as obsolete")]
    ReserveObsolete = 6082,
    #[error("Obligation already part of the same elevation group")]
    ElevationGroupAlreadyActivated = 6083,
    #[error("Obligation has a deposit in a deprecated reserve")]
    ObligationInDeprecatedReserve = 6084,
    #[error("Referrer state owner does not match the given signer")]
    ReferrerStateOwnerMismatch = 6085,
    #[error("User metadata owner is already set")]
    UserMetadataOwnerAlreadySet = 6086,
    #[error("This collateral cannot be liquidated (LTV set to 0)")]
    CollateralNonLiquidatable = 6087,
    #[error("Borrowing is disabled")]
    BorrowingDisabled = 6088,
    #[error("Cannot borrow above borrow limit")]
    BorrowLimitExceeded = 6089,
    #[error("Cannot deposit above deposit limit")]
    DepositLimitExceeded = 6090,
    #[error("Reserve does not accept any new borrows outside elevation group")]
    BorrowingDisabledOutsideElevationGroup = 6091,
    #[error("Net value remaining too small")]
    NetValueRemainingTooSmall = 6092,
    #[error("Cannot get the obligation in a worse position")]
    WorseLtvBlocked = 6093,
    #[error("Cannot have more liabilities than assets in a position")]
    LiabilitiesBiggerThanAssets = 6094,
    #[error("Reserve state and token account cannot drift")]
    ReserveTokenBalanceMismatch = 6095,
    #[error("Reserve token account has been unexpectedly modified")]
    ReserveVaultBalanceMismatch = 6096,
    #[error("Reserve internal state accounting has been unexpectedly modified")]
    ReserveAccountingMismatch = 6097,
    #[error("Borrowing above set utilization rate is disabled")]
    BorrowingAboveUtilizationRateDisabled = 6098,
    #[error("Liquidation must prioritize the debt with the highest borrow factor")]
    LiquidationBorrowFactorPriority = 6099,
    #[error("Liquidation must prioritize the collateral with the lowest LTV")]
    LiquidationLowestLtvPriority = 6100,
    #[error("Elevation group borrow limit exceeded")]
    ElevationGroupBorrowLimitExceeded = 6101,
    #[error("The elevation group does not have a debt reserve defined")]
    ElevationGroupWithoutDebtReserve = 6102,
    #[error("The elevation group does not allow any collateral reserves")]
    ElevationGroupMaxCollateralReserveZero = 6103,
    #[error(
        "In elevation group attempt to borrow from a reserve that is not the debt reserve"
    )]
    ElevationGroupHasAnotherDebtReserve = 6104,
    #[error("The elevation group's debt reserve cannot be used as a collateral reserve")]
    ElevationGroupDebtReserveAsCollateral = 6105,
    #[error(
        "Obligation have more collateral than the maximum allowed by the elevation group"
    )]
    ObligationCollateralExceedsElevationGroupLimit = 6106,
    #[error("Obligation is an elevation group but have more than one debt reserve")]
    ObligationElevationGroupMultipleDebtReserve = 6107,
    #[error("Mint has a token (2022) extension that is not supported")]
    UnsupportedTokenExtension = 6108,
    #[error("Can't have an spl token mint with a t22 account")]
    InvalidTokenAccount = 6109,
    #[error("Can't deposit into this reserve outside elevation group")]
    DepositDisabledOutsideElevationGroup = 6110,
    #[error("Cannot calculate referral amount due to slots mismatch")]
    CannotCalculateReferralAmountDueToSlotsMismatch = 6111,
    #[error("Obligation owners must match")]
    ObligationOwnersMustMatch = 6112,
    #[error("Obligations must match")]
    ObligationsMustMatch = 6113,
    #[error("Lending markets must match")]
    LendingMarketsMustMatch = 6114,
    #[error("Obligation is already marked for deleveraging")]
    ObligationCurrentlyMarkedForDeleveraging = 6115,
    #[error("Maximum withdrawable value of this collateral is zero, LTV needs improved")]
    MaximumWithdrawValueZero = 6116,
    #[error("No max LTV 0 assets allowed in deposits for repay and withdraw")]
    ZeroMaxLtvAssetsInDeposits = 6117,
    #[error("The operation must prioritize the collateral with the lowest LTV")]
    MinLtvAssetsPriority = 6118,
    #[error("Cannot get the obligation liquidatable")]
    WorseLtvThanUnhealthyLtv = 6119,
    #[error("Farm accounts to refresh are missing")]
    FarmAccountsMissing = 6120,
}
impl From<KaminoLendingError> for ProgramError {
    fn from(e: KaminoLendingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for KaminoLendingError {
    fn type_of() -> &'static str {
        "KaminoLendingError"
    }
}
impl PrintProgramError for KaminoLendingError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
