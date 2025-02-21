use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UpdateConfigMode {
    UpdateLoanToValuePct,
    UpdateMaxLiquidationBonusBps,
    UpdateLiquidationThresholdPct,
    UpdateProtocolLiquidationFee,
    UpdateProtocolTakeRate,
    UpdateFeesBorrowFee,
    UpdateFeesFlashLoanFee,
    UpdateFeesReferralFeeBps,
    UpdateDepositLimit,
    UpdateBorrowLimit,
    UpdateTokenInfoLowerHeuristic,
    UpdateTokenInfoUpperHeuristic,
    UpdateTokenInfoExpHeuristic,
    UpdateTokenInfoTwapDivergence,
    UpdateTokenInfoScopeTwap,
    UpdateTokenInfoScopeChain,
    UpdateTokenInfoName,
    UpdateTokenInfoPriceMaxAge,
    UpdateTokenInfoTwapMaxAge,
    UpdateScopePriceFeed,
    UpdatePythPrice,
    UpdateSwitchboardFeed,
    UpdateSwitchboardTwapFeed,
    UpdateBorrowRateCurve,
    UpdateEntireReserveConfig,
    UpdateDebtWithdrawalCap,
    UpdateDepositWithdrawalCap,
    UpdateDebtWithdrawalCapCurrentTotal,
    UpdateDepositWithdrawalCapCurrentTotal,
    UpdateBadDebtLiquidationBonusBps,
    UpdateMinLiquidationBonusBps,
    UpdateDeleveragingMarginCallPeriod,
    UpdateBorrowFactor,
    UpdateAssetTier,
    UpdateElevationGroup,
    UpdateDeleveragingThresholdDecreaseBpsPerDay,
    DeprecatedUpdateMultiplierSideBoost,
    DeprecatedUpdateMultiplierTagBoost,
    UpdateReserveStatus,
    UpdateFarmCollateral,
    UpdateFarmDebt,
    UpdateDisableUsageAsCollateralOutsideEmode,
    UpdateBlockBorrowingAboveUtilizationPct,
    UpdateBlockPriceUsage,
    UpdateBorrowLimitOutsideElevationGroup,
    UpdateBorrowLimitsInElevationGroupAgainstThisReserve,
    UpdateHostFixedInterestRateBps,
    UpdateAutodeleverageEnabled,
    UpdateDeleveragingBonusIncreaseBpsPerDay,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UpdateLendingMarketConfigValue {
    Bool(bool),
    U8(u8),
    U8Array([u8; 8]),
    U16(u16),
    U64(u64),
    U128(u128),
    Pubkey(Pubkey),
    ElevationGroup(ElevationGroup),
    Name([u8; 32]),
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UpdateLendingMarketMode {
    UpdateOwner,
    UpdateEmergencyMode,
    UpdateLiquidationCloseFactor,
    UpdateLiquidationMaxValue,
    DeprecatedUpdateGlobalUnhealthyBorrow,
    UpdateGlobalAllowedBorrow,
    UpdateRiskCouncil,
    UpdateMinFullLiquidationThreshold,
    UpdateInsolvencyRiskLtv,
    UpdateElevationGroup,
    UpdateReferralFeeBps,
    DeprecatedUpdateMultiplierPoints,
    UpdatePriceRefreshTriggerToMaxAgePct,
    UpdateAutodeleverageEnabled,
    UpdateBorrowingDisabled,
    UpdateMinNetValueObligationPostAction,
    UpdateMinValueLtvSkipPriorityLiqCheck,
    UpdateMinValueBfSkipPriorityLiqCheck,
    UpdatePaddingFields,
    UpdateName,
    UpdateIndividualAutodeleverageMarginCallPeriodSecs,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LastUpdate {
    pub slot: u64,
    pub stale: u8,
    pub price_status: u8,
    pub placeholder: [u8; 6],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ElevationGroup {
    pub max_liquidation_bonus_bps: u16,
    pub id: u8,
    pub ltv_pct: u8,
    pub liquidation_threshold_pct: u8,
    pub allow_new_loans: u8,
    pub max_reserves_as_collateral: u8,
    pub padding0: u8,
    pub debt_reserve: Pubkey,
    pub padding1: [u64; 4],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitObligationArgs {
    pub tag: u8,
    pub id: u8,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObligationCollateral {
    pub deposit_reserve: Pubkey,
    pub deposited_amount: u64,
    pub market_value_sf: u128,
    pub borrowed_amount_against_this_collateral_in_elevation_group: u64,
    pub padding: [u64; 9],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObligationLiquidity {
    pub borrow_reserve: Pubkey,
    pub cumulative_borrow_rate_bsf: BigFractionBytes,
    pub padding: u64,
    pub borrowed_amount_sf: u128,
    pub market_value_sf: u128,
    pub borrow_factor_adjusted_market_value_sf: u128,
    pub borrowed_amount_outside_elevation_groups: u64,
    pub padding2: [u64; 7],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetTier {
    Regular,
    IsolatedCollateral,
    IsolatedDebt,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BigFractionBytes {
    pub value: [u64; 4],
    pub padding: [u64; 2],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeeCalculation {
    Exclusive,
    Inclusive,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReserveCollateral {
    pub mint_pubkey: Pubkey,
    pub mint_total_supply: u64,
    pub supply_vault: Pubkey,
    pub padding1: [u128; 32],
    pub padding2: [u128; 32],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReserveConfig {
    pub status: u8,
    pub asset_tier: u8,
    pub host_fixed_interest_rate_bps: u16,
    pub reserved2: [u8; 2],
    pub reserved3: [u8; 8],
    pub protocol_take_rate_pct: u8,
    pub protocol_liquidation_fee_pct: u8,
    pub loan_to_value_pct: u8,
    pub liquidation_threshold_pct: u8,
    pub min_liquidation_bonus_bps: u16,
    pub max_liquidation_bonus_bps: u16,
    pub bad_debt_liquidation_bonus_bps: u16,
    pub deleveraging_margin_call_period_secs: u64,
    pub deleveraging_threshold_decrease_bps_per_day: u64,
    pub fees: ReserveFees,
    pub borrow_rate_curve: BorrowRateCurve,
    pub borrow_factor_pct: u64,
    pub deposit_limit: u64,
    pub borrow_limit: u64,
    pub token_info: TokenInfo,
    pub deposit_withdrawal_cap: WithdrawalCaps,
    pub debt_withdrawal_cap: WithdrawalCaps,
    pub elevation_groups: [u8; 20],
    pub disable_usage_as_coll_outside_emode: u8,
    pub utilization_limit_block_borrowing_above_pct: u8,
    pub autodeleverage_enabled: u8,
    pub reserved1: [u8; 1],
    pub borrow_limit_outside_elevation_group: u64,
    pub borrow_limit_against_this_collateral_in_elevation_group: [u64; 32],
    pub deleveraging_bonus_increase_bps_per_day: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReserveFarmKind {
    Collateral,
    Debt,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReserveFees {
    pub borrow_fee_sf: u64,
    pub flash_loan_fee_sf: u64,
    pub padding: [u8; 8],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReserveLiquidity {
    pub mint_pubkey: Pubkey,
    pub supply_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub available_amount: u64,
    pub borrowed_amount_sf: u128,
    pub market_price_sf: u128,
    pub market_price_last_updated_ts: u64,
    pub mint_decimals: u64,
    pub deposit_limit_crossed_timestamp: u64,
    pub borrow_limit_crossed_timestamp: u64,
    pub cumulative_borrow_rate_bsf: BigFractionBytes,
    pub accumulated_protocol_fees_sf: u128,
    pub accumulated_referrer_fees_sf: u128,
    pub pending_referrer_fees_sf: u128,
    pub absolute_referral_rate_sf: u128,
    pub token_program: Pubkey,
    pub padding2: [u64; 51],
    pub padding3: [u128; 32],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReserveStatus {
    Active,
    Obsolete,
    Hidden,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawalCaps {
    pub config_capacity: i64,
    pub current_total: i64,
    pub last_interval_start_timestamp: u64,
    pub config_interval_length_seconds: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriceHeuristic {
    pub lower: u64,
    pub upper: u64,
    pub exp: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PythConfiguration {
    pub price: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScopeConfiguration {
    pub price_feed: Pubkey,
    pub price_chain: [u16; 4],
    pub twap_chain: [u16; 4],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwitchboardConfiguration {
    pub price_aggregator: Pubkey,
    pub twap_aggregator: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TokenInfo {
    pub name: [u8; 32],
    pub heuristic: PriceHeuristic,
    pub max_twap_divergence_bps: u64,
    pub max_age_price_seconds: u64,
    pub max_age_twap_seconds: u64,
    pub scope_configuration: ScopeConfiguration,
    pub switchboard_configuration: SwitchboardConfiguration,
    pub pyth_configuration: PythConfiguration,
    pub block_price_usage: u8,
    pub reserved: [u8; 7],
    pub padding: [u64; 19],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BorrowRateCurve {
    pub points: [CurvePoint; 11],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CurvePoint {
    pub utilization_rate_bps: u32,
    pub borrow_rate_bps: u32,
}
