use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey, program_error::ProgramError,
};
use std::io::Read;
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum KaminoLendingProgramIx {
    InitLendingMarket(InitLendingMarketIxArgs),
    UpdateLendingMarket(UpdateLendingMarketIxArgs),
    UpdateLendingMarketOwner,
    InitReserve,
    InitFarmsForReserve(InitFarmsForReserveIxArgs),
    UpdateReserveConfig(UpdateReserveConfigIxArgs),
    RedeemFees,
    WithdrawProtocolFee(WithdrawProtocolFeeIxArgs),
    SocializeLoss(SocializeLossIxArgs),
    SocializeLossV2(SocializeLossV2IxArgs),
    MarkObligationForDeleveraging(MarkObligationForDeleveragingIxArgs),
    RefreshReserve,
    RefreshReservesBatch(RefreshReservesBatchIxArgs),
    DepositReserveLiquidity(DepositReserveLiquidityIxArgs),
    RedeemReserveCollateral(RedeemReserveCollateralIxArgs),
    InitObligation(InitObligationIxArgs),
    InitObligationFarmsForReserve(InitObligationFarmsForReserveIxArgs),
    RefreshObligationFarmsForReserve(RefreshObligationFarmsForReserveIxArgs),
    RefreshObligation,
    DepositObligationCollateral(DepositObligationCollateralIxArgs),
    DepositObligationCollateralV2(DepositObligationCollateralV2IxArgs),
    WithdrawObligationCollateral(WithdrawObligationCollateralIxArgs),
    WithdrawObligationCollateralV2(WithdrawObligationCollateralV2IxArgs),
    BorrowObligationLiquidity(BorrowObligationLiquidityIxArgs),
    BorrowObligationLiquidityV2(BorrowObligationLiquidityV2IxArgs),
    RepayObligationLiquidity(RepayObligationLiquidityIxArgs),
    RepayObligationLiquidityV2(RepayObligationLiquidityV2IxArgs),
    RepayAndWithdrawAndRedeem(RepayAndWithdrawAndRedeemIxArgs),
    DepositAndWithdraw(DepositAndWithdrawIxArgs),
    DepositReserveLiquidityAndObligationCollateral(
        DepositReserveLiquidityAndObligationCollateralIxArgs,
    ),
    DepositReserveLiquidityAndObligationCollateralV2(
        DepositReserveLiquidityAndObligationCollateralV2IxArgs,
    ),
    WithdrawObligationCollateralAndRedeemReserveCollateral(
        WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs,
    ),
    WithdrawObligationCollateralAndRedeemReserveCollateralV2(
        WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs,
    ),
    LiquidateObligationAndRedeemReserveCollateral(
        LiquidateObligationAndRedeemReserveCollateralIxArgs,
    ),
    LiquidateObligationAndRedeemReserveCollateralV2(
        LiquidateObligationAndRedeemReserveCollateralV2IxArgs,
    ),
    FlashRepayReserveLiquidity(FlashRepayReserveLiquidityIxArgs),
    FlashBorrowReserveLiquidity(FlashBorrowReserveLiquidityIxArgs),
    RequestElevationGroup(RequestElevationGroupIxArgs),
    InitReferrerTokenState,
    InitUserMetadata(InitUserMetadataIxArgs),
    WithdrawReferrerFees,
    InitReferrerStateAndShortUrl(InitReferrerStateAndShortUrlIxArgs),
    DeleteReferrerStateAndShortUrl,
    IdlMissingTypes(IdlMissingTypesIxArgs),
}
impl KaminoLendingProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            INIT_LENDING_MARKET_IX_DISCM => {
                Ok(
                    Self::InitLendingMarket(
                        InitLendingMarketIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            UPDATE_LENDING_MARKET_IX_DISCM => {
                Ok(
                    Self::UpdateLendingMarket(
                        UpdateLendingMarketIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            UPDATE_LENDING_MARKET_OWNER_IX_DISCM => Ok(Self::UpdateLendingMarketOwner),
            INIT_RESERVE_IX_DISCM => Ok(Self::InitReserve),
            INIT_FARMS_FOR_RESERVE_IX_DISCM => {
                Ok(
                    Self::InitFarmsForReserve(
                        InitFarmsForReserveIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            UPDATE_RESERVE_CONFIG_IX_DISCM => {
                Ok(
                    Self::UpdateReserveConfig(
                        UpdateReserveConfigIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REDEEM_FEES_IX_DISCM => Ok(Self::RedeemFees),
            WITHDRAW_PROTOCOL_FEE_IX_DISCM => {
                Ok(
                    Self::WithdrawProtocolFee(
                        WithdrawProtocolFeeIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            SOCIALIZE_LOSS_IX_DISCM => {
                Ok(Self::SocializeLoss(SocializeLossIxArgs::deserialize(&mut reader)?))
            }
            SOCIALIZE_LOSS_V2_IX_DISCM => {
                Ok(
                    Self::SocializeLossV2(
                        SocializeLossV2IxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            MARK_OBLIGATION_FOR_DELEVERAGING_IX_DISCM => {
                Ok(
                    Self::MarkObligationForDeleveraging(
                        MarkObligationForDeleveragingIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REFRESH_RESERVE_IX_DISCM => Ok(Self::RefreshReserve),
            REFRESH_RESERVES_BATCH_IX_DISCM => {
                Ok(
                    Self::RefreshReservesBatch(
                        RefreshReservesBatchIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            DEPOSIT_RESERVE_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::DepositReserveLiquidity(
                        DepositReserveLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REDEEM_RESERVE_COLLATERAL_IX_DISCM => {
                Ok(
                    Self::RedeemReserveCollateral(
                        RedeemReserveCollateralIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INIT_OBLIGATION_IX_DISCM => {
                Ok(Self::InitObligation(InitObligationIxArgs::deserialize(&mut reader)?))
            }
            INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM => {
                Ok(
                    Self::InitObligationFarmsForReserve(
                        InitObligationFarmsForReserveIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM => {
                Ok(
                    Self::RefreshObligationFarmsForReserve(
                        RefreshObligationFarmsForReserveIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REFRESH_OBLIGATION_IX_DISCM => Ok(Self::RefreshObligation),
            DEPOSIT_OBLIGATION_COLLATERAL_IX_DISCM => {
                Ok(
                    Self::DepositObligationCollateral(
                        DepositObligationCollateralIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_DISCM => {
                Ok(
                    Self::DepositObligationCollateralV2(
                        DepositObligationCollateralV2IxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            WITHDRAW_OBLIGATION_COLLATERAL_IX_DISCM => {
                Ok(
                    Self::WithdrawObligationCollateral(
                        WithdrawObligationCollateralIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_DISCM => {
                Ok(
                    Self::WithdrawObligationCollateralV2(
                        WithdrawObligationCollateralV2IxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            BORROW_OBLIGATION_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::BorrowObligationLiquidity(
                        BorrowObligationLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            BORROW_OBLIGATION_LIQUIDITY_V2_IX_DISCM => {
                Ok(
                    Self::BorrowObligationLiquidityV2(
                        BorrowObligationLiquidityV2IxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REPAY_OBLIGATION_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::RepayObligationLiquidity(
                        RepayObligationLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REPAY_OBLIGATION_LIQUIDITY_V2_IX_DISCM => {
                Ok(
                    Self::RepayObligationLiquidityV2(
                        RepayObligationLiquidityV2IxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REPAY_AND_WITHDRAW_AND_REDEEM_IX_DISCM => {
                Ok(
                    Self::RepayAndWithdrawAndRedeem(
                        RepayAndWithdrawAndRedeemIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            DEPOSIT_AND_WITHDRAW_IX_DISCM => {
                Ok(
                    Self::DepositAndWithdraw(
                        DepositAndWithdrawIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_DISCM => {
                Ok(
                    Self::DepositReserveLiquidityAndObligationCollateral(
                        DepositReserveLiquidityAndObligationCollateralIxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_DISCM => {
                Ok(
                    Self::DepositReserveLiquidityAndObligationCollateralV2(
                        DepositReserveLiquidityAndObligationCollateralV2IxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM => {
                Ok(
                    Self::WithdrawObligationCollateralAndRedeemReserveCollateral(
                        WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM => {
                Ok(
                    Self::WithdrawObligationCollateralAndRedeemReserveCollateralV2(
                        WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM => {
                Ok(
                    Self::LiquidateObligationAndRedeemReserveCollateral(
                        LiquidateObligationAndRedeemReserveCollateralIxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM => {
                Ok(
                    Self::LiquidateObligationAndRedeemReserveCollateralV2(
                        LiquidateObligationAndRedeemReserveCollateralV2IxArgs::deserialize(
                            &mut reader,
                        )?,
                    ),
                )
            }
            FLASH_REPAY_RESERVE_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::FlashRepayReserveLiquidity(
                        FlashRepayReserveLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            FLASH_BORROW_RESERVE_LIQUIDITY_IX_DISCM => {
                Ok(
                    Self::FlashBorrowReserveLiquidity(
                        FlashBorrowReserveLiquidityIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            REQUEST_ELEVATION_GROUP_IX_DISCM => {
                Ok(
                    Self::RequestElevationGroup(
                        RequestElevationGroupIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INIT_REFERRER_TOKEN_STATE_IX_DISCM => Ok(Self::InitReferrerTokenState),
            INIT_USER_METADATA_IX_DISCM => {
                Ok(
                    Self::InitUserMetadata(
                        InitUserMetadataIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            WITHDRAW_REFERRER_FEES_IX_DISCM => Ok(Self::WithdrawReferrerFees),
            INIT_REFERRER_STATE_AND_SHORT_URL_IX_DISCM => {
                Ok(
                    Self::InitReferrerStateAndShortUrl(
                        InitReferrerStateAndShortUrlIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            DELETE_REFERRER_STATE_AND_SHORT_URL_IX_DISCM => {
                Ok(Self::DeleteReferrerStateAndShortUrl)
            }
            IDL_MISSING_TYPES_IX_DISCM => {
                Ok(
                    Self::IdlMissingTypes(
                        IdlMissingTypesIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            _ => {
                Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("discm {:?} not found", maybe_discm),
                    ),
                )
            }
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitLendingMarket(args) => {
                writer.write_all(&INIT_LENDING_MARKET_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateLendingMarket(args) => {
                writer.write_all(&UPDATE_LENDING_MARKET_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateLendingMarketOwner => {
                writer.write_all(&UPDATE_LENDING_MARKET_OWNER_IX_DISCM)
            }
            Self::InitReserve => writer.write_all(&INIT_RESERVE_IX_DISCM),
            Self::InitFarmsForReserve(args) => {
                writer.write_all(&INIT_FARMS_FOR_RESERVE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateReserveConfig(args) => {
                writer.write_all(&UPDATE_RESERVE_CONFIG_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RedeemFees => writer.write_all(&REDEEM_FEES_IX_DISCM),
            Self::WithdrawProtocolFee(args) => {
                writer.write_all(&WITHDRAW_PROTOCOL_FEE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SocializeLoss(args) => {
                writer.write_all(&SOCIALIZE_LOSS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SocializeLossV2(args) => {
                writer.write_all(&SOCIALIZE_LOSS_V2_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::MarkObligationForDeleveraging(args) => {
                writer.write_all(&MARK_OBLIGATION_FOR_DELEVERAGING_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RefreshReserve => writer.write_all(&REFRESH_RESERVE_IX_DISCM),
            Self::RefreshReservesBatch(args) => {
                writer.write_all(&REFRESH_RESERVES_BATCH_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::DepositReserveLiquidity(args) => {
                writer.write_all(&DEPOSIT_RESERVE_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RedeemReserveCollateral(args) => {
                writer.write_all(&REDEEM_RESERVE_COLLATERAL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitObligation(args) => {
                writer.write_all(&INIT_OBLIGATION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitObligationFarmsForReserve(args) => {
                writer.write_all(&INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RefreshObligationFarmsForReserve(args) => {
                writer.write_all(&REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RefreshObligation => writer.write_all(&REFRESH_OBLIGATION_IX_DISCM),
            Self::DepositObligationCollateral(args) => {
                writer.write_all(&DEPOSIT_OBLIGATION_COLLATERAL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::DepositObligationCollateralV2(args) => {
                writer.write_all(&DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::WithdrawObligationCollateral(args) => {
                writer.write_all(&WITHDRAW_OBLIGATION_COLLATERAL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::WithdrawObligationCollateralV2(args) => {
                writer.write_all(&WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::BorrowObligationLiquidity(args) => {
                writer.write_all(&BORROW_OBLIGATION_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::BorrowObligationLiquidityV2(args) => {
                writer.write_all(&BORROW_OBLIGATION_LIQUIDITY_V2_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RepayObligationLiquidity(args) => {
                writer.write_all(&REPAY_OBLIGATION_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RepayObligationLiquidityV2(args) => {
                writer.write_all(&REPAY_OBLIGATION_LIQUIDITY_V2_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RepayAndWithdrawAndRedeem(args) => {
                writer.write_all(&REPAY_AND_WITHDRAW_AND_REDEEM_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::DepositAndWithdraw(args) => {
                writer.write_all(&DEPOSIT_AND_WITHDRAW_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::DepositReserveLiquidityAndObligationCollateral(args) => {
                writer
                    .write_all(
                        &DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_DISCM,
                    )?;
                args.serialize(&mut writer)
            }
            Self::DepositReserveLiquidityAndObligationCollateralV2(args) => {
                writer
                    .write_all(
                        &DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_DISCM,
                    )?;
                args.serialize(&mut writer)
            }
            Self::WithdrawObligationCollateralAndRedeemReserveCollateral(args) => {
                writer
                    .write_all(
                        &WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM,
                    )?;
                args.serialize(&mut writer)
            }
            Self::WithdrawObligationCollateralAndRedeemReserveCollateralV2(args) => {
                writer
                    .write_all(
                        &WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM,
                    )?;
                args.serialize(&mut writer)
            }
            Self::LiquidateObligationAndRedeemReserveCollateral(args) => {
                writer
                    .write_all(
                        &LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM,
                    )?;
                args.serialize(&mut writer)
            }
            Self::LiquidateObligationAndRedeemReserveCollateralV2(args) => {
                writer
                    .write_all(
                        &LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM,
                    )?;
                args.serialize(&mut writer)
            }
            Self::FlashRepayReserveLiquidity(args) => {
                writer.write_all(&FLASH_REPAY_RESERVE_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::FlashBorrowReserveLiquidity(args) => {
                writer.write_all(&FLASH_BORROW_RESERVE_LIQUIDITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RequestElevationGroup(args) => {
                writer.write_all(&REQUEST_ELEVATION_GROUP_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitReferrerTokenState => {
                writer.write_all(&INIT_REFERRER_TOKEN_STATE_IX_DISCM)
            }
            Self::InitUserMetadata(args) => {
                writer.write_all(&INIT_USER_METADATA_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::WithdrawReferrerFees => {
                writer.write_all(&WITHDRAW_REFERRER_FEES_IX_DISCM)
            }
            Self::InitReferrerStateAndShortUrl(args) => {
                writer.write_all(&INIT_REFERRER_STATE_AND_SHORT_URL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::DeleteReferrerStateAndShortUrl => {
                writer.write_all(&DELETE_REFERRER_STATE_AND_SHORT_URL_IX_DISCM)
            }
            Self::IdlMissingTypes(args) => {
                writer.write_all(&IDL_MISSING_TYPES_IX_DISCM)?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const INIT_LENDING_MARKET_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitLendingMarketAccounts<'me, 'info> {
    pub lending_market_owner: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitLendingMarketKeys {
    pub lending_market_owner: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<InitLendingMarketAccounts<'_, '_>> for InitLendingMarketKeys {
    fn from(accounts: InitLendingMarketAccounts) -> Self {
        Self {
            lending_market_owner: *accounts.lending_market_owner.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitLendingMarketKeys> for [AccountMeta; INIT_LENDING_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: InitLendingMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lending_market_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_LENDING_MARKET_IX_ACCOUNTS_LEN]> for InitLendingMarketKeys {
    fn from(pubkeys: [Pubkey; INIT_LENDING_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market_owner: pubkeys[0],
            lending_market: pubkeys[1],
            lending_market_authority: pubkeys[2],
            system_program: pubkeys[3],
            rent: pubkeys[4],
        }
    }
}
impl<'info> From<InitLendingMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_LENDING_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitLendingMarketAccounts<'_, 'info>) -> Self {
        [
            accounts.lending_market_owner.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_LENDING_MARKET_IX_ACCOUNTS_LEN]>
for InitLendingMarketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_LENDING_MARKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lending_market_owner: &arr[0],
            lending_market: &arr[1],
            lending_market_authority: &arr[2],
            system_program: &arr[3],
            rent: &arr[4],
        }
    }
}
pub const INIT_LENDING_MARKET_IX_DISCM: [u8; 8] = [34, 162, 116, 14, 101, 137, 94, 239];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitLendingMarketIxArgs {
    pub quote_currency: [u8; 32],
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitLendingMarketIxData(pub InitLendingMarketIxArgs);
impl From<InitLendingMarketIxArgs> for InitLendingMarketIxData {
    fn from(args: InitLendingMarketIxArgs) -> Self {
        Self(args)
    }
}
impl InitLendingMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_LENDING_MARKET_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INIT_LENDING_MARKET_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitLendingMarketIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_LENDING_MARKET_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_lending_market_ix_with_program_id(
    program_id: Pubkey,
    keys: InitLendingMarketKeys,
    args: InitLendingMarketIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_LENDING_MARKET_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitLendingMarketIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_lending_market_ix(
    keys: InitLendingMarketKeys,
    args: InitLendingMarketIxArgs,
) -> std::io::Result<Instruction> {
    init_lending_market_ix_with_program_id(crate::ID, keys, args)
}
pub fn init_lending_market_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitLendingMarketAccounts<'_, '_>,
    args: InitLendingMarketIxArgs,
) -> ProgramResult {
    let keys: InitLendingMarketKeys = accounts.into();
    let ix = init_lending_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_lending_market_invoke(
    accounts: InitLendingMarketAccounts<'_, '_>,
    args: InitLendingMarketIxArgs,
) -> ProgramResult {
    init_lending_market_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn init_lending_market_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitLendingMarketAccounts<'_, '_>,
    args: InitLendingMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitLendingMarketKeys = accounts.into();
    let ix = init_lending_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_lending_market_invoke_signed(
    accounts: InitLendingMarketAccounts<'_, '_>,
    args: InitLendingMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_lending_market_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn init_lending_market_verify_account_keys(
    accounts: InitLendingMarketAccounts<'_, '_>,
    keys: InitLendingMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lending_market_owner.key, keys.lending_market_owner),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_lending_market_verify_writable_privileges<'me, 'info>(
    accounts: InitLendingMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lending_market_owner, accounts.lending_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_lending_market_verify_signer_privileges<'me, 'info>(
    accounts: InitLendingMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lending_market_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_lending_market_verify_account_privileges<'me, 'info>(
    accounts: InitLendingMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_lending_market_verify_writable_privileges(accounts)?;
    init_lending_market_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_LENDING_MARKET_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLendingMarketAccounts<'me, 'info> {
    pub lending_market_owner: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLendingMarketKeys {
    pub lending_market_owner: Pubkey,
    pub lending_market: Pubkey,
}
impl From<UpdateLendingMarketAccounts<'_, '_>> for UpdateLendingMarketKeys {
    fn from(accounts: UpdateLendingMarketAccounts) -> Self {
        Self {
            lending_market_owner: *accounts.lending_market_owner.key,
            lending_market: *accounts.lending_market.key,
        }
    }
}
impl From<UpdateLendingMarketKeys>
for [AccountMeta; UPDATE_LENDING_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateLendingMarketKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lending_market_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_LENDING_MARKET_IX_ACCOUNTS_LEN]> for UpdateLendingMarketKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LENDING_MARKET_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market_owner: pubkeys[0],
            lending_market: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateLendingMarketAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_LENDING_MARKET_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateLendingMarketAccounts<'_, 'info>) -> Self {
        [accounts.lending_market_owner.clone(), accounts.lending_market.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_LENDING_MARKET_IX_ACCOUNTS_LEN]>
for UpdateLendingMarketAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_LENDING_MARKET_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lending_market_owner: &arr[0],
            lending_market: &arr[1],
        }
    }
}
pub const UPDATE_LENDING_MARKET_IX_DISCM: [u8; 8] = [209, 157, 53, 210, 97, 180, 31, 45];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateLendingMarketIxArgs {
    pub mode: u64,
    pub value: [u8; 72],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLendingMarketIxData(pub UpdateLendingMarketIxArgs);
impl From<UpdateLendingMarketIxArgs> for UpdateLendingMarketIxData {
    fn from(args: UpdateLendingMarketIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateLendingMarketIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LENDING_MARKET_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_LENDING_MARKET_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateLendingMarketIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LENDING_MARKET_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_lending_market_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateLendingMarketKeys,
    args: UpdateLendingMarketIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_LENDING_MARKET_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateLendingMarketIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_lending_market_ix(
    keys: UpdateLendingMarketKeys,
    args: UpdateLendingMarketIxArgs,
) -> std::io::Result<Instruction> {
    update_lending_market_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_lending_market_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLendingMarketAccounts<'_, '_>,
    args: UpdateLendingMarketIxArgs,
) -> ProgramResult {
    let keys: UpdateLendingMarketKeys = accounts.into();
    let ix = update_lending_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_lending_market_invoke(
    accounts: UpdateLendingMarketAccounts<'_, '_>,
    args: UpdateLendingMarketIxArgs,
) -> ProgramResult {
    update_lending_market_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_lending_market_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLendingMarketAccounts<'_, '_>,
    args: UpdateLendingMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateLendingMarketKeys = accounts.into();
    let ix = update_lending_market_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_lending_market_invoke_signed(
    accounts: UpdateLendingMarketAccounts<'_, '_>,
    args: UpdateLendingMarketIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_lending_market_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_lending_market_verify_account_keys(
    accounts: UpdateLendingMarketAccounts<'_, '_>,
    keys: UpdateLendingMarketKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lending_market_owner.key, keys.lending_market_owner),
        (*accounts.lending_market.key, keys.lending_market),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_lending_market_verify_writable_privileges<'me, 'info>(
    accounts: UpdateLendingMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lending_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_lending_market_verify_signer_privileges<'me, 'info>(
    accounts: UpdateLendingMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lending_market_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_lending_market_verify_account_privileges<'me, 'info>(
    accounts: UpdateLendingMarketAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_lending_market_verify_writable_privileges(accounts)?;
    update_lending_market_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_LENDING_MARKET_OWNER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLendingMarketOwnerAccounts<'me, 'info> {
    pub lending_market_owner_cached: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateLendingMarketOwnerKeys {
    pub lending_market_owner_cached: Pubkey,
    pub lending_market: Pubkey,
}
impl From<UpdateLendingMarketOwnerAccounts<'_, '_>> for UpdateLendingMarketOwnerKeys {
    fn from(accounts: UpdateLendingMarketOwnerAccounts) -> Self {
        Self {
            lending_market_owner_cached: *accounts.lending_market_owner_cached.key,
            lending_market: *accounts.lending_market.key,
        }
    }
}
impl From<UpdateLendingMarketOwnerKeys>
for [AccountMeta; UPDATE_LENDING_MARKET_OWNER_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateLendingMarketOwnerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lending_market_owner_cached,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_LENDING_MARKET_OWNER_IX_ACCOUNTS_LEN]>
for UpdateLendingMarketOwnerKeys {
    fn from(pubkeys: [Pubkey; UPDATE_LENDING_MARKET_OWNER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market_owner_cached: pubkeys[0],
            lending_market: pubkeys[1],
        }
    }
}
impl<'info> From<UpdateLendingMarketOwnerAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_LENDING_MARKET_OWNER_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateLendingMarketOwnerAccounts<'_, 'info>) -> Self {
        [accounts.lending_market_owner_cached.clone(), accounts.lending_market.clone()]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_LENDING_MARKET_OWNER_IX_ACCOUNTS_LEN]>
for UpdateLendingMarketOwnerAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_LENDING_MARKET_OWNER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lending_market_owner_cached: &arr[0],
            lending_market: &arr[1],
        }
    }
}
pub const UPDATE_LENDING_MARKET_OWNER_IX_DISCM: [u8; 8] = [
    118,
    224,
    10,
    62,
    196,
    230,
    184,
    89,
];
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateLendingMarketOwnerIxData;
impl UpdateLendingMarketOwnerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_LENDING_MARKET_OWNER_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_LENDING_MARKET_OWNER_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LENDING_MARKET_OWNER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_lending_market_owner_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateLendingMarketOwnerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_LENDING_MARKET_OWNER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateLendingMarketOwnerIxData.try_to_vec()?,
    })
}
pub fn update_lending_market_owner_ix(
    keys: UpdateLendingMarketOwnerKeys,
) -> std::io::Result<Instruction> {
    update_lending_market_owner_ix_with_program_id(crate::ID, keys)
}
pub fn update_lending_market_owner_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLendingMarketOwnerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateLendingMarketOwnerKeys = accounts.into();
    let ix = update_lending_market_owner_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_lending_market_owner_invoke(
    accounts: UpdateLendingMarketOwnerAccounts<'_, '_>,
) -> ProgramResult {
    update_lending_market_owner_invoke_with_program_id(crate::ID, accounts)
}
pub fn update_lending_market_owner_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateLendingMarketOwnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateLendingMarketOwnerKeys = accounts.into();
    let ix = update_lending_market_owner_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_lending_market_owner_invoke_signed(
    accounts: UpdateLendingMarketOwnerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_lending_market_owner_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn update_lending_market_owner_verify_account_keys(
    accounts: UpdateLendingMarketOwnerAccounts<'_, '_>,
    keys: UpdateLendingMarketOwnerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lending_market_owner_cached.key, keys.lending_market_owner_cached),
        (*accounts.lending_market.key, keys.lending_market),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_lending_market_owner_verify_writable_privileges<'me, 'info>(
    accounts: UpdateLendingMarketOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.lending_market] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_lending_market_owner_verify_signer_privileges<'me, 'info>(
    accounts: UpdateLendingMarketOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lending_market_owner_cached] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_lending_market_owner_verify_account_privileges<'me, 'info>(
    accounts: UpdateLendingMarketOwnerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_lending_market_owner_verify_writable_privileges(accounts)?;
    update_lending_market_owner_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_RESERVE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct InitReserveAccounts<'me, 'info> {
    pub lending_market_owner: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub fee_receiver: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_collateral_supply: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub liquidity_token_program: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitReserveKeys {
    pub lending_market_owner: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub fee_receiver: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_collateral_supply: Pubkey,
    pub rent: Pubkey,
    pub liquidity_token_program: Pubkey,
    pub collateral_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitReserveAccounts<'_, '_>> for InitReserveKeys {
    fn from(accounts: InitReserveAccounts) -> Self {
        Self {
            lending_market_owner: *accounts.lending_market_owner.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            reserve: *accounts.reserve.key,
            reserve_liquidity_mint: *accounts.reserve_liquidity_mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            fee_receiver: *accounts.fee_receiver.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_collateral_supply: *accounts.reserve_collateral_supply.key,
            rent: *accounts.rent.key,
            liquidity_token_program: *accounts.liquidity_token_program.key,
            collateral_token_program: *accounts.collateral_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitReserveKeys> for [AccountMeta; INIT_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitReserveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lending_market_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_RESERVE_IX_ACCOUNTS_LEN]> for InitReserveKeys {
    fn from(pubkeys: [Pubkey; INIT_RESERVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market_owner: pubkeys[0],
            lending_market: pubkeys[1],
            lending_market_authority: pubkeys[2],
            reserve: pubkeys[3],
            reserve_liquidity_mint: pubkeys[4],
            reserve_liquidity_supply: pubkeys[5],
            fee_receiver: pubkeys[6],
            reserve_collateral_mint: pubkeys[7],
            reserve_collateral_supply: pubkeys[8],
            rent: pubkeys[9],
            liquidity_token_program: pubkeys[10],
            collateral_token_program: pubkeys[11],
            system_program: pubkeys[12],
        }
    }
}
impl<'info> From<InitReserveAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitReserveAccounts<'_, 'info>) -> Self {
        [
            accounts.lending_market_owner.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.reserve.clone(),
            accounts.reserve_liquidity_mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.fee_receiver.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_collateral_supply.clone(),
            accounts.rent.clone(),
            accounts.liquidity_token_program.clone(),
            accounts.collateral_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_RESERVE_IX_ACCOUNTS_LEN]>
for InitReserveAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_RESERVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market_owner: &arr[0],
            lending_market: &arr[1],
            lending_market_authority: &arr[2],
            reserve: &arr[3],
            reserve_liquidity_mint: &arr[4],
            reserve_liquidity_supply: &arr[5],
            fee_receiver: &arr[6],
            reserve_collateral_mint: &arr[7],
            reserve_collateral_supply: &arr[8],
            rent: &arr[9],
            liquidity_token_program: &arr[10],
            collateral_token_program: &arr[11],
            system_program: &arr[12],
        }
    }
}
pub const INIT_RESERVE_IX_DISCM: [u8; 8] = [138, 245, 71, 225, 153, 4, 3, 43];
#[derive(Clone, Debug, PartialEq)]
pub struct InitReserveIxData;
impl InitReserveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_RESERVE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INIT_RESERVE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_RESERVE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_reserve_ix_with_program_id(
    program_id: Pubkey,
    keys: InitReserveKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_RESERVE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitReserveIxData.try_to_vec()?,
    })
}
pub fn init_reserve_ix(keys: InitReserveKeys) -> std::io::Result<Instruction> {
    init_reserve_ix_with_program_id(crate::ID, keys)
}
pub fn init_reserve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitReserveAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitReserveKeys = accounts.into();
    let ix = init_reserve_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_reserve_invoke(accounts: InitReserveAccounts<'_, '_>) -> ProgramResult {
    init_reserve_invoke_with_program_id(crate::ID, accounts)
}
pub fn init_reserve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitReserveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitReserveKeys = accounts.into();
    let ix = init_reserve_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_reserve_invoke_signed(
    accounts: InitReserveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_reserve_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn init_reserve_verify_account_keys(
    accounts: InitReserveAccounts<'_, '_>,
    keys: InitReserveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lending_market_owner.key, keys.lending_market_owner),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_liquidity_mint.key, keys.reserve_liquidity_mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.fee_receiver.key, keys.fee_receiver),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.reserve_collateral_supply.key, keys.reserve_collateral_supply),
        (*accounts.rent.key, keys.rent),
        (*accounts.liquidity_token_program.key, keys.liquidity_token_program),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_reserve_verify_writable_privileges<'me, 'info>(
    accounts: InitReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lending_market_owner,
        accounts.reserve,
        accounts.reserve_liquidity_supply,
        accounts.fee_receiver,
        accounts.reserve_collateral_mint,
        accounts.reserve_collateral_supply,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_reserve_verify_signer_privileges<'me, 'info>(
    accounts: InitReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lending_market_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_reserve_verify_account_privileges<'me, 'info>(
    accounts: InitReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_reserve_verify_writable_privileges(accounts)?;
    init_reserve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct InitFarmsForReserveAccounts<'me, 'info> {
    pub lending_market_owner: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
    pub farms_global_config: &'me AccountInfo<'info>,
    pub farm_state: &'me AccountInfo<'info>,
    pub farms_vault_authority: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitFarmsForReserveKeys {
    pub lending_market_owner: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub reserve: Pubkey,
    pub farms_program: Pubkey,
    pub farms_global_config: Pubkey,
    pub farm_state: Pubkey,
    pub farms_vault_authority: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitFarmsForReserveAccounts<'_, '_>> for InitFarmsForReserveKeys {
    fn from(accounts: InitFarmsForReserveAccounts) -> Self {
        Self {
            lending_market_owner: *accounts.lending_market_owner.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            reserve: *accounts.reserve.key,
            farms_program: *accounts.farms_program.key,
            farms_global_config: *accounts.farms_global_config.key,
            farm_state: *accounts.farm_state.key,
            farms_vault_authority: *accounts.farms_vault_authority.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitFarmsForReserveKeys>
for [AccountMeta; INIT_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitFarmsForReserveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lending_market_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_global_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN]> for InitFarmsForReserveKeys {
    fn from(pubkeys: [Pubkey; INIT_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market_owner: pubkeys[0],
            lending_market: pubkeys[1],
            lending_market_authority: pubkeys[2],
            reserve: pubkeys[3],
            farms_program: pubkeys[4],
            farms_global_config: pubkeys[5],
            farm_state: pubkeys[6],
            farms_vault_authority: pubkeys[7],
            rent: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<InitFarmsForReserveAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitFarmsForReserveAccounts<'_, 'info>) -> Self {
        [
            accounts.lending_market_owner.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.reserve.clone(),
            accounts.farms_program.clone(),
            accounts.farms_global_config.clone(),
            accounts.farm_state.clone(),
            accounts.farms_vault_authority.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN]>
for InitFarmsForReserveAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lending_market_owner: &arr[0],
            lending_market: &arr[1],
            lending_market_authority: &arr[2],
            reserve: &arr[3],
            farms_program: &arr[4],
            farms_global_config: &arr[5],
            farm_state: &arr[6],
            farms_vault_authority: &arr[7],
            rent: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const INIT_FARMS_FOR_RESERVE_IX_DISCM: [u8; 8] = [218, 6, 62, 233, 1, 33, 232, 82];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitFarmsForReserveIxArgs {
    pub mode: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitFarmsForReserveIxData(pub InitFarmsForReserveIxArgs);
impl From<InitFarmsForReserveIxArgs> for InitFarmsForReserveIxData {
    fn from(args: InitFarmsForReserveIxArgs) -> Self {
        Self(args)
    }
}
impl InitFarmsForReserveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_FARMS_FOR_RESERVE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INIT_FARMS_FOR_RESERVE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitFarmsForReserveIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_FARMS_FOR_RESERVE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_farms_for_reserve_ix_with_program_id(
    program_id: Pubkey,
    keys: InitFarmsForReserveKeys,
    args: InitFarmsForReserveIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitFarmsForReserveIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_farms_for_reserve_ix(
    keys: InitFarmsForReserveKeys,
    args: InitFarmsForReserveIxArgs,
) -> std::io::Result<Instruction> {
    init_farms_for_reserve_ix_with_program_id(crate::ID, keys, args)
}
pub fn init_farms_for_reserve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitFarmsForReserveAccounts<'_, '_>,
    args: InitFarmsForReserveIxArgs,
) -> ProgramResult {
    let keys: InitFarmsForReserveKeys = accounts.into();
    let ix = init_farms_for_reserve_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_farms_for_reserve_invoke(
    accounts: InitFarmsForReserveAccounts<'_, '_>,
    args: InitFarmsForReserveIxArgs,
) -> ProgramResult {
    init_farms_for_reserve_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn init_farms_for_reserve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitFarmsForReserveAccounts<'_, '_>,
    args: InitFarmsForReserveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitFarmsForReserveKeys = accounts.into();
    let ix = init_farms_for_reserve_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_farms_for_reserve_invoke_signed(
    accounts: InitFarmsForReserveAccounts<'_, '_>,
    args: InitFarmsForReserveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_farms_for_reserve_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_farms_for_reserve_verify_account_keys(
    accounts: InitFarmsForReserveAccounts<'_, '_>,
    keys: InitFarmsForReserveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lending_market_owner.key, keys.lending_market_owner),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.farms_program.key, keys.farms_program),
        (*accounts.farms_global_config.key, keys.farms_global_config),
        (*accounts.farm_state.key, keys.farm_state),
        (*accounts.farms_vault_authority.key, keys.farms_vault_authority),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_farms_for_reserve_verify_writable_privileges<'me, 'info>(
    accounts: InitFarmsForReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lending_market_owner,
        accounts.reserve,
        accounts.farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_farms_for_reserve_verify_signer_privileges<'me, 'info>(
    accounts: InitFarmsForReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lending_market_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_farms_for_reserve_verify_account_privileges<'me, 'info>(
    accounts: InitFarmsForReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_farms_for_reserve_verify_writable_privileges(accounts)?;
    init_farms_for_reserve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_RESERVE_CONFIG_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdateReserveConfigAccounts<'me, 'info> {
    pub lending_market_owner: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateReserveConfigKeys {
    pub lending_market_owner: Pubkey,
    pub lending_market: Pubkey,
    pub reserve: Pubkey,
}
impl From<UpdateReserveConfigAccounts<'_, '_>> for UpdateReserveConfigKeys {
    fn from(accounts: UpdateReserveConfigAccounts) -> Self {
        Self {
            lending_market_owner: *accounts.lending_market_owner.key,
            lending_market: *accounts.lending_market.key,
            reserve: *accounts.reserve.key,
        }
    }
}
impl From<UpdateReserveConfigKeys>
for [AccountMeta; UPDATE_RESERVE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateReserveConfigKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lending_market_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_RESERVE_CONFIG_IX_ACCOUNTS_LEN]> for UpdateReserveConfigKeys {
    fn from(pubkeys: [Pubkey; UPDATE_RESERVE_CONFIG_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market_owner: pubkeys[0],
            lending_market: pubkeys[1],
            reserve: pubkeys[2],
        }
    }
}
impl<'info> From<UpdateReserveConfigAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_RESERVE_CONFIG_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateReserveConfigAccounts<'_, 'info>) -> Self {
        [
            accounts.lending_market_owner.clone(),
            accounts.lending_market.clone(),
            accounts.reserve.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_RESERVE_CONFIG_IX_ACCOUNTS_LEN]>
for UpdateReserveConfigAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_RESERVE_CONFIG_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lending_market_owner: &arr[0],
            lending_market: &arr[1],
            reserve: &arr[2],
        }
    }
}
pub const UPDATE_RESERVE_CONFIG_IX_DISCM: [u8; 8] = [61, 148, 100, 70, 143, 107, 17, 13];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateReserveConfigIxArgs {
    pub mode: u64,
    pub value: bytes,
    pub skip_validation: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateReserveConfigIxData(pub UpdateReserveConfigIxArgs);
impl From<UpdateReserveConfigIxArgs> for UpdateReserveConfigIxData {
    fn from(args: UpdateReserveConfigIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateReserveConfigIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_RESERVE_CONFIG_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_RESERVE_CONFIG_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateReserveConfigIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_RESERVE_CONFIG_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_reserve_config_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateReserveConfigKeys,
    args: UpdateReserveConfigIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_RESERVE_CONFIG_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateReserveConfigIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_reserve_config_ix(
    keys: UpdateReserveConfigKeys,
    args: UpdateReserveConfigIxArgs,
) -> std::io::Result<Instruction> {
    update_reserve_config_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_reserve_config_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateReserveConfigAccounts<'_, '_>,
    args: UpdateReserveConfigIxArgs,
) -> ProgramResult {
    let keys: UpdateReserveConfigKeys = accounts.into();
    let ix = update_reserve_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_reserve_config_invoke(
    accounts: UpdateReserveConfigAccounts<'_, '_>,
    args: UpdateReserveConfigIxArgs,
) -> ProgramResult {
    update_reserve_config_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_reserve_config_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateReserveConfigAccounts<'_, '_>,
    args: UpdateReserveConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateReserveConfigKeys = accounts.into();
    let ix = update_reserve_config_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_reserve_config_invoke_signed(
    accounts: UpdateReserveConfigAccounts<'_, '_>,
    args: UpdateReserveConfigIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_reserve_config_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_reserve_config_verify_account_keys(
    accounts: UpdateReserveConfigAccounts<'_, '_>,
    keys: UpdateReserveConfigKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lending_market_owner.key, keys.lending_market_owner),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.reserve.key, keys.reserve),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_reserve_config_verify_writable_privileges<'me, 'info>(
    accounts: UpdateReserveConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.reserve] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_reserve_config_verify_signer_privileges<'me, 'info>(
    accounts: UpdateReserveConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lending_market_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_reserve_config_verify_account_privileges<'me, 'info>(
    accounts: UpdateReserveConfigAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_reserve_config_verify_writable_privileges(accounts)?;
    update_reserve_config_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_FEES_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct RedeemFeesAccounts<'me, 'info> {
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_fee_receiver: &'me AccountInfo<'info>,
    pub reserve_supply_liquidity: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemFeesKeys {
    pub reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_liquidity_fee_receiver: Pubkey,
    pub reserve_supply_liquidity: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<RedeemFeesAccounts<'_, '_>> for RedeemFeesKeys {
    fn from(accounts: RedeemFeesAccounts) -> Self {
        Self {
            reserve: *accounts.reserve.key,
            reserve_liquidity_mint: *accounts.reserve_liquidity_mint.key,
            reserve_liquidity_fee_receiver: *accounts.reserve_liquidity_fee_receiver.key,
            reserve_supply_liquidity: *accounts.reserve_supply_liquidity.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<RedeemFeesKeys> for [AccountMeta; REDEEM_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_fee_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_supply_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REDEEM_FEES_IX_ACCOUNTS_LEN]> for RedeemFeesKeys {
    fn from(pubkeys: [Pubkey; REDEEM_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reserve: pubkeys[0],
            reserve_liquidity_mint: pubkeys[1],
            reserve_liquidity_fee_receiver: pubkeys[2],
            reserve_supply_liquidity: pubkeys[3],
            lending_market: pubkeys[4],
            lending_market_authority: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<RedeemFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.reserve.clone(),
            accounts.reserve_liquidity_mint.clone(),
            accounts.reserve_liquidity_fee_receiver.clone(),
            accounts.reserve_supply_liquidity.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_FEES_IX_ACCOUNTS_LEN]>
for RedeemFeesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REDEEM_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reserve: &arr[0],
            reserve_liquidity_mint: &arr[1],
            reserve_liquidity_fee_receiver: &arr[2],
            reserve_supply_liquidity: &arr[3],
            lending_market: &arr[4],
            lending_market_authority: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const REDEEM_FEES_IX_DISCM: [u8; 8] = [215, 39, 180, 41, 173, 46, 248, 220];
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemFeesIxData;
impl RedeemFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_FEES_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REDEEM_FEES_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RedeemFeesIxData.try_to_vec()?,
    })
}
pub fn redeem_fees_ix(keys: RedeemFeesKeys) -> std::io::Result<Instruction> {
    redeem_fees_ix_with_program_id(crate::ID, keys)
}
pub fn redeem_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RedeemFeesKeys = accounts.into();
    let ix = redeem_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_fees_invoke(accounts: RedeemFeesAccounts<'_, '_>) -> ProgramResult {
    redeem_fees_invoke_with_program_id(crate::ID, accounts)
}
pub fn redeem_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemFeesKeys = accounts.into();
    let ix = redeem_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_fees_invoke_signed(
    accounts: RedeemFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_fees_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn redeem_fees_verify_account_keys(
    accounts: RedeemFeesAccounts<'_, '_>,
    keys: RedeemFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_liquidity_mint.key, keys.reserve_liquidity_mint),
        (
            *accounts.reserve_liquidity_fee_receiver.key,
            keys.reserve_liquidity_fee_receiver,
        ),
        (*accounts.reserve_supply_liquidity.key, keys.reserve_supply_liquidity),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_fees_verify_writable_privileges<'me, 'info>(
    accounts: RedeemFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.reserve,
        accounts.reserve_liquidity_fee_receiver,
        accounts.reserve_supply_liquidity,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_fees_verify_account_privileges<'me, 'info>(
    accounts: RedeemFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_fees_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawProtocolFeeAccounts<'me, 'info> {
    pub lending_market_owner: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub fee_vault: &'me AccountInfo<'info>,
    pub lending_market_owner_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeeKeys {
    pub lending_market_owner: Pubkey,
    pub lending_market: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub lending_market_authority: Pubkey,
    pub fee_vault: Pubkey,
    pub lending_market_owner_ata: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawProtocolFeeAccounts<'_, '_>> for WithdrawProtocolFeeKeys {
    fn from(accounts: WithdrawProtocolFeeAccounts) -> Self {
        Self {
            lending_market_owner: *accounts.lending_market_owner.key,
            lending_market: *accounts.lending_market.key,
            reserve: *accounts.reserve.key,
            reserve_liquidity_mint: *accounts.reserve_liquidity_mint.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            fee_vault: *accounts.fee_vault.key,
            lending_market_owner_ata: *accounts.lending_market_owner_ata.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawProtocolFeeKeys>
for [AccountMeta; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawProtocolFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lending_market_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market_owner_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for WithdrawProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market_owner: pubkeys[0],
            lending_market: pubkeys[1],
            reserve: pubkeys[2],
            reserve_liquidity_mint: pubkeys[3],
            lending_market_authority: pubkeys[4],
            fee_vault: pubkeys[5],
            lending_market_owner_ata: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<WithdrawProtocolFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.lending_market_owner.clone(),
            accounts.lending_market.clone(),
            accounts.reserve.clone(),
            accounts.reserve_liquidity_mint.clone(),
            accounts.lending_market_authority.clone(),
            accounts.fee_vault.clone(),
            accounts.lending_market_owner_ata.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
for WithdrawProtocolFeeAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            lending_market_owner: &arr[0],
            lending_market: &arr[1],
            reserve: &arr[2],
            reserve_liquidity_mint: &arr[3],
            lending_market_authority: &arr[4],
            fee_vault: &arr[5],
            lending_market_owner_ata: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const WITHDRAW_PROTOCOL_FEE_IX_DISCM: [u8; 8] = [
    158,
    201,
    158,
    189,
    33,
    93,
    162,
    103,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawProtocolFeeIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawProtocolFeeIxData(pub WithdrawProtocolFeeIxArgs);
impl From<WithdrawProtocolFeeIxArgs> for WithdrawProtocolFeeIxData {
    fn from(args: WithdrawProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawProtocolFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_PROTOCOL_FEE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_PROTOCOL_FEE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WithdrawProtocolFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_PROTOCOL_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_protocol_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawProtocolFeeKeys,
    args: WithdrawProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: WithdrawProtocolFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_protocol_fee_ix(
    keys: WithdrawProtocolFeeKeys,
    args: WithdrawProtocolFeeIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_protocol_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn withdraw_protocol_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
) -> ProgramResult {
    let keys: WithdrawProtocolFeeKeys = accounts.into();
    let ix = withdraw_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_protocol_fee_invoke(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
) -> ProgramResult {
    withdraw_protocol_fee_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn withdraw_protocol_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawProtocolFeeKeys = accounts.into();
    let ix = withdraw_protocol_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_protocol_fee_invoke_signed(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    args: WithdrawProtocolFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_protocol_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn withdraw_protocol_fee_verify_account_keys(
    accounts: WithdrawProtocolFeeAccounts<'_, '_>,
    keys: WithdrawProtocolFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lending_market_owner.key, keys.lending_market_owner),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_liquidity_mint.key, keys.reserve_liquidity_mint),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.fee_vault.key, keys.fee_vault),
        (*accounts.lending_market_owner_ata.key, keys.lending_market_owner_ata),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fee_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_vault, accounts.lending_market_owner_ata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fee_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lending_market_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_protocol_fee_verify_account_privileges<'me, 'info>(
    accounts: WithdrawProtocolFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_protocol_fee_verify_writable_privileges(accounts)?;
    withdraw_protocol_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SOCIALIZE_LOSS_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SocializeLossAccounts<'me, 'info> {
    pub risk_council: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SocializeLossKeys {
    pub risk_council: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub reserve: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<SocializeLossAccounts<'_, '_>> for SocializeLossKeys {
    fn from(accounts: SocializeLossAccounts) -> Self {
        Self {
            risk_council: *accounts.risk_council.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            reserve: *accounts.reserve.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<SocializeLossKeys> for [AccountMeta; SOCIALIZE_LOSS_IX_ACCOUNTS_LEN] {
    fn from(keys: SocializeLossKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.risk_council,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SOCIALIZE_LOSS_IX_ACCOUNTS_LEN]> for SocializeLossKeys {
    fn from(pubkeys: [Pubkey; SOCIALIZE_LOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            risk_council: pubkeys[0],
            obligation: pubkeys[1],
            lending_market: pubkeys[2],
            reserve: pubkeys[3],
            instruction_sysvar_account: pubkeys[4],
        }
    }
}
impl<'info> From<SocializeLossAccounts<'_, 'info>>
for [AccountInfo<'info>; SOCIALIZE_LOSS_IX_ACCOUNTS_LEN] {
    fn from(accounts: SocializeLossAccounts<'_, 'info>) -> Self {
        [
            accounts.risk_council.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.reserve.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SOCIALIZE_LOSS_IX_ACCOUNTS_LEN]>
for SocializeLossAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SOCIALIZE_LOSS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            risk_council: &arr[0],
            obligation: &arr[1],
            lending_market: &arr[2],
            reserve: &arr[3],
            instruction_sysvar_account: &arr[4],
        }
    }
}
pub const SOCIALIZE_LOSS_IX_DISCM: [u8; 8] = [245, 75, 91, 0, 236, 97, 19, 3];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SocializeLossIxArgs {
    pub liquidity_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SocializeLossIxData(pub SocializeLossIxArgs);
impl From<SocializeLossIxArgs> for SocializeLossIxData {
    fn from(args: SocializeLossIxArgs) -> Self {
        Self(args)
    }
}
impl SocializeLossIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SOCIALIZE_LOSS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SOCIALIZE_LOSS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SocializeLossIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SOCIALIZE_LOSS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn socialize_loss_ix_with_program_id(
    program_id: Pubkey,
    keys: SocializeLossKeys,
    args: SocializeLossIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SOCIALIZE_LOSS_IX_ACCOUNTS_LEN] = keys.into();
    let data: SocializeLossIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn socialize_loss_ix(
    keys: SocializeLossKeys,
    args: SocializeLossIxArgs,
) -> std::io::Result<Instruction> {
    socialize_loss_ix_with_program_id(crate::ID, keys, args)
}
pub fn socialize_loss_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SocializeLossAccounts<'_, '_>,
    args: SocializeLossIxArgs,
) -> ProgramResult {
    let keys: SocializeLossKeys = accounts.into();
    let ix = socialize_loss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn socialize_loss_invoke(
    accounts: SocializeLossAccounts<'_, '_>,
    args: SocializeLossIxArgs,
) -> ProgramResult {
    socialize_loss_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn socialize_loss_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SocializeLossAccounts<'_, '_>,
    args: SocializeLossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SocializeLossKeys = accounts.into();
    let ix = socialize_loss_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn socialize_loss_invoke_signed(
    accounts: SocializeLossAccounts<'_, '_>,
    args: SocializeLossIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    socialize_loss_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn socialize_loss_verify_account_keys(
    accounts: SocializeLossAccounts<'_, '_>,
    keys: SocializeLossKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.risk_council.key, keys.risk_council),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn socialize_loss_verify_writable_privileges<'me, 'info>(
    accounts: SocializeLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.obligation, accounts.reserve] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn socialize_loss_verify_signer_privileges<'me, 'info>(
    accounts: SocializeLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.risk_council] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn socialize_loss_verify_account_privileges<'me, 'info>(
    accounts: SocializeLossAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    socialize_loss_verify_writable_privileges(accounts)?;
    socialize_loss_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SOCIALIZE_LOSS_V2_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct SocializeLossV2Accounts<'me, 'info> {
    pub socialize_loss_accounts_risk_council: &'me AccountInfo<'info>,
    pub socialize_loss_accounts_obligation: &'me AccountInfo<'info>,
    pub socialize_loss_accounts_lending_market: &'me AccountInfo<'info>,
    pub socialize_loss_accounts_reserve: &'me AccountInfo<'info>,
    pub socialize_loss_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SocializeLossV2Keys {
    pub socialize_loss_accounts_risk_council: Pubkey,
    pub socialize_loss_accounts_obligation: Pubkey,
    pub socialize_loss_accounts_lending_market: Pubkey,
    pub socialize_loss_accounts_reserve: Pubkey,
    pub socialize_loss_accounts_instruction_sysvar_account: Pubkey,
    pub farms_accounts_obligation_farm_user_state: Pubkey,
    pub farms_accounts_reserve_farm_state: Pubkey,
    pub lending_market_authority: Pubkey,
    pub farms_program: Pubkey,
}
impl From<SocializeLossV2Accounts<'_, '_>> for SocializeLossV2Keys {
    fn from(accounts: SocializeLossV2Accounts) -> Self {
        Self {
            socialize_loss_accounts_risk_council: *accounts
                .socialize_loss_accounts_risk_council
                .key,
            socialize_loss_accounts_obligation: *accounts
                .socialize_loss_accounts_obligation
                .key,
            socialize_loss_accounts_lending_market: *accounts
                .socialize_loss_accounts_lending_market
                .key,
            socialize_loss_accounts_reserve: *accounts
                .socialize_loss_accounts_reserve
                .key,
            socialize_loss_accounts_instruction_sysvar_account: *accounts
                .socialize_loss_accounts_instruction_sysvar_account
                .key,
            farms_accounts_obligation_farm_user_state: *accounts
                .farms_accounts_obligation_farm_user_state
                .key,
            farms_accounts_reserve_farm_state: *accounts
                .farms_accounts_reserve_farm_state
                .key,
            lending_market_authority: *accounts.lending_market_authority.key,
            farms_program: *accounts.farms_program.key,
        }
    }
}
impl From<SocializeLossV2Keys> for [AccountMeta; SOCIALIZE_LOSS_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: SocializeLossV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.socialize_loss_accounts_risk_council,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.socialize_loss_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.socialize_loss_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.socialize_loss_accounts_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.socialize_loss_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SOCIALIZE_LOSS_V2_IX_ACCOUNTS_LEN]> for SocializeLossV2Keys {
    fn from(pubkeys: [Pubkey; SOCIALIZE_LOSS_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            socialize_loss_accounts_risk_council: pubkeys[0],
            socialize_loss_accounts_obligation: pubkeys[1],
            socialize_loss_accounts_lending_market: pubkeys[2],
            socialize_loss_accounts_reserve: pubkeys[3],
            socialize_loss_accounts_instruction_sysvar_account: pubkeys[4],
            farms_accounts_obligation_farm_user_state: pubkeys[5],
            farms_accounts_reserve_farm_state: pubkeys[6],
            lending_market_authority: pubkeys[7],
            farms_program: pubkeys[8],
        }
    }
}
impl<'info> From<SocializeLossV2Accounts<'_, 'info>>
for [AccountInfo<'info>; SOCIALIZE_LOSS_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: SocializeLossV2Accounts<'_, 'info>) -> Self {
        [
            accounts.socialize_loss_accounts_risk_council.clone(),
            accounts.socialize_loss_accounts_obligation.clone(),
            accounts.socialize_loss_accounts_lending_market.clone(),
            accounts.socialize_loss_accounts_reserve.clone(),
            accounts.socialize_loss_accounts_instruction_sysvar_account.clone(),
            accounts.farms_accounts_obligation_farm_user_state.clone(),
            accounts.farms_accounts_reserve_farm_state.clone(),
            accounts.lending_market_authority.clone(),
            accounts.farms_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SOCIALIZE_LOSS_V2_IX_ACCOUNTS_LEN]>
for SocializeLossV2Accounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; SOCIALIZE_LOSS_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            socialize_loss_accounts_risk_council: &arr[0],
            socialize_loss_accounts_obligation: &arr[1],
            socialize_loss_accounts_lending_market: &arr[2],
            socialize_loss_accounts_reserve: &arr[3],
            socialize_loss_accounts_instruction_sysvar_account: &arr[4],
            farms_accounts_obligation_farm_user_state: &arr[5],
            farms_accounts_reserve_farm_state: &arr[6],
            lending_market_authority: &arr[7],
            farms_program: &arr[8],
        }
    }
}
pub const SOCIALIZE_LOSS_V2_IX_DISCM: [u8; 8] = [238, 95, 98, 220, 187, 40, 204, 154];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SocializeLossV2IxArgs {
    pub liquidity_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SocializeLossV2IxData(pub SocializeLossV2IxArgs);
impl From<SocializeLossV2IxArgs> for SocializeLossV2IxData {
    fn from(args: SocializeLossV2IxArgs) -> Self {
        Self(args)
    }
}
impl SocializeLossV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SOCIALIZE_LOSS_V2_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SOCIALIZE_LOSS_V2_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SocializeLossV2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SOCIALIZE_LOSS_V2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn socialize_loss_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: SocializeLossV2Keys,
    args: SocializeLossV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SOCIALIZE_LOSS_V2_IX_ACCOUNTS_LEN] = keys.into();
    let data: SocializeLossV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn socialize_loss_v2_ix(
    keys: SocializeLossV2Keys,
    args: SocializeLossV2IxArgs,
) -> std::io::Result<Instruction> {
    socialize_loss_v2_ix_with_program_id(crate::ID, keys, args)
}
pub fn socialize_loss_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SocializeLossV2Accounts<'_, '_>,
    args: SocializeLossV2IxArgs,
) -> ProgramResult {
    let keys: SocializeLossV2Keys = accounts.into();
    let ix = socialize_loss_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn socialize_loss_v2_invoke(
    accounts: SocializeLossV2Accounts<'_, '_>,
    args: SocializeLossV2IxArgs,
) -> ProgramResult {
    socialize_loss_v2_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn socialize_loss_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SocializeLossV2Accounts<'_, '_>,
    args: SocializeLossV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SocializeLossV2Keys = accounts.into();
    let ix = socialize_loss_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn socialize_loss_v2_invoke_signed(
    accounts: SocializeLossV2Accounts<'_, '_>,
    args: SocializeLossV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    socialize_loss_v2_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn socialize_loss_v2_verify_account_keys(
    accounts: SocializeLossV2Accounts<'_, '_>,
    keys: SocializeLossV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            *accounts.socialize_loss_accounts_risk_council.key,
            keys.socialize_loss_accounts_risk_council,
        ),
        (
            *accounts.socialize_loss_accounts_obligation.key,
            keys.socialize_loss_accounts_obligation,
        ),
        (
            *accounts.socialize_loss_accounts_lending_market.key,
            keys.socialize_loss_accounts_lending_market,
        ),
        (
            *accounts.socialize_loss_accounts_reserve.key,
            keys.socialize_loss_accounts_reserve,
        ),
        (
            *accounts.socialize_loss_accounts_instruction_sysvar_account.key,
            keys.socialize_loss_accounts_instruction_sysvar_account,
        ),
        (
            *accounts.farms_accounts_obligation_farm_user_state.key,
            keys.farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.farms_accounts_reserve_farm_state.key,
            keys.farms_accounts_reserve_farm_state,
        ),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.farms_program.key, keys.farms_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn socialize_loss_v2_verify_writable_privileges<'me, 'info>(
    accounts: SocializeLossV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.socialize_loss_accounts_obligation,
        accounts.socialize_loss_accounts_reserve,
        accounts.farms_accounts_obligation_farm_user_state,
        accounts.farms_accounts_reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn socialize_loss_v2_verify_signer_privileges<'me, 'info>(
    accounts: SocializeLossV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.socialize_loss_accounts_risk_council] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn socialize_loss_v2_verify_account_privileges<'me, 'info>(
    accounts: SocializeLossV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    socialize_loss_v2_verify_writable_privileges(accounts)?;
    socialize_loss_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MARK_OBLIGATION_FOR_DELEVERAGING_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct MarkObligationForDeleveragingAccounts<'me, 'info> {
    pub risk_council: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarkObligationForDeleveragingKeys {
    pub risk_council: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
}
impl From<MarkObligationForDeleveragingAccounts<'_, '_>>
for MarkObligationForDeleveragingKeys {
    fn from(accounts: MarkObligationForDeleveragingAccounts) -> Self {
        Self {
            risk_council: *accounts.risk_council.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
        }
    }
}
impl From<MarkObligationForDeleveragingKeys>
for [AccountMeta; MARK_OBLIGATION_FOR_DELEVERAGING_IX_ACCOUNTS_LEN] {
    fn from(keys: MarkObligationForDeleveragingKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.risk_council,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARK_OBLIGATION_FOR_DELEVERAGING_IX_ACCOUNTS_LEN]>
for MarkObligationForDeleveragingKeys {
    fn from(
        pubkeys: [Pubkey; MARK_OBLIGATION_FOR_DELEVERAGING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            risk_council: pubkeys[0],
            obligation: pubkeys[1],
            lending_market: pubkeys[2],
        }
    }
}
impl<'info> From<MarkObligationForDeleveragingAccounts<'_, 'info>>
for [AccountInfo<'info>; MARK_OBLIGATION_FOR_DELEVERAGING_IX_ACCOUNTS_LEN] {
    fn from(accounts: MarkObligationForDeleveragingAccounts<'_, 'info>) -> Self {
        [
            accounts.risk_council.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; MARK_OBLIGATION_FOR_DELEVERAGING_IX_ACCOUNTS_LEN]>
for MarkObligationForDeleveragingAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; MARK_OBLIGATION_FOR_DELEVERAGING_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            risk_council: &arr[0],
            obligation: &arr[1],
            lending_market: &arr[2],
        }
    }
}
pub const MARK_OBLIGATION_FOR_DELEVERAGING_IX_DISCM: [u8; 8] = [
    164,
    35,
    182,
    19,
    0,
    116,
    243,
    127,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarkObligationForDeleveragingIxArgs {
    pub autodeleverage_target_ltv_pct: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MarkObligationForDeleveragingIxData(pub MarkObligationForDeleveragingIxArgs);
impl From<MarkObligationForDeleveragingIxArgs> for MarkObligationForDeleveragingIxData {
    fn from(args: MarkObligationForDeleveragingIxArgs) -> Self {
        Self(args)
    }
}
impl MarkObligationForDeleveragingIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARK_OBLIGATION_FOR_DELEVERAGING_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        MARK_OBLIGATION_FOR_DELEVERAGING_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(MarkObligationForDeleveragingIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARK_OBLIGATION_FOR_DELEVERAGING_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mark_obligation_for_deleveraging_ix_with_program_id(
    program_id: Pubkey,
    keys: MarkObligationForDeleveragingKeys,
    args: MarkObligationForDeleveragingIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARK_OBLIGATION_FOR_DELEVERAGING_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: MarkObligationForDeleveragingIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mark_obligation_for_deleveraging_ix(
    keys: MarkObligationForDeleveragingKeys,
    args: MarkObligationForDeleveragingIxArgs,
) -> std::io::Result<Instruction> {
    mark_obligation_for_deleveraging_ix_with_program_id(crate::ID, keys, args)
}
pub fn mark_obligation_for_deleveraging_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarkObligationForDeleveragingAccounts<'_, '_>,
    args: MarkObligationForDeleveragingIxArgs,
) -> ProgramResult {
    let keys: MarkObligationForDeleveragingKeys = accounts.into();
    let ix = mark_obligation_for_deleveraging_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn mark_obligation_for_deleveraging_invoke(
    accounts: MarkObligationForDeleveragingAccounts<'_, '_>,
    args: MarkObligationForDeleveragingIxArgs,
) -> ProgramResult {
    mark_obligation_for_deleveraging_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn mark_obligation_for_deleveraging_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarkObligationForDeleveragingAccounts<'_, '_>,
    args: MarkObligationForDeleveragingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarkObligationForDeleveragingKeys = accounts.into();
    let ix = mark_obligation_for_deleveraging_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mark_obligation_for_deleveraging_invoke_signed(
    accounts: MarkObligationForDeleveragingAccounts<'_, '_>,
    args: MarkObligationForDeleveragingIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mark_obligation_for_deleveraging_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn mark_obligation_for_deleveraging_verify_account_keys(
    accounts: MarkObligationForDeleveragingAccounts<'_, '_>,
    keys: MarkObligationForDeleveragingKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.risk_council.key, keys.risk_council),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mark_obligation_for_deleveraging_verify_writable_privileges<'me, 'info>(
    accounts: MarkObligationForDeleveragingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.obligation] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mark_obligation_for_deleveraging_verify_signer_privileges<'me, 'info>(
    accounts: MarkObligationForDeleveragingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.risk_council] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn mark_obligation_for_deleveraging_verify_account_privileges<'me, 'info>(
    accounts: MarkObligationForDeleveragingAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mark_obligation_for_deleveraging_verify_writable_privileges(accounts)?;
    mark_obligation_for_deleveraging_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REFRESH_RESERVE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct RefreshReserveAccounts<'me, 'info> {
    pub reserve: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub pyth_oracle: &'me AccountInfo<'info>,
    pub switchboard_price_oracle: &'me AccountInfo<'info>,
    pub switchboard_twap_oracle: &'me AccountInfo<'info>,
    pub scope_prices: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshReserveKeys {
    pub reserve: Pubkey,
    pub lending_market: Pubkey,
    pub pyth_oracle: Pubkey,
    pub switchboard_price_oracle: Pubkey,
    pub switchboard_twap_oracle: Pubkey,
    pub scope_prices: Pubkey,
}
impl From<RefreshReserveAccounts<'_, '_>> for RefreshReserveKeys {
    fn from(accounts: RefreshReserveAccounts) -> Self {
        Self {
            reserve: *accounts.reserve.key,
            lending_market: *accounts.lending_market.key,
            pyth_oracle: *accounts.pyth_oracle.key,
            switchboard_price_oracle: *accounts.switchboard_price_oracle.key,
            switchboard_twap_oracle: *accounts.switchboard_twap_oracle.key,
            scope_prices: *accounts.scope_prices.key,
        }
    }
}
impl From<RefreshReserveKeys> for [AccountMeta; REFRESH_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(keys: RefreshReserveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.switchboard_price_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.switchboard_twap_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.scope_prices,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REFRESH_RESERVE_IX_ACCOUNTS_LEN]> for RefreshReserveKeys {
    fn from(pubkeys: [Pubkey; REFRESH_RESERVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reserve: pubkeys[0],
            lending_market: pubkeys[1],
            pyth_oracle: pubkeys[2],
            switchboard_price_oracle: pubkeys[3],
            switchboard_twap_oracle: pubkeys[4],
            scope_prices: pubkeys[5],
        }
    }
}
impl<'info> From<RefreshReserveAccounts<'_, 'info>>
for [AccountInfo<'info>; REFRESH_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefreshReserveAccounts<'_, 'info>) -> Self {
        [
            accounts.reserve.clone(),
            accounts.lending_market.clone(),
            accounts.pyth_oracle.clone(),
            accounts.switchboard_price_oracle.clone(),
            accounts.switchboard_twap_oracle.clone(),
            accounts.scope_prices.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REFRESH_RESERVE_IX_ACCOUNTS_LEN]>
for RefreshReserveAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REFRESH_RESERVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reserve: &arr[0],
            lending_market: &arr[1],
            pyth_oracle: &arr[2],
            switchboard_price_oracle: &arr[3],
            switchboard_twap_oracle: &arr[4],
            scope_prices: &arr[5],
        }
    }
}
pub const REFRESH_RESERVE_IX_DISCM: [u8; 8] = [2, 218, 138, 235, 79, 201, 25, 102];
#[derive(Clone, Debug, PartialEq)]
pub struct RefreshReserveIxData;
impl RefreshReserveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_RESERVE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REFRESH_RESERVE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_RESERVE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refresh_reserve_ix_with_program_id(
    program_id: Pubkey,
    keys: RefreshReserveKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFRESH_RESERVE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RefreshReserveIxData.try_to_vec()?,
    })
}
pub fn refresh_reserve_ix(keys: RefreshReserveKeys) -> std::io::Result<Instruction> {
    refresh_reserve_ix_with_program_id(crate::ID, keys)
}
pub fn refresh_reserve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefreshReserveAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RefreshReserveKeys = accounts.into();
    let ix = refresh_reserve_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn refresh_reserve_invoke(
    accounts: RefreshReserveAccounts<'_, '_>,
) -> ProgramResult {
    refresh_reserve_invoke_with_program_id(crate::ID, accounts)
}
pub fn refresh_reserve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefreshReserveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefreshReserveKeys = accounts.into();
    let ix = refresh_reserve_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refresh_reserve_invoke_signed(
    accounts: RefreshReserveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_reserve_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn refresh_reserve_verify_account_keys(
    accounts: RefreshReserveAccounts<'_, '_>,
    keys: RefreshReserveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.reserve.key, keys.reserve),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.pyth_oracle.key, keys.pyth_oracle),
        (*accounts.switchboard_price_oracle.key, keys.switchboard_price_oracle),
        (*accounts.switchboard_twap_oracle.key, keys.switchboard_twap_oracle),
        (*accounts.scope_prices.key, keys.scope_prices),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refresh_reserve_verify_writable_privileges<'me, 'info>(
    accounts: RefreshReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.reserve] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refresh_reserve_verify_account_privileges<'me, 'info>(
    accounts: RefreshReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refresh_reserve_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const REFRESH_RESERVES_BATCH_IX_DISCM: [u8; 8] = [
    144,
    110,
    26,
    103,
    162,
    204,
    252,
    147,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefreshReservesBatchIxArgs {
    pub skip_price_updates: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RefreshReservesBatchIxData(pub RefreshReservesBatchIxArgs);
impl From<RefreshReservesBatchIxArgs> for RefreshReservesBatchIxData {
    fn from(args: RefreshReservesBatchIxArgs) -> Self {
        Self(args)
    }
}
impl RefreshReservesBatchIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_RESERVES_BATCH_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REFRESH_RESERVES_BATCH_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RefreshReservesBatchIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_RESERVES_BATCH_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refresh_reserves_batch_ix_with_program_id(
    program_id: Pubkey,
    args: RefreshReservesBatchIxArgs,
) -> std::io::Result<Instruction> {
    let data: RefreshReservesBatchIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::new(),
        data: data.try_to_vec()?,
    })
}
pub fn refresh_reserves_batch_ix(
    args: RefreshReservesBatchIxArgs,
) -> std::io::Result<Instruction> {
    refresh_reserves_batch_ix_with_program_id(crate::ID, args)
}
pub fn refresh_reserves_batch_invoke_with_program_id(
    program_id: Pubkey,
    args: RefreshReservesBatchIxArgs,
) -> ProgramResult {
    let ix = refresh_reserves_batch_ix_with_program_id(program_id, args)?;
    invoke(&ix, &[])
}
pub fn refresh_reserves_batch_invoke(args: RefreshReservesBatchIxArgs) -> ProgramResult {
    refresh_reserves_batch_invoke_with_program_id(crate::ID, args)
}
pub fn refresh_reserves_batch_invoke_signed_with_program_id(
    program_id: Pubkey,
    args: RefreshReservesBatchIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = refresh_reserves_batch_ix_with_program_id(program_id, args)?;
    invoke_signed(&ix, &[], seeds)
}
pub fn refresh_reserves_batch_invoke_signed(
    args: RefreshReservesBatchIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_reserves_batch_invoke_signed_with_program_id(crate::ID, args, seeds)
}
pub const DEPOSIT_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct DepositReserveLiquidityAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub user_source_liquidity: &'me AccountInfo<'info>,
    pub user_destination_collateral: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub liquidity_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositReserveLiquidityKeys {
    pub owner: Pubkey,
    pub reserve: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub user_source_liquidity: Pubkey,
    pub user_destination_collateral: Pubkey,
    pub collateral_token_program: Pubkey,
    pub liquidity_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<DepositReserveLiquidityAccounts<'_, '_>> for DepositReserveLiquidityKeys {
    fn from(accounts: DepositReserveLiquidityAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            reserve: *accounts.reserve.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            reserve_liquidity_mint: *accounts.reserve_liquidity_mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            user_source_liquidity: *accounts.user_source_liquidity.key,
            user_destination_collateral: *accounts.user_destination_collateral.key,
            collateral_token_program: *accounts.collateral_token_program.key,
            liquidity_token_program: *accounts.liquidity_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<DepositReserveLiquidityKeys>
for [AccountMeta; DEPOSIT_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositReserveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for DepositReserveLiquidityKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            reserve: pubkeys[1],
            lending_market: pubkeys[2],
            lending_market_authority: pubkeys[3],
            reserve_liquidity_mint: pubkeys[4],
            reserve_liquidity_supply: pubkeys[5],
            reserve_collateral_mint: pubkeys[6],
            user_source_liquidity: pubkeys[7],
            user_destination_collateral: pubkeys[8],
            collateral_token_program: pubkeys[9],
            liquidity_token_program: pubkeys[10],
            instruction_sysvar_account: pubkeys[11],
        }
    }
}
impl<'info> From<DepositReserveLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositReserveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.reserve.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.reserve_liquidity_mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.user_source_liquidity.clone(),
            accounts.user_destination_collateral.clone(),
            accounts.collateral_token_program.clone(),
            accounts.liquidity_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DEPOSIT_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for DepositReserveLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            reserve: &arr[1],
            lending_market: &arr[2],
            lending_market_authority: &arr[3],
            reserve_liquidity_mint: &arr[4],
            reserve_liquidity_supply: &arr[5],
            reserve_collateral_mint: &arr[6],
            user_source_liquidity: &arr[7],
            user_destination_collateral: &arr[8],
            collateral_token_program: &arr[9],
            liquidity_token_program: &arr[10],
            instruction_sysvar_account: &arr[11],
        }
    }
}
pub const DEPOSIT_RESERVE_LIQUIDITY_IX_DISCM: [u8; 8] = [
    169,
    201,
    30,
    126,
    6,
    205,
    102,
    68,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositReserveLiquidityIxArgs {
    pub liquidity_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositReserveLiquidityIxData(pub DepositReserveLiquidityIxArgs);
impl From<DepositReserveLiquidityIxArgs> for DepositReserveLiquidityIxData {
    fn from(args: DepositReserveLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl DepositReserveLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_RESERVE_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DEPOSIT_RESERVE_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(DepositReserveLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_RESERVE_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_reserve_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositReserveLiquidityKeys,
    args: DepositReserveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositReserveLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_reserve_liquidity_ix(
    keys: DepositReserveLiquidityKeys,
    args: DepositReserveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    deposit_reserve_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn deposit_reserve_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositReserveLiquidityAccounts<'_, '_>,
    args: DepositReserveLiquidityIxArgs,
) -> ProgramResult {
    let keys: DepositReserveLiquidityKeys = accounts.into();
    let ix = deposit_reserve_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_reserve_liquidity_invoke(
    accounts: DepositReserveLiquidityAccounts<'_, '_>,
    args: DepositReserveLiquidityIxArgs,
) -> ProgramResult {
    deposit_reserve_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn deposit_reserve_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositReserveLiquidityAccounts<'_, '_>,
    args: DepositReserveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositReserveLiquidityKeys = accounts.into();
    let ix = deposit_reserve_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_reserve_liquidity_invoke_signed(
    accounts: DepositReserveLiquidityAccounts<'_, '_>,
    args: DepositReserveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_reserve_liquidity_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_reserve_liquidity_verify_account_keys(
    accounts: DepositReserveLiquidityAccounts<'_, '_>,
    keys: DepositReserveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.reserve_liquidity_mint.key, keys.reserve_liquidity_mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.user_source_liquidity.key, keys.user_source_liquidity),
        (*accounts.user_destination_collateral.key, keys.user_destination_collateral),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (*accounts.liquidity_token_program.key, keys.liquidity_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_reserve_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: DepositReserveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.reserve,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.user_source_liquidity,
        accounts.user_destination_collateral,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_reserve_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: DepositReserveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_reserve_liquidity_verify_account_privileges<'me, 'info>(
    accounts: DepositReserveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_reserve_liquidity_verify_writable_privileges(accounts)?;
    deposit_reserve_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct RedeemReserveCollateralAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub user_source_collateral: &'me AccountInfo<'info>,
    pub user_destination_liquidity: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub liquidity_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemReserveCollateralKeys {
    pub owner: Pubkey,
    pub lending_market: Pubkey,
    pub reserve: Pubkey,
    pub lending_market_authority: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub user_source_collateral: Pubkey,
    pub user_destination_liquidity: Pubkey,
    pub collateral_token_program: Pubkey,
    pub liquidity_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<RedeemReserveCollateralAccounts<'_, '_>> for RedeemReserveCollateralKeys {
    fn from(accounts: RedeemReserveCollateralAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            lending_market: *accounts.lending_market.key,
            reserve: *accounts.reserve.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            reserve_liquidity_mint: *accounts.reserve_liquidity_mint.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            user_source_collateral: *accounts.user_source_collateral.key,
            user_destination_liquidity: *accounts.user_destination_liquidity.key,
            collateral_token_program: *accounts.collateral_token_program.key,
            liquidity_token_program: *accounts.liquidity_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<RedeemReserveCollateralKeys>
for [AccountMeta; REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemReserveCollateralKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN]>
for RedeemReserveCollateralKeys {
    fn from(pubkeys: [Pubkey; REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            lending_market: pubkeys[1],
            reserve: pubkeys[2],
            lending_market_authority: pubkeys[3],
            reserve_liquidity_mint: pubkeys[4],
            reserve_collateral_mint: pubkeys[5],
            reserve_liquidity_supply: pubkeys[6],
            user_source_collateral: pubkeys[7],
            user_destination_liquidity: pubkeys[8],
            collateral_token_program: pubkeys[9],
            liquidity_token_program: pubkeys[10],
            instruction_sysvar_account: pubkeys[11],
        }
    }
}
impl<'info> From<RedeemReserveCollateralAccounts<'_, 'info>>
for [AccountInfo<'info>; REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemReserveCollateralAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.lending_market.clone(),
            accounts.reserve.clone(),
            accounts.lending_market_authority.clone(),
            accounts.reserve_liquidity_mint.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.user_source_collateral.clone(),
            accounts.user_destination_liquidity.clone(),
            accounts.collateral_token_program.clone(),
            accounts.liquidity_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN]>
for RedeemReserveCollateralAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            lending_market: &arr[1],
            reserve: &arr[2],
            lending_market_authority: &arr[3],
            reserve_liquidity_mint: &arr[4],
            reserve_collateral_mint: &arr[5],
            reserve_liquidity_supply: &arr[6],
            user_source_collateral: &arr[7],
            user_destination_liquidity: &arr[8],
            collateral_token_program: &arr[9],
            liquidity_token_program: &arr[10],
            instruction_sysvar_account: &arr[11],
        }
    }
}
pub const REDEEM_RESERVE_COLLATERAL_IX_DISCM: [u8; 8] = [
    234,
    117,
    181,
    125,
    185,
    142,
    220,
    29,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RedeemReserveCollateralIxArgs {
    pub collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemReserveCollateralIxData(pub RedeemReserveCollateralIxArgs);
impl From<RedeemReserveCollateralIxArgs> for RedeemReserveCollateralIxData {
    fn from(args: RedeemReserveCollateralIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemReserveCollateralIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_RESERVE_COLLATERAL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REDEEM_RESERVE_COLLATERAL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RedeemReserveCollateralIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_RESERVE_COLLATERAL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_reserve_collateral_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemReserveCollateralKeys,
    args: RedeemReserveCollateralIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemReserveCollateralIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_reserve_collateral_ix(
    keys: RedeemReserveCollateralKeys,
    args: RedeemReserveCollateralIxArgs,
) -> std::io::Result<Instruction> {
    redeem_reserve_collateral_ix_with_program_id(crate::ID, keys, args)
}
pub fn redeem_reserve_collateral_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemReserveCollateralAccounts<'_, '_>,
    args: RedeemReserveCollateralIxArgs,
) -> ProgramResult {
    let keys: RedeemReserveCollateralKeys = accounts.into();
    let ix = redeem_reserve_collateral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_reserve_collateral_invoke(
    accounts: RedeemReserveCollateralAccounts<'_, '_>,
    args: RedeemReserveCollateralIxArgs,
) -> ProgramResult {
    redeem_reserve_collateral_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn redeem_reserve_collateral_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemReserveCollateralAccounts<'_, '_>,
    args: RedeemReserveCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemReserveCollateralKeys = accounts.into();
    let ix = redeem_reserve_collateral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_reserve_collateral_invoke_signed(
    accounts: RedeemReserveCollateralAccounts<'_, '_>,
    args: RedeemReserveCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_reserve_collateral_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn redeem_reserve_collateral_verify_account_keys(
    accounts: RedeemReserveCollateralAccounts<'_, '_>,
    keys: RedeemReserveCollateralKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.reserve_liquidity_mint.key, keys.reserve_liquidity_mint),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.user_source_collateral.key, keys.user_source_collateral),
        (*accounts.user_destination_liquidity.key, keys.user_destination_liquidity),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (*accounts.liquidity_token_program.key, keys.liquidity_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_reserve_collateral_verify_writable_privileges<'me, 'info>(
    accounts: RedeemReserveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.reserve,
        accounts.reserve_collateral_mint,
        accounts.reserve_liquidity_supply,
        accounts.user_source_collateral,
        accounts.user_destination_liquidity,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_reserve_collateral_verify_signer_privileges<'me, 'info>(
    accounts: RedeemReserveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_reserve_collateral_verify_account_privileges<'me, 'info>(
    accounts: RedeemReserveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_reserve_collateral_verify_writable_privileges(accounts)?;
    redeem_reserve_collateral_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_OBLIGATION_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitObligationAccounts<'me, 'info> {
    pub obligation_owner: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub seed1_account: &'me AccountInfo<'info>,
    pub seed2_account: &'me AccountInfo<'info>,
    pub owner_user_metadata: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitObligationKeys {
    pub obligation_owner: Pubkey,
    pub fee_payer: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub seed1_account: Pubkey,
    pub seed2_account: Pubkey,
    pub owner_user_metadata: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitObligationAccounts<'_, '_>> for InitObligationKeys {
    fn from(accounts: InitObligationAccounts) -> Self {
        Self {
            obligation_owner: *accounts.obligation_owner.key,
            fee_payer: *accounts.fee_payer.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            seed1_account: *accounts.seed1_account.key,
            seed2_account: *accounts.seed2_account.key,
            owner_user_metadata: *accounts.owner_user_metadata.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitObligationKeys> for [AccountMeta; INIT_OBLIGATION_IX_ACCOUNTS_LEN] {
    fn from(keys: InitObligationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.obligation_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seed1_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.seed2_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner_user_metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_OBLIGATION_IX_ACCOUNTS_LEN]> for InitObligationKeys {
    fn from(pubkeys: [Pubkey; INIT_OBLIGATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            obligation_owner: pubkeys[0],
            fee_payer: pubkeys[1],
            obligation: pubkeys[2],
            lending_market: pubkeys[3],
            seed1_account: pubkeys[4],
            seed2_account: pubkeys[5],
            owner_user_metadata: pubkeys[6],
            rent: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<InitObligationAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_OBLIGATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitObligationAccounts<'_, 'info>) -> Self {
        [
            accounts.obligation_owner.clone(),
            accounts.fee_payer.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.seed1_account.clone(),
            accounts.seed2_account.clone(),
            accounts.owner_user_metadata.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_OBLIGATION_IX_ACCOUNTS_LEN]>
for InitObligationAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_OBLIGATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            obligation_owner: &arr[0],
            fee_payer: &arr[1],
            obligation: &arr[2],
            lending_market: &arr[3],
            seed1_account: &arr[4],
            seed2_account: &arr[5],
            owner_user_metadata: &arr[6],
            rent: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const INIT_OBLIGATION_IX_DISCM: [u8; 8] = [251, 10, 231, 76, 27, 11, 159, 96];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitObligationIxArgs {
    pub args: InitObligationArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitObligationIxData(pub InitObligationIxArgs);
impl From<InitObligationIxArgs> for InitObligationIxData {
    fn from(args: InitObligationIxArgs) -> Self {
        Self(args)
    }
}
impl InitObligationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_OBLIGATION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INIT_OBLIGATION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitObligationIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_OBLIGATION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_obligation_ix_with_program_id(
    program_id: Pubkey,
    keys: InitObligationKeys,
    args: InitObligationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_OBLIGATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitObligationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_obligation_ix(
    keys: InitObligationKeys,
    args: InitObligationIxArgs,
) -> std::io::Result<Instruction> {
    init_obligation_ix_with_program_id(crate::ID, keys, args)
}
pub fn init_obligation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitObligationAccounts<'_, '_>,
    args: InitObligationIxArgs,
) -> ProgramResult {
    let keys: InitObligationKeys = accounts.into();
    let ix = init_obligation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_obligation_invoke(
    accounts: InitObligationAccounts<'_, '_>,
    args: InitObligationIxArgs,
) -> ProgramResult {
    init_obligation_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn init_obligation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitObligationAccounts<'_, '_>,
    args: InitObligationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitObligationKeys = accounts.into();
    let ix = init_obligation_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_obligation_invoke_signed(
    accounts: InitObligationAccounts<'_, '_>,
    args: InitObligationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_obligation_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn init_obligation_verify_account_keys(
    accounts: InitObligationAccounts<'_, '_>,
    keys: InitObligationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.obligation_owner.key, keys.obligation_owner),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.seed1_account.key, keys.seed1_account),
        (*accounts.seed2_account.key, keys.seed2_account),
        (*accounts.owner_user_metadata.key, keys.owner_user_metadata),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_obligation_verify_writable_privileges<'me, 'info>(
    accounts: InitObligationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_payer, accounts.obligation] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_obligation_verify_signer_privileges<'me, 'info>(
    accounts: InitObligationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.obligation_owner, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_obligation_verify_account_privileges<'me, 'info>(
    accounts: InitObligationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_obligation_verify_writable_privileges(accounts)?;
    init_obligation_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitObligationFarmsForReserveAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_farm_state: &'me AccountInfo<'info>,
    pub obligation_farm: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitObligationFarmsForReserveKeys {
    pub payer: Pubkey,
    pub owner: Pubkey,
    pub obligation: Pubkey,
    pub lending_market_authority: Pubkey,
    pub reserve: Pubkey,
    pub reserve_farm_state: Pubkey,
    pub obligation_farm: Pubkey,
    pub lending_market: Pubkey,
    pub farms_program: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitObligationFarmsForReserveAccounts<'_, '_>>
for InitObligationFarmsForReserveKeys {
    fn from(accounts: InitObligationFarmsForReserveAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            owner: *accounts.owner.key,
            obligation: *accounts.obligation.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            reserve: *accounts.reserve.key,
            reserve_farm_state: *accounts.reserve_farm_state.key,
            obligation_farm: *accounts.obligation_farm.key,
            lending_market: *accounts.lending_market.key,
            farms_program: *accounts.farms_program.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitObligationFarmsForReserveKeys>
for [AccountMeta; INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitObligationFarmsForReserveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.obligation_farm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN]>
for InitObligationFarmsForReserveKeys {
    fn from(
        pubkeys: [Pubkey; INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: pubkeys[0],
            owner: pubkeys[1],
            obligation: pubkeys[2],
            lending_market_authority: pubkeys[3],
            reserve: pubkeys[4],
            reserve_farm_state: pubkeys[5],
            obligation_farm: pubkeys[6],
            lending_market: pubkeys[7],
            farms_program: pubkeys[8],
            rent: pubkeys[9],
            system_program: pubkeys[10],
        }
    }
}
impl<'info> From<InitObligationFarmsForReserveAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitObligationFarmsForReserveAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.owner.clone(),
            accounts.obligation.clone(),
            accounts.lending_market_authority.clone(),
            accounts.reserve.clone(),
            accounts.reserve_farm_state.clone(),
            accounts.obligation_farm.clone(),
            accounts.lending_market.clone(),
            accounts.farms_program.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN]>
for InitObligationFarmsForReserveAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            owner: &arr[1],
            obligation: &arr[2],
            lending_market_authority: &arr[3],
            reserve: &arr[4],
            reserve_farm_state: &arr[5],
            obligation_farm: &arr[6],
            lending_market: &arr[7],
            farms_program: &arr[8],
            rent: &arr[9],
            system_program: &arr[10],
        }
    }
}
pub const INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM: [u8; 8] = [
    136,
    63,
    15,
    186,
    211,
    152,
    168,
    164,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitObligationFarmsForReserveIxArgs {
    pub mode: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitObligationFarmsForReserveIxData(pub InitObligationFarmsForReserveIxArgs);
impl From<InitObligationFarmsForReserveIxArgs> for InitObligationFarmsForReserveIxData {
    fn from(args: InitObligationFarmsForReserveIxArgs) -> Self {
        Self(args)
    }
}
impl InitObligationFarmsForReserveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitObligationFarmsForReserveIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_obligation_farms_for_reserve_ix_with_program_id(
    program_id: Pubkey,
    keys: InitObligationFarmsForReserveKeys,
    args: InitObligationFarmsForReserveIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitObligationFarmsForReserveIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_obligation_farms_for_reserve_ix(
    keys: InitObligationFarmsForReserveKeys,
    args: InitObligationFarmsForReserveIxArgs,
) -> std::io::Result<Instruction> {
    init_obligation_farms_for_reserve_ix_with_program_id(crate::ID, keys, args)
}
pub fn init_obligation_farms_for_reserve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitObligationFarmsForReserveAccounts<'_, '_>,
    args: InitObligationFarmsForReserveIxArgs,
) -> ProgramResult {
    let keys: InitObligationFarmsForReserveKeys = accounts.into();
    let ix = init_obligation_farms_for_reserve_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn init_obligation_farms_for_reserve_invoke(
    accounts: InitObligationFarmsForReserveAccounts<'_, '_>,
    args: InitObligationFarmsForReserveIxArgs,
) -> ProgramResult {
    init_obligation_farms_for_reserve_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn init_obligation_farms_for_reserve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitObligationFarmsForReserveAccounts<'_, '_>,
    args: InitObligationFarmsForReserveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitObligationFarmsForReserveKeys = accounts.into();
    let ix = init_obligation_farms_for_reserve_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_obligation_farms_for_reserve_invoke_signed(
    accounts: InitObligationFarmsForReserveAccounts<'_, '_>,
    args: InitObligationFarmsForReserveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_obligation_farms_for_reserve_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_obligation_farms_for_reserve_verify_account_keys(
    accounts: InitObligationFarmsForReserveAccounts<'_, '_>,
    keys: InitObligationFarmsForReserveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.owner.key, keys.owner),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_farm_state.key, keys.reserve_farm_state),
        (*accounts.obligation_farm.key, keys.obligation_farm),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.farms_program.key, keys.farms_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_obligation_farms_for_reserve_verify_writable_privileges<'me, 'info>(
    accounts: InitObligationFarmsForReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.obligation,
        accounts.reserve,
        accounts.reserve_farm_state,
        accounts.obligation_farm,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_obligation_farms_for_reserve_verify_signer_privileges<'me, 'info>(
    accounts: InitObligationFarmsForReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_obligation_farms_for_reserve_verify_account_privileges<'me, 'info>(
    accounts: InitObligationFarmsForReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_obligation_farms_for_reserve_verify_writable_privileges(accounts)?;
    init_obligation_farms_for_reserve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RefreshObligationFarmsForReserveAccounts<'me, 'info> {
    pub crank: &'me AccountInfo<'info>,
    pub base_accounts_obligation: &'me AccountInfo<'info>,
    pub base_accounts_lending_market_authority: &'me AccountInfo<'info>,
    pub base_accounts_reserve: &'me AccountInfo<'info>,
    pub base_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub base_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub base_accounts_lending_market: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshObligationFarmsForReserveKeys {
    pub crank: Pubkey,
    pub base_accounts_obligation: Pubkey,
    pub base_accounts_lending_market_authority: Pubkey,
    pub base_accounts_reserve: Pubkey,
    pub base_accounts_reserve_farm_state: Pubkey,
    pub base_accounts_obligation_farm_user_state: Pubkey,
    pub base_accounts_lending_market: Pubkey,
    pub farms_program: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<RefreshObligationFarmsForReserveAccounts<'_, '_>>
for RefreshObligationFarmsForReserveKeys {
    fn from(accounts: RefreshObligationFarmsForReserveAccounts) -> Self {
        Self {
            crank: *accounts.crank.key,
            base_accounts_obligation: *accounts.base_accounts_obligation.key,
            base_accounts_lending_market_authority: *accounts
                .base_accounts_lending_market_authority
                .key,
            base_accounts_reserve: *accounts.base_accounts_reserve.key,
            base_accounts_reserve_farm_state: *accounts
                .base_accounts_reserve_farm_state
                .key,
            base_accounts_obligation_farm_user_state: *accounts
                .base_accounts_obligation_farm_user_state
                .key,
            base_accounts_lending_market: *accounts.base_accounts_lending_market.key,
            farms_program: *accounts.farms_program.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RefreshObligationFarmsForReserveKeys>
for [AccountMeta; REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(keys: RefreshObligationFarmsForReserveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.crank,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_obligation,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_reserve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN]>
for RefreshObligationFarmsForReserveKeys {
    fn from(
        pubkeys: [Pubkey; REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            crank: pubkeys[0],
            base_accounts_obligation: pubkeys[1],
            base_accounts_lending_market_authority: pubkeys[2],
            base_accounts_reserve: pubkeys[3],
            base_accounts_reserve_farm_state: pubkeys[4],
            base_accounts_obligation_farm_user_state: pubkeys[5],
            base_accounts_lending_market: pubkeys[6],
            farms_program: pubkeys[7],
            rent: pubkeys[8],
            system_program: pubkeys[9],
        }
    }
}
impl<'info> From<RefreshObligationFarmsForReserveAccounts<'_, 'info>>
for [AccountInfo<'info>; REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefreshObligationFarmsForReserveAccounts<'_, 'info>) -> Self {
        [
            accounts.crank.clone(),
            accounts.base_accounts_obligation.clone(),
            accounts.base_accounts_lending_market_authority.clone(),
            accounts.base_accounts_reserve.clone(),
            accounts.base_accounts_reserve_farm_state.clone(),
            accounts.base_accounts_obligation_farm_user_state.clone(),
            accounts.base_accounts_lending_market.clone(),
            accounts.farms_program.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN]>
for RefreshObligationFarmsForReserveAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            crank: &arr[0],
            base_accounts_obligation: &arr[1],
            base_accounts_lending_market_authority: &arr[2],
            base_accounts_reserve: &arr[3],
            base_accounts_reserve_farm_state: &arr[4],
            base_accounts_obligation_farm_user_state: &arr[5],
            base_accounts_lending_market: &arr[6],
            farms_program: &arr[7],
            rent: &arr[8],
            system_program: &arr[9],
        }
    }
}
pub const REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM: [u8; 8] = [
    140,
    144,
    253,
    21,
    10,
    74,
    248,
    3,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefreshObligationFarmsForReserveIxArgs {
    pub mode: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RefreshObligationFarmsForReserveIxData(
    pub RefreshObligationFarmsForReserveIxArgs,
);
impl From<RefreshObligationFarmsForReserveIxArgs>
for RefreshObligationFarmsForReserveIxData {
    fn from(args: RefreshObligationFarmsForReserveIxArgs) -> Self {
        Self(args)
    }
}
impl RefreshObligationFarmsForReserveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RefreshObligationFarmsForReserveIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refresh_obligation_farms_for_reserve_ix_with_program_id(
    program_id: Pubkey,
    keys: RefreshObligationFarmsForReserveKeys,
    args: RefreshObligationFarmsForReserveIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFRESH_OBLIGATION_FARMS_FOR_RESERVE_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: RefreshObligationFarmsForReserveIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn refresh_obligation_farms_for_reserve_ix(
    keys: RefreshObligationFarmsForReserveKeys,
    args: RefreshObligationFarmsForReserveIxArgs,
) -> std::io::Result<Instruction> {
    refresh_obligation_farms_for_reserve_ix_with_program_id(crate::ID, keys, args)
}
pub fn refresh_obligation_farms_for_reserve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefreshObligationFarmsForReserveAccounts<'_, '_>,
    args: RefreshObligationFarmsForReserveIxArgs,
) -> ProgramResult {
    let keys: RefreshObligationFarmsForReserveKeys = accounts.into();
    let ix = refresh_obligation_farms_for_reserve_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn refresh_obligation_farms_for_reserve_invoke(
    accounts: RefreshObligationFarmsForReserveAccounts<'_, '_>,
    args: RefreshObligationFarmsForReserveIxArgs,
) -> ProgramResult {
    refresh_obligation_farms_for_reserve_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn refresh_obligation_farms_for_reserve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefreshObligationFarmsForReserveAccounts<'_, '_>,
    args: RefreshObligationFarmsForReserveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefreshObligationFarmsForReserveKeys = accounts.into();
    let ix = refresh_obligation_farms_for_reserve_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refresh_obligation_farms_for_reserve_invoke_signed(
    accounts: RefreshObligationFarmsForReserveAccounts<'_, '_>,
    args: RefreshObligationFarmsForReserveIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_obligation_farms_for_reserve_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn refresh_obligation_farms_for_reserve_verify_account_keys(
    accounts: RefreshObligationFarmsForReserveAccounts<'_, '_>,
    keys: RefreshObligationFarmsForReserveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.crank.key, keys.crank),
        (*accounts.base_accounts_obligation.key, keys.base_accounts_obligation),
        (
            *accounts.base_accounts_lending_market_authority.key,
            keys.base_accounts_lending_market_authority,
        ),
        (*accounts.base_accounts_reserve.key, keys.base_accounts_reserve),
        (
            *accounts.base_accounts_reserve_farm_state.key,
            keys.base_accounts_reserve_farm_state,
        ),
        (
            *accounts.base_accounts_obligation_farm_user_state.key,
            keys.base_accounts_obligation_farm_user_state,
        ),
        (*accounts.base_accounts_lending_market.key, keys.base_accounts_lending_market),
        (*accounts.farms_program.key, keys.farms_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refresh_obligation_farms_for_reserve_verify_writable_privileges<'me, 'info>(
    accounts: RefreshObligationFarmsForReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.base_accounts_reserve_farm_state,
        accounts.base_accounts_obligation_farm_user_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refresh_obligation_farms_for_reserve_verify_signer_privileges<'me, 'info>(
    accounts: RefreshObligationFarmsForReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.crank] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn refresh_obligation_farms_for_reserve_verify_account_privileges<'me, 'info>(
    accounts: RefreshObligationFarmsForReserveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refresh_obligation_farms_for_reserve_verify_writable_privileges(accounts)?;
    refresh_obligation_farms_for_reserve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REFRESH_OBLIGATION_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct RefreshObligationAccounts<'me, 'info> {
    pub lending_market: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefreshObligationKeys {
    pub lending_market: Pubkey,
    pub obligation: Pubkey,
}
impl From<RefreshObligationAccounts<'_, '_>> for RefreshObligationKeys {
    fn from(accounts: RefreshObligationAccounts) -> Self {
        Self {
            lending_market: *accounts.lending_market.key,
            obligation: *accounts.obligation.key,
        }
    }
}
impl From<RefreshObligationKeys> for [AccountMeta; REFRESH_OBLIGATION_IX_ACCOUNTS_LEN] {
    fn from(keys: RefreshObligationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REFRESH_OBLIGATION_IX_ACCOUNTS_LEN]> for RefreshObligationKeys {
    fn from(pubkeys: [Pubkey; REFRESH_OBLIGATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market: pubkeys[0],
            obligation: pubkeys[1],
        }
    }
}
impl<'info> From<RefreshObligationAccounts<'_, 'info>>
for [AccountInfo<'info>; REFRESH_OBLIGATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefreshObligationAccounts<'_, 'info>) -> Self {
        [accounts.lending_market.clone(), accounts.obligation.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REFRESH_OBLIGATION_IX_ACCOUNTS_LEN]>
for RefreshObligationAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REFRESH_OBLIGATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market: &arr[0],
            obligation: &arr[1],
        }
    }
}
pub const REFRESH_OBLIGATION_IX_DISCM: [u8; 8] = [33, 132, 147, 228, 151, 192, 72, 89];
#[derive(Clone, Debug, PartialEq)]
pub struct RefreshObligationIxData;
impl RefreshObligationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFRESH_OBLIGATION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REFRESH_OBLIGATION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFRESH_OBLIGATION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refresh_obligation_ix_with_program_id(
    program_id: Pubkey,
    keys: RefreshObligationKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFRESH_OBLIGATION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RefreshObligationIxData.try_to_vec()?,
    })
}
pub fn refresh_obligation_ix(
    keys: RefreshObligationKeys,
) -> std::io::Result<Instruction> {
    refresh_obligation_ix_with_program_id(crate::ID, keys)
}
pub fn refresh_obligation_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefreshObligationAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RefreshObligationKeys = accounts.into();
    let ix = refresh_obligation_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn refresh_obligation_invoke(
    accounts: RefreshObligationAccounts<'_, '_>,
) -> ProgramResult {
    refresh_obligation_invoke_with_program_id(crate::ID, accounts)
}
pub fn refresh_obligation_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefreshObligationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefreshObligationKeys = accounts.into();
    let ix = refresh_obligation_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refresh_obligation_invoke_signed(
    accounts: RefreshObligationAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refresh_obligation_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn refresh_obligation_verify_account_keys(
    accounts: RefreshObligationAccounts<'_, '_>,
    keys: RefreshObligationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.obligation.key, keys.obligation),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refresh_obligation_verify_writable_privileges<'me, 'info>(
    accounts: RefreshObligationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.obligation] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn refresh_obligation_verify_account_privileges<'me, 'info>(
    accounts: RefreshObligationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refresh_obligation_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct DepositObligationCollateralAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub deposit_reserve: &'me AccountInfo<'info>,
    pub reserve_destination_collateral: &'me AccountInfo<'info>,
    pub user_source_collateral: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositObligationCollateralKeys {
    pub owner: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub deposit_reserve: Pubkey,
    pub reserve_destination_collateral: Pubkey,
    pub user_source_collateral: Pubkey,
    pub token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<DepositObligationCollateralAccounts<'_, '_>>
for DepositObligationCollateralKeys {
    fn from(accounts: DepositObligationCollateralAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            deposit_reserve: *accounts.deposit_reserve.key,
            reserve_destination_collateral: *accounts.reserve_destination_collateral.key,
            user_source_collateral: *accounts.user_source_collateral.key,
            token_program: *accounts.token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<DepositObligationCollateralKeys>
for [AccountMeta; DEPOSIT_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositObligationCollateralKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_destination_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN]>
for DepositObligationCollateralKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            obligation: pubkeys[1],
            lending_market: pubkeys[2],
            deposit_reserve: pubkeys[3],
            reserve_destination_collateral: pubkeys[4],
            user_source_collateral: pubkeys[5],
            token_program: pubkeys[6],
            instruction_sysvar_account: pubkeys[7],
        }
    }
}
impl<'info> From<DepositObligationCollateralAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositObligationCollateralAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.deposit_reserve.clone(),
            accounts.reserve_destination_collateral.clone(),
            accounts.user_source_collateral.clone(),
            accounts.token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DEPOSIT_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN]>
for DepositObligationCollateralAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            obligation: &arr[1],
            lending_market: &arr[2],
            deposit_reserve: &arr[3],
            reserve_destination_collateral: &arr[4],
            user_source_collateral: &arr[5],
            token_program: &arr[6],
            instruction_sysvar_account: &arr[7],
        }
    }
}
pub const DEPOSIT_OBLIGATION_COLLATERAL_IX_DISCM: [u8; 8] = [
    108,
    209,
    4,
    72,
    21,
    22,
    118,
    133,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositObligationCollateralIxArgs {
    pub collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositObligationCollateralIxData(pub DepositObligationCollateralIxArgs);
impl From<DepositObligationCollateralIxArgs> for DepositObligationCollateralIxData {
    fn from(args: DepositObligationCollateralIxArgs) -> Self {
        Self(args)
    }
}
impl DepositObligationCollateralIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_OBLIGATION_COLLATERAL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DEPOSIT_OBLIGATION_COLLATERAL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(DepositObligationCollateralIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_OBLIGATION_COLLATERAL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_obligation_collateral_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositObligationCollateralKeys,
    args: DepositObligationCollateralIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DepositObligationCollateralIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_obligation_collateral_ix(
    keys: DepositObligationCollateralKeys,
    args: DepositObligationCollateralIxArgs,
) -> std::io::Result<Instruction> {
    deposit_obligation_collateral_ix_with_program_id(crate::ID, keys, args)
}
pub fn deposit_obligation_collateral_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositObligationCollateralAccounts<'_, '_>,
    args: DepositObligationCollateralIxArgs,
) -> ProgramResult {
    let keys: DepositObligationCollateralKeys = accounts.into();
    let ix = deposit_obligation_collateral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_obligation_collateral_invoke(
    accounts: DepositObligationCollateralAccounts<'_, '_>,
    args: DepositObligationCollateralIxArgs,
) -> ProgramResult {
    deposit_obligation_collateral_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn deposit_obligation_collateral_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositObligationCollateralAccounts<'_, '_>,
    args: DepositObligationCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositObligationCollateralKeys = accounts.into();
    let ix = deposit_obligation_collateral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_obligation_collateral_invoke_signed(
    accounts: DepositObligationCollateralAccounts<'_, '_>,
    args: DepositObligationCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_obligation_collateral_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_obligation_collateral_verify_account_keys(
    accounts: DepositObligationCollateralAccounts<'_, '_>,
    keys: DepositObligationCollateralKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.deposit_reserve.key, keys.deposit_reserve),
        (
            *accounts.reserve_destination_collateral.key,
            keys.reserve_destination_collateral,
        ),
        (*accounts.user_source_collateral.key, keys.user_source_collateral),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_obligation_collateral_verify_writable_privileges<'me, 'info>(
    accounts: DepositObligationCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.obligation,
        accounts.deposit_reserve,
        accounts.reserve_destination_collateral,
        accounts.user_source_collateral,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_obligation_collateral_verify_signer_privileges<'me, 'info>(
    accounts: DepositObligationCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_obligation_collateral_verify_account_privileges<'me, 'info>(
    accounts: DepositObligationCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_obligation_collateral_verify_writable_privileges(accounts)?;
    deposit_obligation_collateral_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct DepositObligationCollateralV2Accounts<'me, 'info> {
    pub deposit_accounts_owner: &'me AccountInfo<'info>,
    pub deposit_accounts_obligation: &'me AccountInfo<'info>,
    pub deposit_accounts_lending_market: &'me AccountInfo<'info>,
    pub deposit_accounts_deposit_reserve: &'me AccountInfo<'info>,
    pub deposit_accounts_reserve_destination_collateral: &'me AccountInfo<'info>,
    pub deposit_accounts_user_source_collateral: &'me AccountInfo<'info>,
    pub deposit_accounts_token_program: &'me AccountInfo<'info>,
    pub deposit_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositObligationCollateralV2Keys {
    pub deposit_accounts_owner: Pubkey,
    pub deposit_accounts_obligation: Pubkey,
    pub deposit_accounts_lending_market: Pubkey,
    pub deposit_accounts_deposit_reserve: Pubkey,
    pub deposit_accounts_reserve_destination_collateral: Pubkey,
    pub deposit_accounts_user_source_collateral: Pubkey,
    pub deposit_accounts_token_program: Pubkey,
    pub deposit_accounts_instruction_sysvar_account: Pubkey,
    pub lending_market_authority: Pubkey,
    pub farms_accounts_obligation_farm_user_state: Pubkey,
    pub farms_accounts_reserve_farm_state: Pubkey,
    pub farms_program: Pubkey,
}
impl From<DepositObligationCollateralV2Accounts<'_, '_>>
for DepositObligationCollateralV2Keys {
    fn from(accounts: DepositObligationCollateralV2Accounts) -> Self {
        Self {
            deposit_accounts_owner: *accounts.deposit_accounts_owner.key,
            deposit_accounts_obligation: *accounts.deposit_accounts_obligation.key,
            deposit_accounts_lending_market: *accounts
                .deposit_accounts_lending_market
                .key,
            deposit_accounts_deposit_reserve: *accounts
                .deposit_accounts_deposit_reserve
                .key,
            deposit_accounts_reserve_destination_collateral: *accounts
                .deposit_accounts_reserve_destination_collateral
                .key,
            deposit_accounts_user_source_collateral: *accounts
                .deposit_accounts_user_source_collateral
                .key,
            deposit_accounts_token_program: *accounts.deposit_accounts_token_program.key,
            deposit_accounts_instruction_sysvar_account: *accounts
                .deposit_accounts_instruction_sysvar_account
                .key,
            lending_market_authority: *accounts.lending_market_authority.key,
            farms_accounts_obligation_farm_user_state: *accounts
                .farms_accounts_obligation_farm_user_state
                .key,
            farms_accounts_reserve_farm_state: *accounts
                .farms_accounts_reserve_farm_state
                .key,
            farms_program: *accounts.farms_program.key,
        }
    }
}
impl From<DepositObligationCollateralV2Keys>
for [AccountMeta; DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositObligationCollateralV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.deposit_accounts_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_deposit_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_reserve_destination_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_user_source_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN]>
for DepositObligationCollateralV2Keys {
    fn from(
        pubkeys: [Pubkey; DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            deposit_accounts_owner: pubkeys[0],
            deposit_accounts_obligation: pubkeys[1],
            deposit_accounts_lending_market: pubkeys[2],
            deposit_accounts_deposit_reserve: pubkeys[3],
            deposit_accounts_reserve_destination_collateral: pubkeys[4],
            deposit_accounts_user_source_collateral: pubkeys[5],
            deposit_accounts_token_program: pubkeys[6],
            deposit_accounts_instruction_sysvar_account: pubkeys[7],
            lending_market_authority: pubkeys[8],
            farms_accounts_obligation_farm_user_state: pubkeys[9],
            farms_accounts_reserve_farm_state: pubkeys[10],
            farms_program: pubkeys[11],
        }
    }
}
impl<'info> From<DepositObligationCollateralV2Accounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositObligationCollateralV2Accounts<'_, 'info>) -> Self {
        [
            accounts.deposit_accounts_owner.clone(),
            accounts.deposit_accounts_obligation.clone(),
            accounts.deposit_accounts_lending_market.clone(),
            accounts.deposit_accounts_deposit_reserve.clone(),
            accounts.deposit_accounts_reserve_destination_collateral.clone(),
            accounts.deposit_accounts_user_source_collateral.clone(),
            accounts.deposit_accounts_token_program.clone(),
            accounts.deposit_accounts_instruction_sysvar_account.clone(),
            accounts.lending_market_authority.clone(),
            accounts.farms_accounts_obligation_farm_user_state.clone(),
            accounts.farms_accounts_reserve_farm_state.clone(),
            accounts.farms_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN]>
for DepositObligationCollateralV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            deposit_accounts_owner: &arr[0],
            deposit_accounts_obligation: &arr[1],
            deposit_accounts_lending_market: &arr[2],
            deposit_accounts_deposit_reserve: &arr[3],
            deposit_accounts_reserve_destination_collateral: &arr[4],
            deposit_accounts_user_source_collateral: &arr[5],
            deposit_accounts_token_program: &arr[6],
            deposit_accounts_instruction_sysvar_account: &arr[7],
            lending_market_authority: &arr[8],
            farms_accounts_obligation_farm_user_state: &arr[9],
            farms_accounts_reserve_farm_state: &arr[10],
            farms_program: &arr[11],
        }
    }
}
pub const DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_DISCM: [u8; 8] = [
    137,
    145,
    151,
    94,
    167,
    113,
    4,
    145,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositObligationCollateralV2IxArgs {
    pub collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositObligationCollateralV2IxData(pub DepositObligationCollateralV2IxArgs);
impl From<DepositObligationCollateralV2IxArgs> for DepositObligationCollateralV2IxData {
    fn from(args: DepositObligationCollateralV2IxArgs) -> Self {
        Self(args)
    }
}
impl DepositObligationCollateralV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(DepositObligationCollateralV2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_obligation_collateral_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositObligationCollateralV2Keys,
    args: DepositObligationCollateralV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DepositObligationCollateralV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_obligation_collateral_v2_ix(
    keys: DepositObligationCollateralV2Keys,
    args: DepositObligationCollateralV2IxArgs,
) -> std::io::Result<Instruction> {
    deposit_obligation_collateral_v2_ix_with_program_id(crate::ID, keys, args)
}
pub fn deposit_obligation_collateral_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositObligationCollateralV2Accounts<'_, '_>,
    args: DepositObligationCollateralV2IxArgs,
) -> ProgramResult {
    let keys: DepositObligationCollateralV2Keys = accounts.into();
    let ix = deposit_obligation_collateral_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_obligation_collateral_v2_invoke(
    accounts: DepositObligationCollateralV2Accounts<'_, '_>,
    args: DepositObligationCollateralV2IxArgs,
) -> ProgramResult {
    deposit_obligation_collateral_v2_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn deposit_obligation_collateral_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositObligationCollateralV2Accounts<'_, '_>,
    args: DepositObligationCollateralV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositObligationCollateralV2Keys = accounts.into();
    let ix = deposit_obligation_collateral_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_obligation_collateral_v2_invoke_signed(
    accounts: DepositObligationCollateralV2Accounts<'_, '_>,
    args: DepositObligationCollateralV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_obligation_collateral_v2_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_obligation_collateral_v2_verify_account_keys(
    accounts: DepositObligationCollateralV2Accounts<'_, '_>,
    keys: DepositObligationCollateralV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.deposit_accounts_owner.key, keys.deposit_accounts_owner),
        (*accounts.deposit_accounts_obligation.key, keys.deposit_accounts_obligation),
        (
            *accounts.deposit_accounts_lending_market.key,
            keys.deposit_accounts_lending_market,
        ),
        (
            *accounts.deposit_accounts_deposit_reserve.key,
            keys.deposit_accounts_deposit_reserve,
        ),
        (
            *accounts.deposit_accounts_reserve_destination_collateral.key,
            keys.deposit_accounts_reserve_destination_collateral,
        ),
        (
            *accounts.deposit_accounts_user_source_collateral.key,
            keys.deposit_accounts_user_source_collateral,
        ),
        (
            *accounts.deposit_accounts_token_program.key,
            keys.deposit_accounts_token_program,
        ),
        (
            *accounts.deposit_accounts_instruction_sysvar_account.key,
            keys.deposit_accounts_instruction_sysvar_account,
        ),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (
            *accounts.farms_accounts_obligation_farm_user_state.key,
            keys.farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.farms_accounts_reserve_farm_state.key,
            keys.farms_accounts_reserve_farm_state,
        ),
        (*accounts.farms_program.key, keys.farms_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_obligation_collateral_v2_verify_writable_privileges<'me, 'info>(
    accounts: DepositObligationCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deposit_accounts_obligation,
        accounts.deposit_accounts_deposit_reserve,
        accounts.deposit_accounts_reserve_destination_collateral,
        accounts.deposit_accounts_user_source_collateral,
        accounts.farms_accounts_obligation_farm_user_state,
        accounts.farms_accounts_reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_obligation_collateral_v2_verify_signer_privileges<'me, 'info>(
    accounts: DepositObligationCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.deposit_accounts_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_obligation_collateral_v2_verify_account_privileges<'me, 'info>(
    accounts: DepositObligationCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_obligation_collateral_v2_verify_writable_privileges(accounts)?;
    deposit_obligation_collateral_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawObligationCollateralAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub withdraw_reserve: &'me AccountInfo<'info>,
    pub reserve_source_collateral: &'me AccountInfo<'info>,
    pub user_destination_collateral: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawObligationCollateralKeys {
    pub owner: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub withdraw_reserve: Pubkey,
    pub reserve_source_collateral: Pubkey,
    pub user_destination_collateral: Pubkey,
    pub token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<WithdrawObligationCollateralAccounts<'_, '_>>
for WithdrawObligationCollateralKeys {
    fn from(accounts: WithdrawObligationCollateralAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            withdraw_reserve: *accounts.withdraw_reserve.key,
            reserve_source_collateral: *accounts.reserve_source_collateral.key,
            user_destination_collateral: *accounts.user_destination_collateral.key,
            token_program: *accounts.token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<WithdrawObligationCollateralKeys>
for [AccountMeta; WITHDRAW_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawObligationCollateralKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_source_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN]>
for WithdrawObligationCollateralKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            obligation: pubkeys[1],
            lending_market: pubkeys[2],
            lending_market_authority: pubkeys[3],
            withdraw_reserve: pubkeys[4],
            reserve_source_collateral: pubkeys[5],
            user_destination_collateral: pubkeys[6],
            token_program: pubkeys[7],
            instruction_sysvar_account: pubkeys[8],
        }
    }
}
impl<'info> From<WithdrawObligationCollateralAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawObligationCollateralAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.withdraw_reserve.clone(),
            accounts.reserve_source_collateral.clone(),
            accounts.user_destination_collateral.clone(),
            accounts.token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN]>
for WithdrawObligationCollateralAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            obligation: &arr[1],
            lending_market: &arr[2],
            lending_market_authority: &arr[3],
            withdraw_reserve: &arr[4],
            reserve_source_collateral: &arr[5],
            user_destination_collateral: &arr[6],
            token_program: &arr[7],
            instruction_sysvar_account: &arr[8],
        }
    }
}
pub const WITHDRAW_OBLIGATION_COLLATERAL_IX_DISCM: [u8; 8] = [
    37,
    116,
    205,
    103,
    243,
    192,
    92,
    198,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawObligationCollateralIxArgs {
    pub collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawObligationCollateralIxData(pub WithdrawObligationCollateralIxArgs);
impl From<WithdrawObligationCollateralIxArgs> for WithdrawObligationCollateralIxData {
    fn from(args: WithdrawObligationCollateralIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawObligationCollateralIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_OBLIGATION_COLLATERAL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_OBLIGATION_COLLATERAL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WithdrawObligationCollateralIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_OBLIGATION_COLLATERAL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_obligation_collateral_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawObligationCollateralKeys,
    args: WithdrawObligationCollateralIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawObligationCollateralIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_obligation_collateral_ix(
    keys: WithdrawObligationCollateralKeys,
    args: WithdrawObligationCollateralIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_obligation_collateral_ix_with_program_id(crate::ID, keys, args)
}
pub fn withdraw_obligation_collateral_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawObligationCollateralAccounts<'_, '_>,
    args: WithdrawObligationCollateralIxArgs,
) -> ProgramResult {
    let keys: WithdrawObligationCollateralKeys = accounts.into();
    let ix = withdraw_obligation_collateral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_obligation_collateral_invoke(
    accounts: WithdrawObligationCollateralAccounts<'_, '_>,
    args: WithdrawObligationCollateralIxArgs,
) -> ProgramResult {
    withdraw_obligation_collateral_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn withdraw_obligation_collateral_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawObligationCollateralAccounts<'_, '_>,
    args: WithdrawObligationCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawObligationCollateralKeys = accounts.into();
    let ix = withdraw_obligation_collateral_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_obligation_collateral_invoke_signed(
    accounts: WithdrawObligationCollateralAccounts<'_, '_>,
    args: WithdrawObligationCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_obligation_collateral_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_obligation_collateral_verify_account_keys(
    accounts: WithdrawObligationCollateralAccounts<'_, '_>,
    keys: WithdrawObligationCollateralKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.withdraw_reserve.key, keys.withdraw_reserve),
        (*accounts.reserve_source_collateral.key, keys.reserve_source_collateral),
        (*accounts.user_destination_collateral.key, keys.user_destination_collateral),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawObligationCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.obligation,
        accounts.withdraw_reserve,
        accounts.reserve_source_collateral,
        accounts.user_destination_collateral,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawObligationCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_verify_account_privileges<'me, 'info>(
    accounts: WithdrawObligationCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_obligation_collateral_verify_writable_privileges(accounts)?;
    withdraw_obligation_collateral_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawObligationCollateralV2Accounts<'me, 'info> {
    pub withdraw_accounts_owner: &'me AccountInfo<'info>,
    pub withdraw_accounts_obligation: &'me AccountInfo<'info>,
    pub withdraw_accounts_lending_market: &'me AccountInfo<'info>,
    pub withdraw_accounts_lending_market_authority: &'me AccountInfo<'info>,
    pub withdraw_accounts_withdraw_reserve: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_source_collateral: &'me AccountInfo<'info>,
    pub withdraw_accounts_user_destination_collateral: &'me AccountInfo<'info>,
    pub withdraw_accounts_token_program: &'me AccountInfo<'info>,
    pub withdraw_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawObligationCollateralV2Keys {
    pub withdraw_accounts_owner: Pubkey,
    pub withdraw_accounts_obligation: Pubkey,
    pub withdraw_accounts_lending_market: Pubkey,
    pub withdraw_accounts_lending_market_authority: Pubkey,
    pub withdraw_accounts_withdraw_reserve: Pubkey,
    pub withdraw_accounts_reserve_source_collateral: Pubkey,
    pub withdraw_accounts_user_destination_collateral: Pubkey,
    pub withdraw_accounts_token_program: Pubkey,
    pub withdraw_accounts_instruction_sysvar_account: Pubkey,
    pub farms_accounts_obligation_farm_user_state: Pubkey,
    pub farms_accounts_reserve_farm_state: Pubkey,
    pub farms_program: Pubkey,
}
impl From<WithdrawObligationCollateralV2Accounts<'_, '_>>
for WithdrawObligationCollateralV2Keys {
    fn from(accounts: WithdrawObligationCollateralV2Accounts) -> Self {
        Self {
            withdraw_accounts_owner: *accounts.withdraw_accounts_owner.key,
            withdraw_accounts_obligation: *accounts.withdraw_accounts_obligation.key,
            withdraw_accounts_lending_market: *accounts
                .withdraw_accounts_lending_market
                .key,
            withdraw_accounts_lending_market_authority: *accounts
                .withdraw_accounts_lending_market_authority
                .key,
            withdraw_accounts_withdraw_reserve: *accounts
                .withdraw_accounts_withdraw_reserve
                .key,
            withdraw_accounts_reserve_source_collateral: *accounts
                .withdraw_accounts_reserve_source_collateral
                .key,
            withdraw_accounts_user_destination_collateral: *accounts
                .withdraw_accounts_user_destination_collateral
                .key,
            withdraw_accounts_token_program: *accounts
                .withdraw_accounts_token_program
                .key,
            withdraw_accounts_instruction_sysvar_account: *accounts
                .withdraw_accounts_instruction_sysvar_account
                .key,
            farms_accounts_obligation_farm_user_state: *accounts
                .farms_accounts_obligation_farm_user_state
                .key,
            farms_accounts_reserve_farm_state: *accounts
                .farms_accounts_reserve_farm_state
                .key,
            farms_program: *accounts.farms_program.key,
        }
    }
}
impl From<WithdrawObligationCollateralV2Keys>
for [AccountMeta; WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawObligationCollateralV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.withdraw_accounts_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_withdraw_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_source_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_user_destination_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN]>
for WithdrawObligationCollateralV2Keys {
    fn from(
        pubkeys: [Pubkey; WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            withdraw_accounts_owner: pubkeys[0],
            withdraw_accounts_obligation: pubkeys[1],
            withdraw_accounts_lending_market: pubkeys[2],
            withdraw_accounts_lending_market_authority: pubkeys[3],
            withdraw_accounts_withdraw_reserve: pubkeys[4],
            withdraw_accounts_reserve_source_collateral: pubkeys[5],
            withdraw_accounts_user_destination_collateral: pubkeys[6],
            withdraw_accounts_token_program: pubkeys[7],
            withdraw_accounts_instruction_sysvar_account: pubkeys[8],
            farms_accounts_obligation_farm_user_state: pubkeys[9],
            farms_accounts_reserve_farm_state: pubkeys[10],
            farms_program: pubkeys[11],
        }
    }
}
impl<'info> From<WithdrawObligationCollateralV2Accounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawObligationCollateralV2Accounts<'_, 'info>) -> Self {
        [
            accounts.withdraw_accounts_owner.clone(),
            accounts.withdraw_accounts_obligation.clone(),
            accounts.withdraw_accounts_lending_market.clone(),
            accounts.withdraw_accounts_lending_market_authority.clone(),
            accounts.withdraw_accounts_withdraw_reserve.clone(),
            accounts.withdraw_accounts_reserve_source_collateral.clone(),
            accounts.withdraw_accounts_user_destination_collateral.clone(),
            accounts.withdraw_accounts_token_program.clone(),
            accounts.withdraw_accounts_instruction_sysvar_account.clone(),
            accounts.farms_accounts_obligation_farm_user_state.clone(),
            accounts.farms_accounts_reserve_farm_state.clone(),
            accounts.farms_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN]>
for WithdrawObligationCollateralV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            withdraw_accounts_owner: &arr[0],
            withdraw_accounts_obligation: &arr[1],
            withdraw_accounts_lending_market: &arr[2],
            withdraw_accounts_lending_market_authority: &arr[3],
            withdraw_accounts_withdraw_reserve: &arr[4],
            withdraw_accounts_reserve_source_collateral: &arr[5],
            withdraw_accounts_user_destination_collateral: &arr[6],
            withdraw_accounts_token_program: &arr[7],
            withdraw_accounts_instruction_sysvar_account: &arr[8],
            farms_accounts_obligation_farm_user_state: &arr[9],
            farms_accounts_reserve_farm_state: &arr[10],
            farms_program: &arr[11],
        }
    }
}
pub const WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_DISCM: [u8; 8] = [
    202,
    249,
    117,
    114,
    231,
    192,
    47,
    138,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawObligationCollateralV2IxArgs {
    pub collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawObligationCollateralV2IxData(
    pub WithdrawObligationCollateralV2IxArgs,
);
impl From<WithdrawObligationCollateralV2IxArgs>
for WithdrawObligationCollateralV2IxData {
    fn from(args: WithdrawObligationCollateralV2IxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawObligationCollateralV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WithdrawObligationCollateralV2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_obligation_collateral_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawObligationCollateralV2Keys,
    args: WithdrawObligationCollateralV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawObligationCollateralV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_obligation_collateral_v2_ix(
    keys: WithdrawObligationCollateralV2Keys,
    args: WithdrawObligationCollateralV2IxArgs,
) -> std::io::Result<Instruction> {
    withdraw_obligation_collateral_v2_ix_with_program_id(crate::ID, keys, args)
}
pub fn withdraw_obligation_collateral_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawObligationCollateralV2Accounts<'_, '_>,
    args: WithdrawObligationCollateralV2IxArgs,
) -> ProgramResult {
    let keys: WithdrawObligationCollateralV2Keys = accounts.into();
    let ix = withdraw_obligation_collateral_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_obligation_collateral_v2_invoke(
    accounts: WithdrawObligationCollateralV2Accounts<'_, '_>,
    args: WithdrawObligationCollateralV2IxArgs,
) -> ProgramResult {
    withdraw_obligation_collateral_v2_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn withdraw_obligation_collateral_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawObligationCollateralV2Accounts<'_, '_>,
    args: WithdrawObligationCollateralV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawObligationCollateralV2Keys = accounts.into();
    let ix = withdraw_obligation_collateral_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_obligation_collateral_v2_invoke_signed(
    accounts: WithdrawObligationCollateralV2Accounts<'_, '_>,
    args: WithdrawObligationCollateralV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_obligation_collateral_v2_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_obligation_collateral_v2_verify_account_keys(
    accounts: WithdrawObligationCollateralV2Accounts<'_, '_>,
    keys: WithdrawObligationCollateralV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.withdraw_accounts_owner.key, keys.withdraw_accounts_owner),
        (*accounts.withdraw_accounts_obligation.key, keys.withdraw_accounts_obligation),
        (
            *accounts.withdraw_accounts_lending_market.key,
            keys.withdraw_accounts_lending_market,
        ),
        (
            *accounts.withdraw_accounts_lending_market_authority.key,
            keys.withdraw_accounts_lending_market_authority,
        ),
        (
            *accounts.withdraw_accounts_withdraw_reserve.key,
            keys.withdraw_accounts_withdraw_reserve,
        ),
        (
            *accounts.withdraw_accounts_reserve_source_collateral.key,
            keys.withdraw_accounts_reserve_source_collateral,
        ),
        (
            *accounts.withdraw_accounts_user_destination_collateral.key,
            keys.withdraw_accounts_user_destination_collateral,
        ),
        (
            *accounts.withdraw_accounts_token_program.key,
            keys.withdraw_accounts_token_program,
        ),
        (
            *accounts.withdraw_accounts_instruction_sysvar_account.key,
            keys.withdraw_accounts_instruction_sysvar_account,
        ),
        (
            *accounts.farms_accounts_obligation_farm_user_state.key,
            keys.farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.farms_accounts_reserve_farm_state.key,
            keys.farms_accounts_reserve_farm_state,
        ),
        (*accounts.farms_program.key, keys.farms_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_v2_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawObligationCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.withdraw_accounts_obligation,
        accounts.withdraw_accounts_withdraw_reserve,
        accounts.withdraw_accounts_reserve_source_collateral,
        accounts.withdraw_accounts_user_destination_collateral,
        accounts.farms_accounts_obligation_farm_user_state,
        accounts.farms_accounts_reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_v2_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawObligationCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.withdraw_accounts_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_v2_verify_account_privileges<'me, 'info>(
    accounts: WithdrawObligationCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_obligation_collateral_v2_verify_writable_privileges(accounts)?;
    withdraw_obligation_collateral_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BORROW_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct BorrowObligationLiquidityAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub borrow_reserve: &'me AccountInfo<'info>,
    pub borrow_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub reserve_source_liquidity: &'me AccountInfo<'info>,
    pub borrow_reserve_liquidity_fee_receiver: &'me AccountInfo<'info>,
    pub user_destination_liquidity: &'me AccountInfo<'info>,
    pub referrer_token_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorrowObligationLiquidityKeys {
    pub owner: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub borrow_reserve: Pubkey,
    pub borrow_reserve_liquidity_mint: Pubkey,
    pub reserve_source_liquidity: Pubkey,
    pub borrow_reserve_liquidity_fee_receiver: Pubkey,
    pub user_destination_liquidity: Pubkey,
    pub referrer_token_state: Pubkey,
    pub token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<BorrowObligationLiquidityAccounts<'_, '_>> for BorrowObligationLiquidityKeys {
    fn from(accounts: BorrowObligationLiquidityAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            borrow_reserve: *accounts.borrow_reserve.key,
            borrow_reserve_liquidity_mint: *accounts.borrow_reserve_liquidity_mint.key,
            reserve_source_liquidity: *accounts.reserve_source_liquidity.key,
            borrow_reserve_liquidity_fee_receiver: *accounts
                .borrow_reserve_liquidity_fee_receiver
                .key,
            user_destination_liquidity: *accounts.user_destination_liquidity.key,
            referrer_token_state: *accounts.referrer_token_state.key,
            token_program: *accounts.token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<BorrowObligationLiquidityKeys>
for [AccountMeta; BORROW_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: BorrowObligationLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.borrow_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrow_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrow_reserve_liquidity_fee_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_token_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BORROW_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN]>
for BorrowObligationLiquidityKeys {
    fn from(pubkeys: [Pubkey; BORROW_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            obligation: pubkeys[1],
            lending_market: pubkeys[2],
            lending_market_authority: pubkeys[3],
            borrow_reserve: pubkeys[4],
            borrow_reserve_liquidity_mint: pubkeys[5],
            reserve_source_liquidity: pubkeys[6],
            borrow_reserve_liquidity_fee_receiver: pubkeys[7],
            user_destination_liquidity: pubkeys[8],
            referrer_token_state: pubkeys[9],
            token_program: pubkeys[10],
            instruction_sysvar_account: pubkeys[11],
        }
    }
}
impl<'info> From<BorrowObligationLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; BORROW_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BorrowObligationLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.borrow_reserve.clone(),
            accounts.borrow_reserve_liquidity_mint.clone(),
            accounts.reserve_source_liquidity.clone(),
            accounts.borrow_reserve_liquidity_fee_receiver.clone(),
            accounts.user_destination_liquidity.clone(),
            accounts.referrer_token_state.clone(),
            accounts.token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; BORROW_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN]>
for BorrowObligationLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BORROW_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            obligation: &arr[1],
            lending_market: &arr[2],
            lending_market_authority: &arr[3],
            borrow_reserve: &arr[4],
            borrow_reserve_liquidity_mint: &arr[5],
            reserve_source_liquidity: &arr[6],
            borrow_reserve_liquidity_fee_receiver: &arr[7],
            user_destination_liquidity: &arr[8],
            referrer_token_state: &arr[9],
            token_program: &arr[10],
            instruction_sysvar_account: &arr[11],
        }
    }
}
pub const BORROW_OBLIGATION_LIQUIDITY_IX_DISCM: [u8; 8] = [
    121,
    127,
    18,
    204,
    73,
    245,
    225,
    65,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BorrowObligationLiquidityIxArgs {
    pub liquidity_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowObligationLiquidityIxData(pub BorrowObligationLiquidityIxArgs);
impl From<BorrowObligationLiquidityIxArgs> for BorrowObligationLiquidityIxData {
    fn from(args: BorrowObligationLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl BorrowObligationLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_OBLIGATION_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        BORROW_OBLIGATION_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(BorrowObligationLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_OBLIGATION_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn borrow_obligation_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: BorrowObligationLiquidityKeys,
    args: BorrowObligationLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BORROW_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: BorrowObligationLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn borrow_obligation_liquidity_ix(
    keys: BorrowObligationLiquidityKeys,
    args: BorrowObligationLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    borrow_obligation_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn borrow_obligation_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BorrowObligationLiquidityAccounts<'_, '_>,
    args: BorrowObligationLiquidityIxArgs,
) -> ProgramResult {
    let keys: BorrowObligationLiquidityKeys = accounts.into();
    let ix = borrow_obligation_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn borrow_obligation_liquidity_invoke(
    accounts: BorrowObligationLiquidityAccounts<'_, '_>,
    args: BorrowObligationLiquidityIxArgs,
) -> ProgramResult {
    borrow_obligation_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn borrow_obligation_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BorrowObligationLiquidityAccounts<'_, '_>,
    args: BorrowObligationLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BorrowObligationLiquidityKeys = accounts.into();
    let ix = borrow_obligation_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn borrow_obligation_liquidity_invoke_signed(
    accounts: BorrowObligationLiquidityAccounts<'_, '_>,
    args: BorrowObligationLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    borrow_obligation_liquidity_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn borrow_obligation_liquidity_verify_account_keys(
    accounts: BorrowObligationLiquidityAccounts<'_, '_>,
    keys: BorrowObligationLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.borrow_reserve.key, keys.borrow_reserve),
        (
            *accounts.borrow_reserve_liquidity_mint.key,
            keys.borrow_reserve_liquidity_mint,
        ),
        (*accounts.reserve_source_liquidity.key, keys.reserve_source_liquidity),
        (
            *accounts.borrow_reserve_liquidity_fee_receiver.key,
            keys.borrow_reserve_liquidity_fee_receiver,
        ),
        (*accounts.user_destination_liquidity.key, keys.user_destination_liquidity),
        (*accounts.referrer_token_state.key, keys.referrer_token_state),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn borrow_obligation_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: BorrowObligationLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.obligation,
        accounts.borrow_reserve,
        accounts.reserve_source_liquidity,
        accounts.borrow_reserve_liquidity_fee_receiver,
        accounts.user_destination_liquidity,
        accounts.referrer_token_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn borrow_obligation_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: BorrowObligationLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn borrow_obligation_liquidity_verify_account_privileges<'me, 'info>(
    accounts: BorrowObligationLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    borrow_obligation_liquidity_verify_writable_privileges(accounts)?;
    borrow_obligation_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BORROW_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct BorrowObligationLiquidityV2Accounts<'me, 'info> {
    pub borrow_accounts_owner: &'me AccountInfo<'info>,
    pub borrow_accounts_obligation: &'me AccountInfo<'info>,
    pub borrow_accounts_lending_market: &'me AccountInfo<'info>,
    pub borrow_accounts_lending_market_authority: &'me AccountInfo<'info>,
    pub borrow_accounts_borrow_reserve: &'me AccountInfo<'info>,
    pub borrow_accounts_borrow_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub borrow_accounts_reserve_source_liquidity: &'me AccountInfo<'info>,
    pub borrow_accounts_borrow_reserve_liquidity_fee_receiver: &'me AccountInfo<'info>,
    pub borrow_accounts_user_destination_liquidity: &'me AccountInfo<'info>,
    pub borrow_accounts_referrer_token_state: &'me AccountInfo<'info>,
    pub borrow_accounts_token_program: &'me AccountInfo<'info>,
    pub borrow_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorrowObligationLiquidityV2Keys {
    pub borrow_accounts_owner: Pubkey,
    pub borrow_accounts_obligation: Pubkey,
    pub borrow_accounts_lending_market: Pubkey,
    pub borrow_accounts_lending_market_authority: Pubkey,
    pub borrow_accounts_borrow_reserve: Pubkey,
    pub borrow_accounts_borrow_reserve_liquidity_mint: Pubkey,
    pub borrow_accounts_reserve_source_liquidity: Pubkey,
    pub borrow_accounts_borrow_reserve_liquidity_fee_receiver: Pubkey,
    pub borrow_accounts_user_destination_liquidity: Pubkey,
    pub borrow_accounts_referrer_token_state: Pubkey,
    pub borrow_accounts_token_program: Pubkey,
    pub borrow_accounts_instruction_sysvar_account: Pubkey,
    pub farms_accounts_obligation_farm_user_state: Pubkey,
    pub farms_accounts_reserve_farm_state: Pubkey,
    pub farms_program: Pubkey,
}
impl From<BorrowObligationLiquidityV2Accounts<'_, '_>>
for BorrowObligationLiquidityV2Keys {
    fn from(accounts: BorrowObligationLiquidityV2Accounts) -> Self {
        Self {
            borrow_accounts_owner: *accounts.borrow_accounts_owner.key,
            borrow_accounts_obligation: *accounts.borrow_accounts_obligation.key,
            borrow_accounts_lending_market: *accounts.borrow_accounts_lending_market.key,
            borrow_accounts_lending_market_authority: *accounts
                .borrow_accounts_lending_market_authority
                .key,
            borrow_accounts_borrow_reserve: *accounts.borrow_accounts_borrow_reserve.key,
            borrow_accounts_borrow_reserve_liquidity_mint: *accounts
                .borrow_accounts_borrow_reserve_liquidity_mint
                .key,
            borrow_accounts_reserve_source_liquidity: *accounts
                .borrow_accounts_reserve_source_liquidity
                .key,
            borrow_accounts_borrow_reserve_liquidity_fee_receiver: *accounts
                .borrow_accounts_borrow_reserve_liquidity_fee_receiver
                .key,
            borrow_accounts_user_destination_liquidity: *accounts
                .borrow_accounts_user_destination_liquidity
                .key,
            borrow_accounts_referrer_token_state: *accounts
                .borrow_accounts_referrer_token_state
                .key,
            borrow_accounts_token_program: *accounts.borrow_accounts_token_program.key,
            borrow_accounts_instruction_sysvar_account: *accounts
                .borrow_accounts_instruction_sysvar_account
                .key,
            farms_accounts_obligation_farm_user_state: *accounts
                .farms_accounts_obligation_farm_user_state
                .key,
            farms_accounts_reserve_farm_state: *accounts
                .farms_accounts_reserve_farm_state
                .key,
            farms_program: *accounts.farms_program.key,
        }
    }
}
impl From<BorrowObligationLiquidityV2Keys>
for [AccountMeta; BORROW_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: BorrowObligationLiquidityV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.borrow_accounts_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.borrow_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrow_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.borrow_accounts_lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.borrow_accounts_borrow_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrow_accounts_borrow_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.borrow_accounts_reserve_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrow_accounts_borrow_reserve_liquidity_fee_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrow_accounts_user_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrow_accounts_referrer_token_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.borrow_accounts_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.borrow_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BORROW_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN]>
for BorrowObligationLiquidityV2Keys {
    fn from(pubkeys: [Pubkey; BORROW_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            borrow_accounts_owner: pubkeys[0],
            borrow_accounts_obligation: pubkeys[1],
            borrow_accounts_lending_market: pubkeys[2],
            borrow_accounts_lending_market_authority: pubkeys[3],
            borrow_accounts_borrow_reserve: pubkeys[4],
            borrow_accounts_borrow_reserve_liquidity_mint: pubkeys[5],
            borrow_accounts_reserve_source_liquidity: pubkeys[6],
            borrow_accounts_borrow_reserve_liquidity_fee_receiver: pubkeys[7],
            borrow_accounts_user_destination_liquidity: pubkeys[8],
            borrow_accounts_referrer_token_state: pubkeys[9],
            borrow_accounts_token_program: pubkeys[10],
            borrow_accounts_instruction_sysvar_account: pubkeys[11],
            farms_accounts_obligation_farm_user_state: pubkeys[12],
            farms_accounts_reserve_farm_state: pubkeys[13],
            farms_program: pubkeys[14],
        }
    }
}
impl<'info> From<BorrowObligationLiquidityV2Accounts<'_, 'info>>
for [AccountInfo<'info>; BORROW_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: BorrowObligationLiquidityV2Accounts<'_, 'info>) -> Self {
        [
            accounts.borrow_accounts_owner.clone(),
            accounts.borrow_accounts_obligation.clone(),
            accounts.borrow_accounts_lending_market.clone(),
            accounts.borrow_accounts_lending_market_authority.clone(),
            accounts.borrow_accounts_borrow_reserve.clone(),
            accounts.borrow_accounts_borrow_reserve_liquidity_mint.clone(),
            accounts.borrow_accounts_reserve_source_liquidity.clone(),
            accounts.borrow_accounts_borrow_reserve_liquidity_fee_receiver.clone(),
            accounts.borrow_accounts_user_destination_liquidity.clone(),
            accounts.borrow_accounts_referrer_token_state.clone(),
            accounts.borrow_accounts_token_program.clone(),
            accounts.borrow_accounts_instruction_sysvar_account.clone(),
            accounts.farms_accounts_obligation_farm_user_state.clone(),
            accounts.farms_accounts_reserve_farm_state.clone(),
            accounts.farms_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; BORROW_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN]>
for BorrowObligationLiquidityV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; BORROW_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            borrow_accounts_owner: &arr[0],
            borrow_accounts_obligation: &arr[1],
            borrow_accounts_lending_market: &arr[2],
            borrow_accounts_lending_market_authority: &arr[3],
            borrow_accounts_borrow_reserve: &arr[4],
            borrow_accounts_borrow_reserve_liquidity_mint: &arr[5],
            borrow_accounts_reserve_source_liquidity: &arr[6],
            borrow_accounts_borrow_reserve_liquidity_fee_receiver: &arr[7],
            borrow_accounts_user_destination_liquidity: &arr[8],
            borrow_accounts_referrer_token_state: &arr[9],
            borrow_accounts_token_program: &arr[10],
            borrow_accounts_instruction_sysvar_account: &arr[11],
            farms_accounts_obligation_farm_user_state: &arr[12],
            farms_accounts_reserve_farm_state: &arr[13],
            farms_program: &arr[14],
        }
    }
}
pub const BORROW_OBLIGATION_LIQUIDITY_V2_IX_DISCM: [u8; 8] = [
    161,
    128,
    143,
    245,
    171,
    199,
    194,
    6,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BorrowObligationLiquidityV2IxArgs {
    pub liquidity_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BorrowObligationLiquidityV2IxData(pub BorrowObligationLiquidityV2IxArgs);
impl From<BorrowObligationLiquidityV2IxArgs> for BorrowObligationLiquidityV2IxData {
    fn from(args: BorrowObligationLiquidityV2IxArgs) -> Self {
        Self(args)
    }
}
impl BorrowObligationLiquidityV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BORROW_OBLIGATION_LIQUIDITY_V2_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        BORROW_OBLIGATION_LIQUIDITY_V2_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(BorrowObligationLiquidityV2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BORROW_OBLIGATION_LIQUIDITY_V2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn borrow_obligation_liquidity_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: BorrowObligationLiquidityV2Keys,
    args: BorrowObligationLiquidityV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BORROW_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: BorrowObligationLiquidityV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn borrow_obligation_liquidity_v2_ix(
    keys: BorrowObligationLiquidityV2Keys,
    args: BorrowObligationLiquidityV2IxArgs,
) -> std::io::Result<Instruction> {
    borrow_obligation_liquidity_v2_ix_with_program_id(crate::ID, keys, args)
}
pub fn borrow_obligation_liquidity_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BorrowObligationLiquidityV2Accounts<'_, '_>,
    args: BorrowObligationLiquidityV2IxArgs,
) -> ProgramResult {
    let keys: BorrowObligationLiquidityV2Keys = accounts.into();
    let ix = borrow_obligation_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn borrow_obligation_liquidity_v2_invoke(
    accounts: BorrowObligationLiquidityV2Accounts<'_, '_>,
    args: BorrowObligationLiquidityV2IxArgs,
) -> ProgramResult {
    borrow_obligation_liquidity_v2_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn borrow_obligation_liquidity_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BorrowObligationLiquidityV2Accounts<'_, '_>,
    args: BorrowObligationLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BorrowObligationLiquidityV2Keys = accounts.into();
    let ix = borrow_obligation_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn borrow_obligation_liquidity_v2_invoke_signed(
    accounts: BorrowObligationLiquidityV2Accounts<'_, '_>,
    args: BorrowObligationLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    borrow_obligation_liquidity_v2_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn borrow_obligation_liquidity_v2_verify_account_keys(
    accounts: BorrowObligationLiquidityV2Accounts<'_, '_>,
    keys: BorrowObligationLiquidityV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.borrow_accounts_owner.key, keys.borrow_accounts_owner),
        (*accounts.borrow_accounts_obligation.key, keys.borrow_accounts_obligation),
        (
            *accounts.borrow_accounts_lending_market.key,
            keys.borrow_accounts_lending_market,
        ),
        (
            *accounts.borrow_accounts_lending_market_authority.key,
            keys.borrow_accounts_lending_market_authority,
        ),
        (
            *accounts.borrow_accounts_borrow_reserve.key,
            keys.borrow_accounts_borrow_reserve,
        ),
        (
            *accounts.borrow_accounts_borrow_reserve_liquidity_mint.key,
            keys.borrow_accounts_borrow_reserve_liquidity_mint,
        ),
        (
            *accounts.borrow_accounts_reserve_source_liquidity.key,
            keys.borrow_accounts_reserve_source_liquidity,
        ),
        (
            *accounts.borrow_accounts_borrow_reserve_liquidity_fee_receiver.key,
            keys.borrow_accounts_borrow_reserve_liquidity_fee_receiver,
        ),
        (
            *accounts.borrow_accounts_user_destination_liquidity.key,
            keys.borrow_accounts_user_destination_liquidity,
        ),
        (
            *accounts.borrow_accounts_referrer_token_state.key,
            keys.borrow_accounts_referrer_token_state,
        ),
        (
            *accounts.borrow_accounts_token_program.key,
            keys.borrow_accounts_token_program,
        ),
        (
            *accounts.borrow_accounts_instruction_sysvar_account.key,
            keys.borrow_accounts_instruction_sysvar_account,
        ),
        (
            *accounts.farms_accounts_obligation_farm_user_state.key,
            keys.farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.farms_accounts_reserve_farm_state.key,
            keys.farms_accounts_reserve_farm_state,
        ),
        (*accounts.farms_program.key, keys.farms_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn borrow_obligation_liquidity_v2_verify_writable_privileges<'me, 'info>(
    accounts: BorrowObligationLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.borrow_accounts_obligation,
        accounts.borrow_accounts_borrow_reserve,
        accounts.borrow_accounts_reserve_source_liquidity,
        accounts.borrow_accounts_borrow_reserve_liquidity_fee_receiver,
        accounts.borrow_accounts_user_destination_liquidity,
        accounts.borrow_accounts_referrer_token_state,
        accounts.farms_accounts_obligation_farm_user_state,
        accounts.farms_accounts_reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn borrow_obligation_liquidity_v2_verify_signer_privileges<'me, 'info>(
    accounts: BorrowObligationLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.borrow_accounts_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn borrow_obligation_liquidity_v2_verify_account_privileges<'me, 'info>(
    accounts: BorrowObligationLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    borrow_obligation_liquidity_v2_verify_writable_privileges(accounts)?;
    borrow_obligation_liquidity_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REPAY_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct RepayObligationLiquidityAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub repay_reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub reserve_destination_liquidity: &'me AccountInfo<'info>,
    pub user_source_liquidity: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RepayObligationLiquidityKeys {
    pub owner: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub repay_reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_destination_liquidity: Pubkey,
    pub user_source_liquidity: Pubkey,
    pub token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<RepayObligationLiquidityAccounts<'_, '_>> for RepayObligationLiquidityKeys {
    fn from(accounts: RepayObligationLiquidityAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            repay_reserve: *accounts.repay_reserve.key,
            reserve_liquidity_mint: *accounts.reserve_liquidity_mint.key,
            reserve_destination_liquidity: *accounts.reserve_destination_liquidity.key,
            user_source_liquidity: *accounts.user_source_liquidity.key,
            token_program: *accounts.token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<RepayObligationLiquidityKeys>
for [AccountMeta; REPAY_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: RepayObligationLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REPAY_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RepayObligationLiquidityKeys {
    fn from(pubkeys: [Pubkey; REPAY_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            obligation: pubkeys[1],
            lending_market: pubkeys[2],
            repay_reserve: pubkeys[3],
            reserve_liquidity_mint: pubkeys[4],
            reserve_destination_liquidity: pubkeys[5],
            user_source_liquidity: pubkeys[6],
            token_program: pubkeys[7],
            instruction_sysvar_account: pubkeys[8],
        }
    }
}
impl<'info> From<RepayObligationLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; REPAY_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: RepayObligationLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.repay_reserve.clone(),
            accounts.reserve_liquidity_mint.clone(),
            accounts.reserve_destination_liquidity.clone(),
            accounts.user_source_liquidity.clone(),
            accounts.token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REPAY_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN]>
for RepayObligationLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REPAY_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            obligation: &arr[1],
            lending_market: &arr[2],
            repay_reserve: &arr[3],
            reserve_liquidity_mint: &arr[4],
            reserve_destination_liquidity: &arr[5],
            user_source_liquidity: &arr[6],
            token_program: &arr[7],
            instruction_sysvar_account: &arr[8],
        }
    }
}
pub const REPAY_OBLIGATION_LIQUIDITY_IX_DISCM: [u8; 8] = [
    145,
    178,
    13,
    225,
    76,
    240,
    147,
    72,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RepayObligationLiquidityIxArgs {
    pub liquidity_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RepayObligationLiquidityIxData(pub RepayObligationLiquidityIxArgs);
impl From<RepayObligationLiquidityIxArgs> for RepayObligationLiquidityIxData {
    fn from(args: RepayObligationLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl RepayObligationLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REPAY_OBLIGATION_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REPAY_OBLIGATION_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RepayObligationLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REPAY_OBLIGATION_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn repay_obligation_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: RepayObligationLiquidityKeys,
    args: RepayObligationLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REPAY_OBLIGATION_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RepayObligationLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn repay_obligation_liquidity_ix(
    keys: RepayObligationLiquidityKeys,
    args: RepayObligationLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    repay_obligation_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn repay_obligation_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RepayObligationLiquidityAccounts<'_, '_>,
    args: RepayObligationLiquidityIxArgs,
) -> ProgramResult {
    let keys: RepayObligationLiquidityKeys = accounts.into();
    let ix = repay_obligation_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn repay_obligation_liquidity_invoke(
    accounts: RepayObligationLiquidityAccounts<'_, '_>,
    args: RepayObligationLiquidityIxArgs,
) -> ProgramResult {
    repay_obligation_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn repay_obligation_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RepayObligationLiquidityAccounts<'_, '_>,
    args: RepayObligationLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RepayObligationLiquidityKeys = accounts.into();
    let ix = repay_obligation_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn repay_obligation_liquidity_invoke_signed(
    accounts: RepayObligationLiquidityAccounts<'_, '_>,
    args: RepayObligationLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    repay_obligation_liquidity_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn repay_obligation_liquidity_verify_account_keys(
    accounts: RepayObligationLiquidityAccounts<'_, '_>,
    keys: RepayObligationLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.repay_reserve.key, keys.repay_reserve),
        (*accounts.reserve_liquidity_mint.key, keys.reserve_liquidity_mint),
        (
            *accounts.reserve_destination_liquidity.key,
            keys.reserve_destination_liquidity,
        ),
        (*accounts.user_source_liquidity.key, keys.user_source_liquidity),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn repay_obligation_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: RepayObligationLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.obligation,
        accounts.repay_reserve,
        accounts.reserve_destination_liquidity,
        accounts.user_source_liquidity,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn repay_obligation_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: RepayObligationLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn repay_obligation_liquidity_verify_account_privileges<'me, 'info>(
    accounts: RepayObligationLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    repay_obligation_liquidity_verify_writable_privileges(accounts)?;
    repay_obligation_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REPAY_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct RepayObligationLiquidityV2Accounts<'me, 'info> {
    pub repay_accounts_owner: &'me AccountInfo<'info>,
    pub repay_accounts_obligation: &'me AccountInfo<'info>,
    pub repay_accounts_lending_market: &'me AccountInfo<'info>,
    pub repay_accounts_repay_reserve: &'me AccountInfo<'info>,
    pub repay_accounts_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub repay_accounts_reserve_destination_liquidity: &'me AccountInfo<'info>,
    pub repay_accounts_user_source_liquidity: &'me AccountInfo<'info>,
    pub repay_accounts_token_program: &'me AccountInfo<'info>,
    pub repay_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RepayObligationLiquidityV2Keys {
    pub repay_accounts_owner: Pubkey,
    pub repay_accounts_obligation: Pubkey,
    pub repay_accounts_lending_market: Pubkey,
    pub repay_accounts_repay_reserve: Pubkey,
    pub repay_accounts_reserve_liquidity_mint: Pubkey,
    pub repay_accounts_reserve_destination_liquidity: Pubkey,
    pub repay_accounts_user_source_liquidity: Pubkey,
    pub repay_accounts_token_program: Pubkey,
    pub repay_accounts_instruction_sysvar_account: Pubkey,
    pub farms_accounts_obligation_farm_user_state: Pubkey,
    pub farms_accounts_reserve_farm_state: Pubkey,
    pub lending_market_authority: Pubkey,
    pub farms_program: Pubkey,
}
impl From<RepayObligationLiquidityV2Accounts<'_, '_>>
for RepayObligationLiquidityV2Keys {
    fn from(accounts: RepayObligationLiquidityV2Accounts) -> Self {
        Self {
            repay_accounts_owner: *accounts.repay_accounts_owner.key,
            repay_accounts_obligation: *accounts.repay_accounts_obligation.key,
            repay_accounts_lending_market: *accounts.repay_accounts_lending_market.key,
            repay_accounts_repay_reserve: *accounts.repay_accounts_repay_reserve.key,
            repay_accounts_reserve_liquidity_mint: *accounts
                .repay_accounts_reserve_liquidity_mint
                .key,
            repay_accounts_reserve_destination_liquidity: *accounts
                .repay_accounts_reserve_destination_liquidity
                .key,
            repay_accounts_user_source_liquidity: *accounts
                .repay_accounts_user_source_liquidity
                .key,
            repay_accounts_token_program: *accounts.repay_accounts_token_program.key,
            repay_accounts_instruction_sysvar_account: *accounts
                .repay_accounts_instruction_sysvar_account
                .key,
            farms_accounts_obligation_farm_user_state: *accounts
                .farms_accounts_obligation_farm_user_state
                .key,
            farms_accounts_reserve_farm_state: *accounts
                .farms_accounts_reserve_farm_state
                .key,
            lending_market_authority: *accounts.lending_market_authority.key,
            farms_program: *accounts.farms_program.key,
        }
    }
}
impl From<RepayObligationLiquidityV2Keys>
for [AccountMeta; REPAY_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: RepayObligationLiquidityV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.repay_accounts_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_repay_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_reserve_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_user_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REPAY_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN]>
for RepayObligationLiquidityV2Keys {
    fn from(pubkeys: [Pubkey; REPAY_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            repay_accounts_owner: pubkeys[0],
            repay_accounts_obligation: pubkeys[1],
            repay_accounts_lending_market: pubkeys[2],
            repay_accounts_repay_reserve: pubkeys[3],
            repay_accounts_reserve_liquidity_mint: pubkeys[4],
            repay_accounts_reserve_destination_liquidity: pubkeys[5],
            repay_accounts_user_source_liquidity: pubkeys[6],
            repay_accounts_token_program: pubkeys[7],
            repay_accounts_instruction_sysvar_account: pubkeys[8],
            farms_accounts_obligation_farm_user_state: pubkeys[9],
            farms_accounts_reserve_farm_state: pubkeys[10],
            lending_market_authority: pubkeys[11],
            farms_program: pubkeys[12],
        }
    }
}
impl<'info> From<RepayObligationLiquidityV2Accounts<'_, 'info>>
for [AccountInfo<'info>; REPAY_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN] {
    fn from(accounts: RepayObligationLiquidityV2Accounts<'_, 'info>) -> Self {
        [
            accounts.repay_accounts_owner.clone(),
            accounts.repay_accounts_obligation.clone(),
            accounts.repay_accounts_lending_market.clone(),
            accounts.repay_accounts_repay_reserve.clone(),
            accounts.repay_accounts_reserve_liquidity_mint.clone(),
            accounts.repay_accounts_reserve_destination_liquidity.clone(),
            accounts.repay_accounts_user_source_liquidity.clone(),
            accounts.repay_accounts_token_program.clone(),
            accounts.repay_accounts_instruction_sysvar_account.clone(),
            accounts.farms_accounts_obligation_farm_user_state.clone(),
            accounts.farms_accounts_reserve_farm_state.clone(),
            accounts.lending_market_authority.clone(),
            accounts.farms_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REPAY_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN]>
for RepayObligationLiquidityV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REPAY_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            repay_accounts_owner: &arr[0],
            repay_accounts_obligation: &arr[1],
            repay_accounts_lending_market: &arr[2],
            repay_accounts_repay_reserve: &arr[3],
            repay_accounts_reserve_liquidity_mint: &arr[4],
            repay_accounts_reserve_destination_liquidity: &arr[5],
            repay_accounts_user_source_liquidity: &arr[6],
            repay_accounts_token_program: &arr[7],
            repay_accounts_instruction_sysvar_account: &arr[8],
            farms_accounts_obligation_farm_user_state: &arr[9],
            farms_accounts_reserve_farm_state: &arr[10],
            lending_market_authority: &arr[11],
            farms_program: &arr[12],
        }
    }
}
pub const REPAY_OBLIGATION_LIQUIDITY_V2_IX_DISCM: [u8; 8] = [
    116,
    174,
    213,
    76,
    180,
    53,
    210,
    144,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RepayObligationLiquidityV2IxArgs {
    pub liquidity_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RepayObligationLiquidityV2IxData(pub RepayObligationLiquidityV2IxArgs);
impl From<RepayObligationLiquidityV2IxArgs> for RepayObligationLiquidityV2IxData {
    fn from(args: RepayObligationLiquidityV2IxArgs) -> Self {
        Self(args)
    }
}
impl RepayObligationLiquidityV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REPAY_OBLIGATION_LIQUIDITY_V2_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REPAY_OBLIGATION_LIQUIDITY_V2_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RepayObligationLiquidityV2IxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REPAY_OBLIGATION_LIQUIDITY_V2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn repay_obligation_liquidity_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: RepayObligationLiquidityV2Keys,
    args: RepayObligationLiquidityV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REPAY_OBLIGATION_LIQUIDITY_V2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: RepayObligationLiquidityV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn repay_obligation_liquidity_v2_ix(
    keys: RepayObligationLiquidityV2Keys,
    args: RepayObligationLiquidityV2IxArgs,
) -> std::io::Result<Instruction> {
    repay_obligation_liquidity_v2_ix_with_program_id(crate::ID, keys, args)
}
pub fn repay_obligation_liquidity_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RepayObligationLiquidityV2Accounts<'_, '_>,
    args: RepayObligationLiquidityV2IxArgs,
) -> ProgramResult {
    let keys: RepayObligationLiquidityV2Keys = accounts.into();
    let ix = repay_obligation_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn repay_obligation_liquidity_v2_invoke(
    accounts: RepayObligationLiquidityV2Accounts<'_, '_>,
    args: RepayObligationLiquidityV2IxArgs,
) -> ProgramResult {
    repay_obligation_liquidity_v2_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn repay_obligation_liquidity_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RepayObligationLiquidityV2Accounts<'_, '_>,
    args: RepayObligationLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RepayObligationLiquidityV2Keys = accounts.into();
    let ix = repay_obligation_liquidity_v2_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn repay_obligation_liquidity_v2_invoke_signed(
    accounts: RepayObligationLiquidityV2Accounts<'_, '_>,
    args: RepayObligationLiquidityV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    repay_obligation_liquidity_v2_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn repay_obligation_liquidity_v2_verify_account_keys(
    accounts: RepayObligationLiquidityV2Accounts<'_, '_>,
    keys: RepayObligationLiquidityV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.repay_accounts_owner.key, keys.repay_accounts_owner),
        (*accounts.repay_accounts_obligation.key, keys.repay_accounts_obligation),
        (
            *accounts.repay_accounts_lending_market.key,
            keys.repay_accounts_lending_market,
        ),
        (*accounts.repay_accounts_repay_reserve.key, keys.repay_accounts_repay_reserve),
        (
            *accounts.repay_accounts_reserve_liquidity_mint.key,
            keys.repay_accounts_reserve_liquidity_mint,
        ),
        (
            *accounts.repay_accounts_reserve_destination_liquidity.key,
            keys.repay_accounts_reserve_destination_liquidity,
        ),
        (
            *accounts.repay_accounts_user_source_liquidity.key,
            keys.repay_accounts_user_source_liquidity,
        ),
        (*accounts.repay_accounts_token_program.key, keys.repay_accounts_token_program),
        (
            *accounts.repay_accounts_instruction_sysvar_account.key,
            keys.repay_accounts_instruction_sysvar_account,
        ),
        (
            *accounts.farms_accounts_obligation_farm_user_state.key,
            keys.farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.farms_accounts_reserve_farm_state.key,
            keys.farms_accounts_reserve_farm_state,
        ),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.farms_program.key, keys.farms_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn repay_obligation_liquidity_v2_verify_writable_privileges<'me, 'info>(
    accounts: RepayObligationLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.repay_accounts_obligation,
        accounts.repay_accounts_repay_reserve,
        accounts.repay_accounts_reserve_destination_liquidity,
        accounts.repay_accounts_user_source_liquidity,
        accounts.farms_accounts_obligation_farm_user_state,
        accounts.farms_accounts_reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn repay_obligation_liquidity_v2_verify_signer_privileges<'me, 'info>(
    accounts: RepayObligationLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.repay_accounts_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn repay_obligation_liquidity_v2_verify_account_privileges<'me, 'info>(
    accounts: RepayObligationLiquidityV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    repay_obligation_liquidity_v2_verify_writable_privileges(accounts)?;
    repay_obligation_liquidity_v2_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REPAY_AND_WITHDRAW_AND_REDEEM_IX_ACCOUNTS_LEN: usize = 28;
#[derive(Copy, Clone, Debug)]
pub struct RepayAndWithdrawAndRedeemAccounts<'me, 'info> {
    pub repay_accounts_owner: &'me AccountInfo<'info>,
    pub repay_accounts_obligation: &'me AccountInfo<'info>,
    pub repay_accounts_lending_market: &'me AccountInfo<'info>,
    pub repay_accounts_repay_reserve: &'me AccountInfo<'info>,
    pub repay_accounts_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub repay_accounts_reserve_destination_liquidity: &'me AccountInfo<'info>,
    pub repay_accounts_user_source_liquidity: &'me AccountInfo<'info>,
    pub repay_accounts_token_program: &'me AccountInfo<'info>,
    pub repay_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub withdraw_accounts_owner: &'me AccountInfo<'info>,
    pub withdraw_accounts_obligation: &'me AccountInfo<'info>,
    pub withdraw_accounts_lending_market: &'me AccountInfo<'info>,
    pub withdraw_accounts_lending_market_authority: &'me AccountInfo<'info>,
    pub withdraw_accounts_withdraw_reserve: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_source_collateral: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_collateral_mint: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub withdraw_accounts_user_destination_liquidity: &'me AccountInfo<'info>,
    pub withdraw_accounts_placeholder_user_destination_collateral: &'me AccountInfo<
        'info,
    >,
    pub withdraw_accounts_collateral_token_program: &'me AccountInfo<'info>,
    pub withdraw_accounts_liquidity_token_program: &'me AccountInfo<'info>,
    pub withdraw_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub collateral_farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub collateral_farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub debt_farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub debt_farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RepayAndWithdrawAndRedeemKeys {
    pub repay_accounts_owner: Pubkey,
    pub repay_accounts_obligation: Pubkey,
    pub repay_accounts_lending_market: Pubkey,
    pub repay_accounts_repay_reserve: Pubkey,
    pub repay_accounts_reserve_liquidity_mint: Pubkey,
    pub repay_accounts_reserve_destination_liquidity: Pubkey,
    pub repay_accounts_user_source_liquidity: Pubkey,
    pub repay_accounts_token_program: Pubkey,
    pub repay_accounts_instruction_sysvar_account: Pubkey,
    pub withdraw_accounts_owner: Pubkey,
    pub withdraw_accounts_obligation: Pubkey,
    pub withdraw_accounts_lending_market: Pubkey,
    pub withdraw_accounts_lending_market_authority: Pubkey,
    pub withdraw_accounts_withdraw_reserve: Pubkey,
    pub withdraw_accounts_reserve_liquidity_mint: Pubkey,
    pub withdraw_accounts_reserve_source_collateral: Pubkey,
    pub withdraw_accounts_reserve_collateral_mint: Pubkey,
    pub withdraw_accounts_reserve_liquidity_supply: Pubkey,
    pub withdraw_accounts_user_destination_liquidity: Pubkey,
    pub withdraw_accounts_placeholder_user_destination_collateral: Pubkey,
    pub withdraw_accounts_collateral_token_program: Pubkey,
    pub withdraw_accounts_liquidity_token_program: Pubkey,
    pub withdraw_accounts_instruction_sysvar_account: Pubkey,
    pub collateral_farms_accounts_obligation_farm_user_state: Pubkey,
    pub collateral_farms_accounts_reserve_farm_state: Pubkey,
    pub debt_farms_accounts_obligation_farm_user_state: Pubkey,
    pub debt_farms_accounts_reserve_farm_state: Pubkey,
    pub farms_program: Pubkey,
}
impl From<RepayAndWithdrawAndRedeemAccounts<'_, '_>> for RepayAndWithdrawAndRedeemKeys {
    fn from(accounts: RepayAndWithdrawAndRedeemAccounts) -> Self {
        Self {
            repay_accounts_owner: *accounts.repay_accounts_owner.key,
            repay_accounts_obligation: *accounts.repay_accounts_obligation.key,
            repay_accounts_lending_market: *accounts.repay_accounts_lending_market.key,
            repay_accounts_repay_reserve: *accounts.repay_accounts_repay_reserve.key,
            repay_accounts_reserve_liquidity_mint: *accounts
                .repay_accounts_reserve_liquidity_mint
                .key,
            repay_accounts_reserve_destination_liquidity: *accounts
                .repay_accounts_reserve_destination_liquidity
                .key,
            repay_accounts_user_source_liquidity: *accounts
                .repay_accounts_user_source_liquidity
                .key,
            repay_accounts_token_program: *accounts.repay_accounts_token_program.key,
            repay_accounts_instruction_sysvar_account: *accounts
                .repay_accounts_instruction_sysvar_account
                .key,
            withdraw_accounts_owner: *accounts.withdraw_accounts_owner.key,
            withdraw_accounts_obligation: *accounts.withdraw_accounts_obligation.key,
            withdraw_accounts_lending_market: *accounts
                .withdraw_accounts_lending_market
                .key,
            withdraw_accounts_lending_market_authority: *accounts
                .withdraw_accounts_lending_market_authority
                .key,
            withdraw_accounts_withdraw_reserve: *accounts
                .withdraw_accounts_withdraw_reserve
                .key,
            withdraw_accounts_reserve_liquidity_mint: *accounts
                .withdraw_accounts_reserve_liquidity_mint
                .key,
            withdraw_accounts_reserve_source_collateral: *accounts
                .withdraw_accounts_reserve_source_collateral
                .key,
            withdraw_accounts_reserve_collateral_mint: *accounts
                .withdraw_accounts_reserve_collateral_mint
                .key,
            withdraw_accounts_reserve_liquidity_supply: *accounts
                .withdraw_accounts_reserve_liquidity_supply
                .key,
            withdraw_accounts_user_destination_liquidity: *accounts
                .withdraw_accounts_user_destination_liquidity
                .key,
            withdraw_accounts_placeholder_user_destination_collateral: *accounts
                .withdraw_accounts_placeholder_user_destination_collateral
                .key,
            withdraw_accounts_collateral_token_program: *accounts
                .withdraw_accounts_collateral_token_program
                .key,
            withdraw_accounts_liquidity_token_program: *accounts
                .withdraw_accounts_liquidity_token_program
                .key,
            withdraw_accounts_instruction_sysvar_account: *accounts
                .withdraw_accounts_instruction_sysvar_account
                .key,
            collateral_farms_accounts_obligation_farm_user_state: *accounts
                .collateral_farms_accounts_obligation_farm_user_state
                .key,
            collateral_farms_accounts_reserve_farm_state: *accounts
                .collateral_farms_accounts_reserve_farm_state
                .key,
            debt_farms_accounts_obligation_farm_user_state: *accounts
                .debt_farms_accounts_obligation_farm_user_state
                .key,
            debt_farms_accounts_reserve_farm_state: *accounts
                .debt_farms_accounts_reserve_farm_state
                .key,
            farms_program: *accounts.farms_program.key,
        }
    }
}
impl From<RepayAndWithdrawAndRedeemKeys>
for [AccountMeta; REPAY_AND_WITHDRAW_AND_REDEEM_IX_ACCOUNTS_LEN] {
    fn from(keys: RepayAndWithdrawAndRedeemKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.repay_accounts_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_repay_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_reserve_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_user_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_withdraw_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_source_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_user_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_placeholder_user_destination_collateral,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.debt_farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.debt_farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REPAY_AND_WITHDRAW_AND_REDEEM_IX_ACCOUNTS_LEN]>
for RepayAndWithdrawAndRedeemKeys {
    fn from(pubkeys: [Pubkey; REPAY_AND_WITHDRAW_AND_REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            repay_accounts_owner: pubkeys[0],
            repay_accounts_obligation: pubkeys[1],
            repay_accounts_lending_market: pubkeys[2],
            repay_accounts_repay_reserve: pubkeys[3],
            repay_accounts_reserve_liquidity_mint: pubkeys[4],
            repay_accounts_reserve_destination_liquidity: pubkeys[5],
            repay_accounts_user_source_liquidity: pubkeys[6],
            repay_accounts_token_program: pubkeys[7],
            repay_accounts_instruction_sysvar_account: pubkeys[8],
            withdraw_accounts_owner: pubkeys[9],
            withdraw_accounts_obligation: pubkeys[10],
            withdraw_accounts_lending_market: pubkeys[11],
            withdraw_accounts_lending_market_authority: pubkeys[12],
            withdraw_accounts_withdraw_reserve: pubkeys[13],
            withdraw_accounts_reserve_liquidity_mint: pubkeys[14],
            withdraw_accounts_reserve_source_collateral: pubkeys[15],
            withdraw_accounts_reserve_collateral_mint: pubkeys[16],
            withdraw_accounts_reserve_liquidity_supply: pubkeys[17],
            withdraw_accounts_user_destination_liquidity: pubkeys[18],
            withdraw_accounts_placeholder_user_destination_collateral: pubkeys[19],
            withdraw_accounts_collateral_token_program: pubkeys[20],
            withdraw_accounts_liquidity_token_program: pubkeys[21],
            withdraw_accounts_instruction_sysvar_account: pubkeys[22],
            collateral_farms_accounts_obligation_farm_user_state: pubkeys[23],
            collateral_farms_accounts_reserve_farm_state: pubkeys[24],
            debt_farms_accounts_obligation_farm_user_state: pubkeys[25],
            debt_farms_accounts_reserve_farm_state: pubkeys[26],
            farms_program: pubkeys[27],
        }
    }
}
impl<'info> From<RepayAndWithdrawAndRedeemAccounts<'_, 'info>>
for [AccountInfo<'info>; REPAY_AND_WITHDRAW_AND_REDEEM_IX_ACCOUNTS_LEN] {
    fn from(accounts: RepayAndWithdrawAndRedeemAccounts<'_, 'info>) -> Self {
        [
            accounts.repay_accounts_owner.clone(),
            accounts.repay_accounts_obligation.clone(),
            accounts.repay_accounts_lending_market.clone(),
            accounts.repay_accounts_repay_reserve.clone(),
            accounts.repay_accounts_reserve_liquidity_mint.clone(),
            accounts.repay_accounts_reserve_destination_liquidity.clone(),
            accounts.repay_accounts_user_source_liquidity.clone(),
            accounts.repay_accounts_token_program.clone(),
            accounts.repay_accounts_instruction_sysvar_account.clone(),
            accounts.withdraw_accounts_owner.clone(),
            accounts.withdraw_accounts_obligation.clone(),
            accounts.withdraw_accounts_lending_market.clone(),
            accounts.withdraw_accounts_lending_market_authority.clone(),
            accounts.withdraw_accounts_withdraw_reserve.clone(),
            accounts.withdraw_accounts_reserve_liquidity_mint.clone(),
            accounts.withdraw_accounts_reserve_source_collateral.clone(),
            accounts.withdraw_accounts_reserve_collateral_mint.clone(),
            accounts.withdraw_accounts_reserve_liquidity_supply.clone(),
            accounts.withdraw_accounts_user_destination_liquidity.clone(),
            accounts.withdraw_accounts_placeholder_user_destination_collateral.clone(),
            accounts.withdraw_accounts_collateral_token_program.clone(),
            accounts.withdraw_accounts_liquidity_token_program.clone(),
            accounts.withdraw_accounts_instruction_sysvar_account.clone(),
            accounts.collateral_farms_accounts_obligation_farm_user_state.clone(),
            accounts.collateral_farms_accounts_reserve_farm_state.clone(),
            accounts.debt_farms_accounts_obligation_farm_user_state.clone(),
            accounts.debt_farms_accounts_reserve_farm_state.clone(),
            accounts.farms_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; REPAY_AND_WITHDRAW_AND_REDEEM_IX_ACCOUNTS_LEN]>
for RepayAndWithdrawAndRedeemAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REPAY_AND_WITHDRAW_AND_REDEEM_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            repay_accounts_owner: &arr[0],
            repay_accounts_obligation: &arr[1],
            repay_accounts_lending_market: &arr[2],
            repay_accounts_repay_reserve: &arr[3],
            repay_accounts_reserve_liquidity_mint: &arr[4],
            repay_accounts_reserve_destination_liquidity: &arr[5],
            repay_accounts_user_source_liquidity: &arr[6],
            repay_accounts_token_program: &arr[7],
            repay_accounts_instruction_sysvar_account: &arr[8],
            withdraw_accounts_owner: &arr[9],
            withdraw_accounts_obligation: &arr[10],
            withdraw_accounts_lending_market: &arr[11],
            withdraw_accounts_lending_market_authority: &arr[12],
            withdraw_accounts_withdraw_reserve: &arr[13],
            withdraw_accounts_reserve_liquidity_mint: &arr[14],
            withdraw_accounts_reserve_source_collateral: &arr[15],
            withdraw_accounts_reserve_collateral_mint: &arr[16],
            withdraw_accounts_reserve_liquidity_supply: &arr[17],
            withdraw_accounts_user_destination_liquidity: &arr[18],
            withdraw_accounts_placeholder_user_destination_collateral: &arr[19],
            withdraw_accounts_collateral_token_program: &arr[20],
            withdraw_accounts_liquidity_token_program: &arr[21],
            withdraw_accounts_instruction_sysvar_account: &arr[22],
            collateral_farms_accounts_obligation_farm_user_state: &arr[23],
            collateral_farms_accounts_reserve_farm_state: &arr[24],
            debt_farms_accounts_obligation_farm_user_state: &arr[25],
            debt_farms_accounts_reserve_farm_state: &arr[26],
            farms_program: &arr[27],
        }
    }
}
pub const REPAY_AND_WITHDRAW_AND_REDEEM_IX_DISCM: [u8; 8] = [
    2,
    54,
    152,
    3,
    148,
    96,
    109,
    218,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RepayAndWithdrawAndRedeemIxArgs {
    pub repay_amount: u64,
    pub withdraw_collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RepayAndWithdrawAndRedeemIxData(pub RepayAndWithdrawAndRedeemIxArgs);
impl From<RepayAndWithdrawAndRedeemIxArgs> for RepayAndWithdrawAndRedeemIxData {
    fn from(args: RepayAndWithdrawAndRedeemIxArgs) -> Self {
        Self(args)
    }
}
impl RepayAndWithdrawAndRedeemIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REPAY_AND_WITHDRAW_AND_REDEEM_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REPAY_AND_WITHDRAW_AND_REDEEM_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RepayAndWithdrawAndRedeemIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REPAY_AND_WITHDRAW_AND_REDEEM_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn repay_and_withdraw_and_redeem_ix_with_program_id(
    program_id: Pubkey,
    keys: RepayAndWithdrawAndRedeemKeys,
    args: RepayAndWithdrawAndRedeemIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REPAY_AND_WITHDRAW_AND_REDEEM_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: RepayAndWithdrawAndRedeemIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn repay_and_withdraw_and_redeem_ix(
    keys: RepayAndWithdrawAndRedeemKeys,
    args: RepayAndWithdrawAndRedeemIxArgs,
) -> std::io::Result<Instruction> {
    repay_and_withdraw_and_redeem_ix_with_program_id(crate::ID, keys, args)
}
pub fn repay_and_withdraw_and_redeem_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RepayAndWithdrawAndRedeemAccounts<'_, '_>,
    args: RepayAndWithdrawAndRedeemIxArgs,
) -> ProgramResult {
    let keys: RepayAndWithdrawAndRedeemKeys = accounts.into();
    let ix = repay_and_withdraw_and_redeem_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn repay_and_withdraw_and_redeem_invoke(
    accounts: RepayAndWithdrawAndRedeemAccounts<'_, '_>,
    args: RepayAndWithdrawAndRedeemIxArgs,
) -> ProgramResult {
    repay_and_withdraw_and_redeem_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn repay_and_withdraw_and_redeem_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RepayAndWithdrawAndRedeemAccounts<'_, '_>,
    args: RepayAndWithdrawAndRedeemIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RepayAndWithdrawAndRedeemKeys = accounts.into();
    let ix = repay_and_withdraw_and_redeem_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn repay_and_withdraw_and_redeem_invoke_signed(
    accounts: RepayAndWithdrawAndRedeemAccounts<'_, '_>,
    args: RepayAndWithdrawAndRedeemIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    repay_and_withdraw_and_redeem_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn repay_and_withdraw_and_redeem_verify_account_keys(
    accounts: RepayAndWithdrawAndRedeemAccounts<'_, '_>,
    keys: RepayAndWithdrawAndRedeemKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.repay_accounts_owner.key, keys.repay_accounts_owner),
        (*accounts.repay_accounts_obligation.key, keys.repay_accounts_obligation),
        (
            *accounts.repay_accounts_lending_market.key,
            keys.repay_accounts_lending_market,
        ),
        (*accounts.repay_accounts_repay_reserve.key, keys.repay_accounts_repay_reserve),
        (
            *accounts.repay_accounts_reserve_liquidity_mint.key,
            keys.repay_accounts_reserve_liquidity_mint,
        ),
        (
            *accounts.repay_accounts_reserve_destination_liquidity.key,
            keys.repay_accounts_reserve_destination_liquidity,
        ),
        (
            *accounts.repay_accounts_user_source_liquidity.key,
            keys.repay_accounts_user_source_liquidity,
        ),
        (*accounts.repay_accounts_token_program.key, keys.repay_accounts_token_program),
        (
            *accounts.repay_accounts_instruction_sysvar_account.key,
            keys.repay_accounts_instruction_sysvar_account,
        ),
        (*accounts.withdraw_accounts_owner.key, keys.withdraw_accounts_owner),
        (*accounts.withdraw_accounts_obligation.key, keys.withdraw_accounts_obligation),
        (
            *accounts.withdraw_accounts_lending_market.key,
            keys.withdraw_accounts_lending_market,
        ),
        (
            *accounts.withdraw_accounts_lending_market_authority.key,
            keys.withdraw_accounts_lending_market_authority,
        ),
        (
            *accounts.withdraw_accounts_withdraw_reserve.key,
            keys.withdraw_accounts_withdraw_reserve,
        ),
        (
            *accounts.withdraw_accounts_reserve_liquidity_mint.key,
            keys.withdraw_accounts_reserve_liquidity_mint,
        ),
        (
            *accounts.withdraw_accounts_reserve_source_collateral.key,
            keys.withdraw_accounts_reserve_source_collateral,
        ),
        (
            *accounts.withdraw_accounts_reserve_collateral_mint.key,
            keys.withdraw_accounts_reserve_collateral_mint,
        ),
        (
            *accounts.withdraw_accounts_reserve_liquidity_supply.key,
            keys.withdraw_accounts_reserve_liquidity_supply,
        ),
        (
            *accounts.withdraw_accounts_user_destination_liquidity.key,
            keys.withdraw_accounts_user_destination_liquidity,
        ),
        (
            *accounts.withdraw_accounts_placeholder_user_destination_collateral.key,
            keys.withdraw_accounts_placeholder_user_destination_collateral,
        ),
        (
            *accounts.withdraw_accounts_collateral_token_program.key,
            keys.withdraw_accounts_collateral_token_program,
        ),
        (
            *accounts.withdraw_accounts_liquidity_token_program.key,
            keys.withdraw_accounts_liquidity_token_program,
        ),
        (
            *accounts.withdraw_accounts_instruction_sysvar_account.key,
            keys.withdraw_accounts_instruction_sysvar_account,
        ),
        (
            *accounts.collateral_farms_accounts_obligation_farm_user_state.key,
            keys.collateral_farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.collateral_farms_accounts_reserve_farm_state.key,
            keys.collateral_farms_accounts_reserve_farm_state,
        ),
        (
            *accounts.debt_farms_accounts_obligation_farm_user_state.key,
            keys.debt_farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.debt_farms_accounts_reserve_farm_state.key,
            keys.debt_farms_accounts_reserve_farm_state,
        ),
        (*accounts.farms_program.key, keys.farms_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn repay_and_withdraw_and_redeem_verify_writable_privileges<'me, 'info>(
    accounts: RepayAndWithdrawAndRedeemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.repay_accounts_obligation,
        accounts.repay_accounts_repay_reserve,
        accounts.repay_accounts_reserve_destination_liquidity,
        accounts.repay_accounts_user_source_liquidity,
        accounts.withdraw_accounts_owner,
        accounts.withdraw_accounts_obligation,
        accounts.withdraw_accounts_withdraw_reserve,
        accounts.withdraw_accounts_reserve_source_collateral,
        accounts.withdraw_accounts_reserve_collateral_mint,
        accounts.withdraw_accounts_reserve_liquidity_supply,
        accounts.withdraw_accounts_user_destination_liquidity,
        accounts.collateral_farms_accounts_obligation_farm_user_state,
        accounts.collateral_farms_accounts_reserve_farm_state,
        accounts.debt_farms_accounts_obligation_farm_user_state,
        accounts.debt_farms_accounts_reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn repay_and_withdraw_and_redeem_verify_signer_privileges<'me, 'info>(
    accounts: RepayAndWithdrawAndRedeemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.repay_accounts_owner,
        accounts.withdraw_accounts_owner,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn repay_and_withdraw_and_redeem_verify_account_privileges<'me, 'info>(
    accounts: RepayAndWithdrawAndRedeemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    repay_and_withdraw_and_redeem_verify_writable_privileges(accounts)?;
    repay_and_withdraw_and_redeem_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_AND_WITHDRAW_IX_ACCOUNTS_LEN: usize = 33;
#[derive(Copy, Clone, Debug)]
pub struct DepositAndWithdrawAccounts<'me, 'info> {
    pub deposit_accounts_owner: &'me AccountInfo<'info>,
    pub deposit_accounts_obligation: &'me AccountInfo<'info>,
    pub deposit_accounts_lending_market: &'me AccountInfo<'info>,
    pub deposit_accounts_lending_market_authority: &'me AccountInfo<'info>,
    pub deposit_accounts_reserve: &'me AccountInfo<'info>,
    pub deposit_accounts_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub deposit_accounts_reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub deposit_accounts_reserve_collateral_mint: &'me AccountInfo<'info>,
    pub deposit_accounts_reserve_destination_deposit_collateral: &'me AccountInfo<'info>,
    pub deposit_accounts_user_source_liquidity: &'me AccountInfo<'info>,
    pub deposit_accounts_placeholder_user_destination_collateral: &'me AccountInfo<
        'info,
    >,
    pub deposit_accounts_collateral_token_program: &'me AccountInfo<'info>,
    pub deposit_accounts_liquidity_token_program: &'me AccountInfo<'info>,
    pub deposit_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub withdraw_accounts_owner: &'me AccountInfo<'info>,
    pub withdraw_accounts_obligation: &'me AccountInfo<'info>,
    pub withdraw_accounts_lending_market: &'me AccountInfo<'info>,
    pub withdraw_accounts_lending_market_authority: &'me AccountInfo<'info>,
    pub withdraw_accounts_withdraw_reserve: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_source_collateral: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_collateral_mint: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub withdraw_accounts_user_destination_liquidity: &'me AccountInfo<'info>,
    pub withdraw_accounts_placeholder_user_destination_collateral: &'me AccountInfo<
        'info,
    >,
    pub withdraw_accounts_collateral_token_program: &'me AccountInfo<'info>,
    pub withdraw_accounts_liquidity_token_program: &'me AccountInfo<'info>,
    pub withdraw_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub deposit_farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub deposit_farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub withdraw_farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub withdraw_farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositAndWithdrawKeys {
    pub deposit_accounts_owner: Pubkey,
    pub deposit_accounts_obligation: Pubkey,
    pub deposit_accounts_lending_market: Pubkey,
    pub deposit_accounts_lending_market_authority: Pubkey,
    pub deposit_accounts_reserve: Pubkey,
    pub deposit_accounts_reserve_liquidity_mint: Pubkey,
    pub deposit_accounts_reserve_liquidity_supply: Pubkey,
    pub deposit_accounts_reserve_collateral_mint: Pubkey,
    pub deposit_accounts_reserve_destination_deposit_collateral: Pubkey,
    pub deposit_accounts_user_source_liquidity: Pubkey,
    pub deposit_accounts_placeholder_user_destination_collateral: Pubkey,
    pub deposit_accounts_collateral_token_program: Pubkey,
    pub deposit_accounts_liquidity_token_program: Pubkey,
    pub deposit_accounts_instruction_sysvar_account: Pubkey,
    pub withdraw_accounts_owner: Pubkey,
    pub withdraw_accounts_obligation: Pubkey,
    pub withdraw_accounts_lending_market: Pubkey,
    pub withdraw_accounts_lending_market_authority: Pubkey,
    pub withdraw_accounts_withdraw_reserve: Pubkey,
    pub withdraw_accounts_reserve_liquidity_mint: Pubkey,
    pub withdraw_accounts_reserve_source_collateral: Pubkey,
    pub withdraw_accounts_reserve_collateral_mint: Pubkey,
    pub withdraw_accounts_reserve_liquidity_supply: Pubkey,
    pub withdraw_accounts_user_destination_liquidity: Pubkey,
    pub withdraw_accounts_placeholder_user_destination_collateral: Pubkey,
    pub withdraw_accounts_collateral_token_program: Pubkey,
    pub withdraw_accounts_liquidity_token_program: Pubkey,
    pub withdraw_accounts_instruction_sysvar_account: Pubkey,
    pub deposit_farms_accounts_obligation_farm_user_state: Pubkey,
    pub deposit_farms_accounts_reserve_farm_state: Pubkey,
    pub withdraw_farms_accounts_obligation_farm_user_state: Pubkey,
    pub withdraw_farms_accounts_reserve_farm_state: Pubkey,
    pub farms_program: Pubkey,
}
impl From<DepositAndWithdrawAccounts<'_, '_>> for DepositAndWithdrawKeys {
    fn from(accounts: DepositAndWithdrawAccounts) -> Self {
        Self {
            deposit_accounts_owner: *accounts.deposit_accounts_owner.key,
            deposit_accounts_obligation: *accounts.deposit_accounts_obligation.key,
            deposit_accounts_lending_market: *accounts
                .deposit_accounts_lending_market
                .key,
            deposit_accounts_lending_market_authority: *accounts
                .deposit_accounts_lending_market_authority
                .key,
            deposit_accounts_reserve: *accounts.deposit_accounts_reserve.key,
            deposit_accounts_reserve_liquidity_mint: *accounts
                .deposit_accounts_reserve_liquidity_mint
                .key,
            deposit_accounts_reserve_liquidity_supply: *accounts
                .deposit_accounts_reserve_liquidity_supply
                .key,
            deposit_accounts_reserve_collateral_mint: *accounts
                .deposit_accounts_reserve_collateral_mint
                .key,
            deposit_accounts_reserve_destination_deposit_collateral: *accounts
                .deposit_accounts_reserve_destination_deposit_collateral
                .key,
            deposit_accounts_user_source_liquidity: *accounts
                .deposit_accounts_user_source_liquidity
                .key,
            deposit_accounts_placeholder_user_destination_collateral: *accounts
                .deposit_accounts_placeholder_user_destination_collateral
                .key,
            deposit_accounts_collateral_token_program: *accounts
                .deposit_accounts_collateral_token_program
                .key,
            deposit_accounts_liquidity_token_program: *accounts
                .deposit_accounts_liquidity_token_program
                .key,
            deposit_accounts_instruction_sysvar_account: *accounts
                .deposit_accounts_instruction_sysvar_account
                .key,
            withdraw_accounts_owner: *accounts.withdraw_accounts_owner.key,
            withdraw_accounts_obligation: *accounts.withdraw_accounts_obligation.key,
            withdraw_accounts_lending_market: *accounts
                .withdraw_accounts_lending_market
                .key,
            withdraw_accounts_lending_market_authority: *accounts
                .withdraw_accounts_lending_market_authority
                .key,
            withdraw_accounts_withdraw_reserve: *accounts
                .withdraw_accounts_withdraw_reserve
                .key,
            withdraw_accounts_reserve_liquidity_mint: *accounts
                .withdraw_accounts_reserve_liquidity_mint
                .key,
            withdraw_accounts_reserve_source_collateral: *accounts
                .withdraw_accounts_reserve_source_collateral
                .key,
            withdraw_accounts_reserve_collateral_mint: *accounts
                .withdraw_accounts_reserve_collateral_mint
                .key,
            withdraw_accounts_reserve_liquidity_supply: *accounts
                .withdraw_accounts_reserve_liquidity_supply
                .key,
            withdraw_accounts_user_destination_liquidity: *accounts
                .withdraw_accounts_user_destination_liquidity
                .key,
            withdraw_accounts_placeholder_user_destination_collateral: *accounts
                .withdraw_accounts_placeholder_user_destination_collateral
                .key,
            withdraw_accounts_collateral_token_program: *accounts
                .withdraw_accounts_collateral_token_program
                .key,
            withdraw_accounts_liquidity_token_program: *accounts
                .withdraw_accounts_liquidity_token_program
                .key,
            withdraw_accounts_instruction_sysvar_account: *accounts
                .withdraw_accounts_instruction_sysvar_account
                .key,
            deposit_farms_accounts_obligation_farm_user_state: *accounts
                .deposit_farms_accounts_obligation_farm_user_state
                .key,
            deposit_farms_accounts_reserve_farm_state: *accounts
                .deposit_farms_accounts_reserve_farm_state
                .key,
            withdraw_farms_accounts_obligation_farm_user_state: *accounts
                .withdraw_farms_accounts_obligation_farm_user_state
                .key,
            withdraw_farms_accounts_reserve_farm_state: *accounts
                .withdraw_farms_accounts_reserve_farm_state
                .key,
            farms_program: *accounts.farms_program.key,
        }
    }
}
impl From<DepositAndWithdrawKeys>
for [AccountMeta; DEPOSIT_AND_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositAndWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.deposit_accounts_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_reserve_destination_deposit_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_user_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_placeholder_user_destination_collateral,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_withdraw_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_source_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_user_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_placeholder_user_destination_collateral,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_AND_WITHDRAW_IX_ACCOUNTS_LEN]> for DepositAndWithdrawKeys {
    fn from(pubkeys: [Pubkey; DEPOSIT_AND_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            deposit_accounts_owner: pubkeys[0],
            deposit_accounts_obligation: pubkeys[1],
            deposit_accounts_lending_market: pubkeys[2],
            deposit_accounts_lending_market_authority: pubkeys[3],
            deposit_accounts_reserve: pubkeys[4],
            deposit_accounts_reserve_liquidity_mint: pubkeys[5],
            deposit_accounts_reserve_liquidity_supply: pubkeys[6],
            deposit_accounts_reserve_collateral_mint: pubkeys[7],
            deposit_accounts_reserve_destination_deposit_collateral: pubkeys[8],
            deposit_accounts_user_source_liquidity: pubkeys[9],
            deposit_accounts_placeholder_user_destination_collateral: pubkeys[10],
            deposit_accounts_collateral_token_program: pubkeys[11],
            deposit_accounts_liquidity_token_program: pubkeys[12],
            deposit_accounts_instruction_sysvar_account: pubkeys[13],
            withdraw_accounts_owner: pubkeys[14],
            withdraw_accounts_obligation: pubkeys[15],
            withdraw_accounts_lending_market: pubkeys[16],
            withdraw_accounts_lending_market_authority: pubkeys[17],
            withdraw_accounts_withdraw_reserve: pubkeys[18],
            withdraw_accounts_reserve_liquidity_mint: pubkeys[19],
            withdraw_accounts_reserve_source_collateral: pubkeys[20],
            withdraw_accounts_reserve_collateral_mint: pubkeys[21],
            withdraw_accounts_reserve_liquidity_supply: pubkeys[22],
            withdraw_accounts_user_destination_liquidity: pubkeys[23],
            withdraw_accounts_placeholder_user_destination_collateral: pubkeys[24],
            withdraw_accounts_collateral_token_program: pubkeys[25],
            withdraw_accounts_liquidity_token_program: pubkeys[26],
            withdraw_accounts_instruction_sysvar_account: pubkeys[27],
            deposit_farms_accounts_obligation_farm_user_state: pubkeys[28],
            deposit_farms_accounts_reserve_farm_state: pubkeys[29],
            withdraw_farms_accounts_obligation_farm_user_state: pubkeys[30],
            withdraw_farms_accounts_reserve_farm_state: pubkeys[31],
            farms_program: pubkeys[32],
        }
    }
}
impl<'info> From<DepositAndWithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; DEPOSIT_AND_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: DepositAndWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.deposit_accounts_owner.clone(),
            accounts.deposit_accounts_obligation.clone(),
            accounts.deposit_accounts_lending_market.clone(),
            accounts.deposit_accounts_lending_market_authority.clone(),
            accounts.deposit_accounts_reserve.clone(),
            accounts.deposit_accounts_reserve_liquidity_mint.clone(),
            accounts.deposit_accounts_reserve_liquidity_supply.clone(),
            accounts.deposit_accounts_reserve_collateral_mint.clone(),
            accounts.deposit_accounts_reserve_destination_deposit_collateral.clone(),
            accounts.deposit_accounts_user_source_liquidity.clone(),
            accounts.deposit_accounts_placeholder_user_destination_collateral.clone(),
            accounts.deposit_accounts_collateral_token_program.clone(),
            accounts.deposit_accounts_liquidity_token_program.clone(),
            accounts.deposit_accounts_instruction_sysvar_account.clone(),
            accounts.withdraw_accounts_owner.clone(),
            accounts.withdraw_accounts_obligation.clone(),
            accounts.withdraw_accounts_lending_market.clone(),
            accounts.withdraw_accounts_lending_market_authority.clone(),
            accounts.withdraw_accounts_withdraw_reserve.clone(),
            accounts.withdraw_accounts_reserve_liquidity_mint.clone(),
            accounts.withdraw_accounts_reserve_source_collateral.clone(),
            accounts.withdraw_accounts_reserve_collateral_mint.clone(),
            accounts.withdraw_accounts_reserve_liquidity_supply.clone(),
            accounts.withdraw_accounts_user_destination_liquidity.clone(),
            accounts.withdraw_accounts_placeholder_user_destination_collateral.clone(),
            accounts.withdraw_accounts_collateral_token_program.clone(),
            accounts.withdraw_accounts_liquidity_token_program.clone(),
            accounts.withdraw_accounts_instruction_sysvar_account.clone(),
            accounts.deposit_farms_accounts_obligation_farm_user_state.clone(),
            accounts.deposit_farms_accounts_reserve_farm_state.clone(),
            accounts.withdraw_farms_accounts_obligation_farm_user_state.clone(),
            accounts.withdraw_farms_accounts_reserve_farm_state.clone(),
            accounts.farms_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPOSIT_AND_WITHDRAW_IX_ACCOUNTS_LEN]>
for DepositAndWithdrawAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; DEPOSIT_AND_WITHDRAW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            deposit_accounts_owner: &arr[0],
            deposit_accounts_obligation: &arr[1],
            deposit_accounts_lending_market: &arr[2],
            deposit_accounts_lending_market_authority: &arr[3],
            deposit_accounts_reserve: &arr[4],
            deposit_accounts_reserve_liquidity_mint: &arr[5],
            deposit_accounts_reserve_liquidity_supply: &arr[6],
            deposit_accounts_reserve_collateral_mint: &arr[7],
            deposit_accounts_reserve_destination_deposit_collateral: &arr[8],
            deposit_accounts_user_source_liquidity: &arr[9],
            deposit_accounts_placeholder_user_destination_collateral: &arr[10],
            deposit_accounts_collateral_token_program: &arr[11],
            deposit_accounts_liquidity_token_program: &arr[12],
            deposit_accounts_instruction_sysvar_account: &arr[13],
            withdraw_accounts_owner: &arr[14],
            withdraw_accounts_obligation: &arr[15],
            withdraw_accounts_lending_market: &arr[16],
            withdraw_accounts_lending_market_authority: &arr[17],
            withdraw_accounts_withdraw_reserve: &arr[18],
            withdraw_accounts_reserve_liquidity_mint: &arr[19],
            withdraw_accounts_reserve_source_collateral: &arr[20],
            withdraw_accounts_reserve_collateral_mint: &arr[21],
            withdraw_accounts_reserve_liquidity_supply: &arr[22],
            withdraw_accounts_user_destination_liquidity: &arr[23],
            withdraw_accounts_placeholder_user_destination_collateral: &arr[24],
            withdraw_accounts_collateral_token_program: &arr[25],
            withdraw_accounts_liquidity_token_program: &arr[26],
            withdraw_accounts_instruction_sysvar_account: &arr[27],
            deposit_farms_accounts_obligation_farm_user_state: &arr[28],
            deposit_farms_accounts_reserve_farm_state: &arr[29],
            withdraw_farms_accounts_obligation_farm_user_state: &arr[30],
            withdraw_farms_accounts_reserve_farm_state: &arr[31],
            farms_program: &arr[32],
        }
    }
}
pub const DEPOSIT_AND_WITHDRAW_IX_DISCM: [u8; 8] = [141, 153, 39, 15, 64, 61, 88, 84];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositAndWithdrawIxArgs {
    pub liquidity_amount: u64,
    pub withdraw_collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositAndWithdrawIxData(pub DepositAndWithdrawIxArgs);
impl From<DepositAndWithdrawIxArgs> for DepositAndWithdrawIxData {
    fn from(args: DepositAndWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl DepositAndWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_AND_WITHDRAW_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DEPOSIT_AND_WITHDRAW_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(DepositAndWithdrawIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_AND_WITHDRAW_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_and_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositAndWithdrawKeys,
    args: DepositAndWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_AND_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: DepositAndWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_and_withdraw_ix(
    keys: DepositAndWithdrawKeys,
    args: DepositAndWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    deposit_and_withdraw_ix_with_program_id(crate::ID, keys, args)
}
pub fn deposit_and_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositAndWithdrawAccounts<'_, '_>,
    args: DepositAndWithdrawIxArgs,
) -> ProgramResult {
    let keys: DepositAndWithdrawKeys = accounts.into();
    let ix = deposit_and_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_and_withdraw_invoke(
    accounts: DepositAndWithdrawAccounts<'_, '_>,
    args: DepositAndWithdrawIxArgs,
) -> ProgramResult {
    deposit_and_withdraw_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn deposit_and_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositAndWithdrawAccounts<'_, '_>,
    args: DepositAndWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositAndWithdrawKeys = accounts.into();
    let ix = deposit_and_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_and_withdraw_invoke_signed(
    accounts: DepositAndWithdrawAccounts<'_, '_>,
    args: DepositAndWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_and_withdraw_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn deposit_and_withdraw_verify_account_keys(
    accounts: DepositAndWithdrawAccounts<'_, '_>,
    keys: DepositAndWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.deposit_accounts_owner.key, keys.deposit_accounts_owner),
        (*accounts.deposit_accounts_obligation.key, keys.deposit_accounts_obligation),
        (
            *accounts.deposit_accounts_lending_market.key,
            keys.deposit_accounts_lending_market,
        ),
        (
            *accounts.deposit_accounts_lending_market_authority.key,
            keys.deposit_accounts_lending_market_authority,
        ),
        (*accounts.deposit_accounts_reserve.key, keys.deposit_accounts_reserve),
        (
            *accounts.deposit_accounts_reserve_liquidity_mint.key,
            keys.deposit_accounts_reserve_liquidity_mint,
        ),
        (
            *accounts.deposit_accounts_reserve_liquidity_supply.key,
            keys.deposit_accounts_reserve_liquidity_supply,
        ),
        (
            *accounts.deposit_accounts_reserve_collateral_mint.key,
            keys.deposit_accounts_reserve_collateral_mint,
        ),
        (
            *accounts.deposit_accounts_reserve_destination_deposit_collateral.key,
            keys.deposit_accounts_reserve_destination_deposit_collateral,
        ),
        (
            *accounts.deposit_accounts_user_source_liquidity.key,
            keys.deposit_accounts_user_source_liquidity,
        ),
        (
            *accounts.deposit_accounts_placeholder_user_destination_collateral.key,
            keys.deposit_accounts_placeholder_user_destination_collateral,
        ),
        (
            *accounts.deposit_accounts_collateral_token_program.key,
            keys.deposit_accounts_collateral_token_program,
        ),
        (
            *accounts.deposit_accounts_liquidity_token_program.key,
            keys.deposit_accounts_liquidity_token_program,
        ),
        (
            *accounts.deposit_accounts_instruction_sysvar_account.key,
            keys.deposit_accounts_instruction_sysvar_account,
        ),
        (*accounts.withdraw_accounts_owner.key, keys.withdraw_accounts_owner),
        (*accounts.withdraw_accounts_obligation.key, keys.withdraw_accounts_obligation),
        (
            *accounts.withdraw_accounts_lending_market.key,
            keys.withdraw_accounts_lending_market,
        ),
        (
            *accounts.withdraw_accounts_lending_market_authority.key,
            keys.withdraw_accounts_lending_market_authority,
        ),
        (
            *accounts.withdraw_accounts_withdraw_reserve.key,
            keys.withdraw_accounts_withdraw_reserve,
        ),
        (
            *accounts.withdraw_accounts_reserve_liquidity_mint.key,
            keys.withdraw_accounts_reserve_liquidity_mint,
        ),
        (
            *accounts.withdraw_accounts_reserve_source_collateral.key,
            keys.withdraw_accounts_reserve_source_collateral,
        ),
        (
            *accounts.withdraw_accounts_reserve_collateral_mint.key,
            keys.withdraw_accounts_reserve_collateral_mint,
        ),
        (
            *accounts.withdraw_accounts_reserve_liquidity_supply.key,
            keys.withdraw_accounts_reserve_liquidity_supply,
        ),
        (
            *accounts.withdraw_accounts_user_destination_liquidity.key,
            keys.withdraw_accounts_user_destination_liquidity,
        ),
        (
            *accounts.withdraw_accounts_placeholder_user_destination_collateral.key,
            keys.withdraw_accounts_placeholder_user_destination_collateral,
        ),
        (
            *accounts.withdraw_accounts_collateral_token_program.key,
            keys.withdraw_accounts_collateral_token_program,
        ),
        (
            *accounts.withdraw_accounts_liquidity_token_program.key,
            keys.withdraw_accounts_liquidity_token_program,
        ),
        (
            *accounts.withdraw_accounts_instruction_sysvar_account.key,
            keys.withdraw_accounts_instruction_sysvar_account,
        ),
        (
            *accounts.deposit_farms_accounts_obligation_farm_user_state.key,
            keys.deposit_farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.deposit_farms_accounts_reserve_farm_state.key,
            keys.deposit_farms_accounts_reserve_farm_state,
        ),
        (
            *accounts.withdraw_farms_accounts_obligation_farm_user_state.key,
            keys.withdraw_farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.withdraw_farms_accounts_reserve_farm_state.key,
            keys.withdraw_farms_accounts_reserve_farm_state,
        ),
        (*accounts.farms_program.key, keys.farms_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_and_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: DepositAndWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deposit_accounts_owner,
        accounts.deposit_accounts_obligation,
        accounts.deposit_accounts_reserve,
        accounts.deposit_accounts_reserve_liquidity_supply,
        accounts.deposit_accounts_reserve_collateral_mint,
        accounts.deposit_accounts_reserve_destination_deposit_collateral,
        accounts.deposit_accounts_user_source_liquidity,
        accounts.withdraw_accounts_owner,
        accounts.withdraw_accounts_obligation,
        accounts.withdraw_accounts_withdraw_reserve,
        accounts.withdraw_accounts_reserve_source_collateral,
        accounts.withdraw_accounts_reserve_collateral_mint,
        accounts.withdraw_accounts_reserve_liquidity_supply,
        accounts.withdraw_accounts_user_destination_liquidity,
        accounts.deposit_farms_accounts_obligation_farm_user_state,
        accounts.deposit_farms_accounts_reserve_farm_state,
        accounts.withdraw_farms_accounts_obligation_farm_user_state,
        accounts.withdraw_farms_accounts_reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_and_withdraw_verify_signer_privileges<'me, 'info>(
    accounts: DepositAndWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [
        accounts.deposit_accounts_owner,
        accounts.withdraw_accounts_owner,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_and_withdraw_verify_account_privileges<'me, 'info>(
    accounts: DepositAndWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_and_withdraw_verify_writable_privileges(accounts)?;
    deposit_and_withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct DepositReserveLiquidityAndObligationCollateralAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_destination_deposit_collateral: &'me AccountInfo<'info>,
    pub user_source_liquidity: &'me AccountInfo<'info>,
    pub placeholder_user_destination_collateral: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub liquidity_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositReserveLiquidityAndObligationCollateralKeys {
    pub owner: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_destination_deposit_collateral: Pubkey,
    pub user_source_liquidity: Pubkey,
    pub placeholder_user_destination_collateral: Pubkey,
    pub collateral_token_program: Pubkey,
    pub liquidity_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<DepositReserveLiquidityAndObligationCollateralAccounts<'_, '_>>
for DepositReserveLiquidityAndObligationCollateralKeys {
    fn from(accounts: DepositReserveLiquidityAndObligationCollateralAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            reserve: *accounts.reserve.key,
            reserve_liquidity_mint: *accounts.reserve_liquidity_mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_destination_deposit_collateral: *accounts
                .reserve_destination_deposit_collateral
                .key,
            user_source_liquidity: *accounts.user_source_liquidity.key,
            placeholder_user_destination_collateral: *accounts
                .placeholder_user_destination_collateral
                .key,
            collateral_token_program: *accounts.collateral_token_program.key,
            liquidity_token_program: *accounts.liquidity_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<DepositReserveLiquidityAndObligationCollateralKeys>
for [AccountMeta; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositReserveLiquidityAndObligationCollateralKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_destination_deposit_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.placeholder_user_destination_collateral,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN]>
for DepositReserveLiquidityAndObligationCollateralKeys {
    fn from(
        pubkeys: [Pubkey; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: pubkeys[0],
            obligation: pubkeys[1],
            lending_market: pubkeys[2],
            lending_market_authority: pubkeys[3],
            reserve: pubkeys[4],
            reserve_liquidity_mint: pubkeys[5],
            reserve_liquidity_supply: pubkeys[6],
            reserve_collateral_mint: pubkeys[7],
            reserve_destination_deposit_collateral: pubkeys[8],
            user_source_liquidity: pubkeys[9],
            placeholder_user_destination_collateral: pubkeys[10],
            collateral_token_program: pubkeys[11],
            liquidity_token_program: pubkeys[12],
            instruction_sysvar_account: pubkeys[13],
        }
    }
}
impl<'info> From<DepositReserveLiquidityAndObligationCollateralAccounts<'_, 'info>>
for [AccountInfo<
    'info,
>; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: DepositReserveLiquidityAndObligationCollateralAccounts<'_, 'info>,
    ) -> Self {
        [
            accounts.owner.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.reserve.clone(),
            accounts.reserve_liquidity_mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_destination_deposit_collateral.clone(),
            accounts.user_source_liquidity.clone(),
            accounts.placeholder_user_destination_collateral.clone(),
            accounts.collateral_token_program.clone(),
            accounts.liquidity_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN],
> for DepositReserveLiquidityAndObligationCollateralAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            obligation: &arr[1],
            lending_market: &arr[2],
            lending_market_authority: &arr[3],
            reserve: &arr[4],
            reserve_liquidity_mint: &arr[5],
            reserve_liquidity_supply: &arr[6],
            reserve_collateral_mint: &arr[7],
            reserve_destination_deposit_collateral: &arr[8],
            user_source_liquidity: &arr[9],
            placeholder_user_destination_collateral: &arr[10],
            collateral_token_program: &arr[11],
            liquidity_token_program: &arr[12],
            instruction_sysvar_account: &arr[13],
        }
    }
}
pub const DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_DISCM: [u8; 8] = [
    129,
    199,
    4,
    2,
    222,
    39,
    26,
    46,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositReserveLiquidityAndObligationCollateralIxArgs {
    pub liquidity_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositReserveLiquidityAndObligationCollateralIxData(
    pub DepositReserveLiquidityAndObligationCollateralIxArgs,
);
impl From<DepositReserveLiquidityAndObligationCollateralIxArgs>
for DepositReserveLiquidityAndObligationCollateralIxData {
    fn from(args: DepositReserveLiquidityAndObligationCollateralIxArgs) -> Self {
        Self(args)
    }
}
impl DepositReserveLiquidityAndObligationCollateralIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_DISCM,
                        maybe_discm
                    ),
                ),
            );
        }
        Ok(
            Self(
                DepositReserveLiquidityAndObligationCollateralIxArgs::deserialize(
                    &mut reader,
                )?,
            ),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositReserveLiquidityAndObligationCollateralKeys,
    args: DepositReserveLiquidityAndObligationCollateralIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DepositReserveLiquidityAndObligationCollateralIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_ix(
    keys: DepositReserveLiquidityAndObligationCollateralKeys,
    args: DepositReserveLiquidityAndObligationCollateralIxArgs,
) -> std::io::Result<Instruction> {
    deposit_reserve_liquidity_and_obligation_collateral_ix_with_program_id(
        crate::ID,
        keys,
        args,
    )
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositReserveLiquidityAndObligationCollateralAccounts<'_, '_>,
    args: DepositReserveLiquidityAndObligationCollateralIxArgs,
) -> ProgramResult {
    let keys: DepositReserveLiquidityAndObligationCollateralKeys = accounts.into();
    let ix = deposit_reserve_liquidity_and_obligation_collateral_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_invoke(
    accounts: DepositReserveLiquidityAndObligationCollateralAccounts<'_, '_>,
    args: DepositReserveLiquidityAndObligationCollateralIxArgs,
) -> ProgramResult {
    deposit_reserve_liquidity_and_obligation_collateral_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositReserveLiquidityAndObligationCollateralAccounts<'_, '_>,
    args: DepositReserveLiquidityAndObligationCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositReserveLiquidityAndObligationCollateralKeys = accounts.into();
    let ix = deposit_reserve_liquidity_and_obligation_collateral_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_invoke_signed(
    accounts: DepositReserveLiquidityAndObligationCollateralAccounts<'_, '_>,
    args: DepositReserveLiquidityAndObligationCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_reserve_liquidity_and_obligation_collateral_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_verify_account_keys(
    accounts: DepositReserveLiquidityAndObligationCollateralAccounts<'_, '_>,
    keys: DepositReserveLiquidityAndObligationCollateralKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_liquidity_mint.key, keys.reserve_liquidity_mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (
            *accounts.reserve_destination_deposit_collateral.key,
            keys.reserve_destination_deposit_collateral,
        ),
        (*accounts.user_source_liquidity.key, keys.user_source_liquidity),
        (
            *accounts.placeholder_user_destination_collateral.key,
            keys.placeholder_user_destination_collateral,
        ),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (*accounts.liquidity_token_program.key, keys.liquidity_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: DepositReserveLiquidityAndObligationCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.obligation,
        accounts.reserve,
        accounts.reserve_liquidity_supply,
        accounts.reserve_collateral_mint,
        accounts.reserve_destination_deposit_collateral,
        accounts.user_source_liquidity,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: DepositReserveLiquidityAndObligationCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_verify_account_privileges<
    'me,
    'info,
>(
    accounts: DepositReserveLiquidityAndObligationCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_reserve_liquidity_and_obligation_collateral_verify_writable_privileges(
        accounts,
    )?;
    deposit_reserve_liquidity_and_obligation_collateral_verify_signer_privileges(
        accounts,
    )?;
    Ok(())
}
pub const DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct DepositReserveLiquidityAndObligationCollateralV2Accounts<'me, 'info> {
    pub deposit_accounts_owner: &'me AccountInfo<'info>,
    pub deposit_accounts_obligation: &'me AccountInfo<'info>,
    pub deposit_accounts_lending_market: &'me AccountInfo<'info>,
    pub deposit_accounts_lending_market_authority: &'me AccountInfo<'info>,
    pub deposit_accounts_reserve: &'me AccountInfo<'info>,
    pub deposit_accounts_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub deposit_accounts_reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub deposit_accounts_reserve_collateral_mint: &'me AccountInfo<'info>,
    pub deposit_accounts_reserve_destination_deposit_collateral: &'me AccountInfo<'info>,
    pub deposit_accounts_user_source_liquidity: &'me AccountInfo<'info>,
    pub deposit_accounts_placeholder_user_destination_collateral: &'me AccountInfo<
        'info,
    >,
    pub deposit_accounts_collateral_token_program: &'me AccountInfo<'info>,
    pub deposit_accounts_liquidity_token_program: &'me AccountInfo<'info>,
    pub deposit_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepositReserveLiquidityAndObligationCollateralV2Keys {
    pub deposit_accounts_owner: Pubkey,
    pub deposit_accounts_obligation: Pubkey,
    pub deposit_accounts_lending_market: Pubkey,
    pub deposit_accounts_lending_market_authority: Pubkey,
    pub deposit_accounts_reserve: Pubkey,
    pub deposit_accounts_reserve_liquidity_mint: Pubkey,
    pub deposit_accounts_reserve_liquidity_supply: Pubkey,
    pub deposit_accounts_reserve_collateral_mint: Pubkey,
    pub deposit_accounts_reserve_destination_deposit_collateral: Pubkey,
    pub deposit_accounts_user_source_liquidity: Pubkey,
    pub deposit_accounts_placeholder_user_destination_collateral: Pubkey,
    pub deposit_accounts_collateral_token_program: Pubkey,
    pub deposit_accounts_liquidity_token_program: Pubkey,
    pub deposit_accounts_instruction_sysvar_account: Pubkey,
    pub farms_accounts_obligation_farm_user_state: Pubkey,
    pub farms_accounts_reserve_farm_state: Pubkey,
    pub farms_program: Pubkey,
}
impl From<DepositReserveLiquidityAndObligationCollateralV2Accounts<'_, '_>>
for DepositReserveLiquidityAndObligationCollateralV2Keys {
    fn from(accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts) -> Self {
        Self {
            deposit_accounts_owner: *accounts.deposit_accounts_owner.key,
            deposit_accounts_obligation: *accounts.deposit_accounts_obligation.key,
            deposit_accounts_lending_market: *accounts
                .deposit_accounts_lending_market
                .key,
            deposit_accounts_lending_market_authority: *accounts
                .deposit_accounts_lending_market_authority
                .key,
            deposit_accounts_reserve: *accounts.deposit_accounts_reserve.key,
            deposit_accounts_reserve_liquidity_mint: *accounts
                .deposit_accounts_reserve_liquidity_mint
                .key,
            deposit_accounts_reserve_liquidity_supply: *accounts
                .deposit_accounts_reserve_liquidity_supply
                .key,
            deposit_accounts_reserve_collateral_mint: *accounts
                .deposit_accounts_reserve_collateral_mint
                .key,
            deposit_accounts_reserve_destination_deposit_collateral: *accounts
                .deposit_accounts_reserve_destination_deposit_collateral
                .key,
            deposit_accounts_user_source_liquidity: *accounts
                .deposit_accounts_user_source_liquidity
                .key,
            deposit_accounts_placeholder_user_destination_collateral: *accounts
                .deposit_accounts_placeholder_user_destination_collateral
                .key,
            deposit_accounts_collateral_token_program: *accounts
                .deposit_accounts_collateral_token_program
                .key,
            deposit_accounts_liquidity_token_program: *accounts
                .deposit_accounts_liquidity_token_program
                .key,
            deposit_accounts_instruction_sysvar_account: *accounts
                .deposit_accounts_instruction_sysvar_account
                .key,
            farms_accounts_obligation_farm_user_state: *accounts
                .farms_accounts_obligation_farm_user_state
                .key,
            farms_accounts_reserve_farm_state: *accounts
                .farms_accounts_reserve_farm_state
                .key,
            farms_program: *accounts.farms_program.key,
        }
    }
}
impl From<DepositReserveLiquidityAndObligationCollateralV2Keys>
for [AccountMeta; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: DepositReserveLiquidityAndObligationCollateralV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.deposit_accounts_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_reserve_destination_deposit_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_user_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_placeholder_user_destination_collateral,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.deposit_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<
    [Pubkey; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN],
> for DepositReserveLiquidityAndObligationCollateralV2Keys {
    fn from(
        pubkeys: [Pubkey; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            deposit_accounts_owner: pubkeys[0],
            deposit_accounts_obligation: pubkeys[1],
            deposit_accounts_lending_market: pubkeys[2],
            deposit_accounts_lending_market_authority: pubkeys[3],
            deposit_accounts_reserve: pubkeys[4],
            deposit_accounts_reserve_liquidity_mint: pubkeys[5],
            deposit_accounts_reserve_liquidity_supply: pubkeys[6],
            deposit_accounts_reserve_collateral_mint: pubkeys[7],
            deposit_accounts_reserve_destination_deposit_collateral: pubkeys[8],
            deposit_accounts_user_source_liquidity: pubkeys[9],
            deposit_accounts_placeholder_user_destination_collateral: pubkeys[10],
            deposit_accounts_collateral_token_program: pubkeys[11],
            deposit_accounts_liquidity_token_program: pubkeys[12],
            deposit_accounts_instruction_sysvar_account: pubkeys[13],
            farms_accounts_obligation_farm_user_state: pubkeys[14],
            farms_accounts_reserve_farm_state: pubkeys[15],
            farms_program: pubkeys[16],
        }
    }
}
impl<'info> From<DepositReserveLiquidityAndObligationCollateralV2Accounts<'_, 'info>>
for [AccountInfo<
    'info,
>; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts<'_, 'info>,
    ) -> Self {
        [
            accounts.deposit_accounts_owner.clone(),
            accounts.deposit_accounts_obligation.clone(),
            accounts.deposit_accounts_lending_market.clone(),
            accounts.deposit_accounts_lending_market_authority.clone(),
            accounts.deposit_accounts_reserve.clone(),
            accounts.deposit_accounts_reserve_liquidity_mint.clone(),
            accounts.deposit_accounts_reserve_liquidity_supply.clone(),
            accounts.deposit_accounts_reserve_collateral_mint.clone(),
            accounts.deposit_accounts_reserve_destination_deposit_collateral.clone(),
            accounts.deposit_accounts_user_source_liquidity.clone(),
            accounts.deposit_accounts_placeholder_user_destination_collateral.clone(),
            accounts.deposit_accounts_collateral_token_program.clone(),
            accounts.deposit_accounts_liquidity_token_program.clone(),
            accounts.deposit_accounts_instruction_sysvar_account.clone(),
            accounts.farms_accounts_obligation_farm_user_state.clone(),
            accounts.farms_accounts_reserve_farm_state.clone(),
            accounts.farms_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN],
> for DepositReserveLiquidityAndObligationCollateralV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            deposit_accounts_owner: &arr[0],
            deposit_accounts_obligation: &arr[1],
            deposit_accounts_lending_market: &arr[2],
            deposit_accounts_lending_market_authority: &arr[3],
            deposit_accounts_reserve: &arr[4],
            deposit_accounts_reserve_liquidity_mint: &arr[5],
            deposit_accounts_reserve_liquidity_supply: &arr[6],
            deposit_accounts_reserve_collateral_mint: &arr[7],
            deposit_accounts_reserve_destination_deposit_collateral: &arr[8],
            deposit_accounts_user_source_liquidity: &arr[9],
            deposit_accounts_placeholder_user_destination_collateral: &arr[10],
            deposit_accounts_collateral_token_program: &arr[11],
            deposit_accounts_liquidity_token_program: &arr[12],
            deposit_accounts_instruction_sysvar_account: &arr[13],
            farms_accounts_obligation_farm_user_state: &arr[14],
            farms_accounts_reserve_farm_state: &arr[15],
            farms_program: &arr[16],
        }
    }
}
pub const DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_DISCM: [u8; 8] = [
    216,
    224,
    191,
    27,
    204,
    151,
    102,
    175,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositReserveLiquidityAndObligationCollateralV2IxArgs {
    pub liquidity_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositReserveLiquidityAndObligationCollateralV2IxData(
    pub DepositReserveLiquidityAndObligationCollateralV2IxArgs,
);
impl From<DepositReserveLiquidityAndObligationCollateralV2IxArgs>
for DepositReserveLiquidityAndObligationCollateralV2IxData {
    fn from(args: DepositReserveLiquidityAndObligationCollateralV2IxArgs) -> Self {
        Self(args)
    }
}
impl DepositReserveLiquidityAndObligationCollateralV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_DISCM
        {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_DISCM,
                        maybe_discm
                    ),
                ),
            );
        }
        Ok(
            Self(
                DepositReserveLiquidityAndObligationCollateralV2IxArgs::deserialize(
                    &mut reader,
                )?,
            ),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer
            .write_all(
                &DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_DISCM,
            )?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: DepositReserveLiquidityAndObligationCollateralV2Keys,
    args: DepositReserveLiquidityAndObligationCollateralV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DEPOSIT_RESERVE_LIQUIDITY_AND_OBLIGATION_COLLATERAL_V2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: DepositReserveLiquidityAndObligationCollateralV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_v2_ix(
    keys: DepositReserveLiquidityAndObligationCollateralV2Keys,
    args: DepositReserveLiquidityAndObligationCollateralV2IxArgs,
) -> std::io::Result<Instruction> {
    deposit_reserve_liquidity_and_obligation_collateral_v2_ix_with_program_id(
        crate::ID,
        keys,
        args,
    )
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts<'_, '_>,
    args: DepositReserveLiquidityAndObligationCollateralV2IxArgs,
) -> ProgramResult {
    let keys: DepositReserveLiquidityAndObligationCollateralV2Keys = accounts.into();
    let ix = deposit_reserve_liquidity_and_obligation_collateral_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_v2_invoke(
    accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts<'_, '_>,
    args: DepositReserveLiquidityAndObligationCollateralV2IxArgs,
) -> ProgramResult {
    deposit_reserve_liquidity_and_obligation_collateral_v2_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts<'_, '_>,
    args: DepositReserveLiquidityAndObligationCollateralV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DepositReserveLiquidityAndObligationCollateralV2Keys = accounts.into();
    let ix = deposit_reserve_liquidity_and_obligation_collateral_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_v2_invoke_signed(
    accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts<'_, '_>,
    args: DepositReserveLiquidityAndObligationCollateralV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deposit_reserve_liquidity_and_obligation_collateral_v2_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_v2_verify_account_keys(
    accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts<'_, '_>,
    keys: DepositReserveLiquidityAndObligationCollateralV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.deposit_accounts_owner.key, keys.deposit_accounts_owner),
        (*accounts.deposit_accounts_obligation.key, keys.deposit_accounts_obligation),
        (
            *accounts.deposit_accounts_lending_market.key,
            keys.deposit_accounts_lending_market,
        ),
        (
            *accounts.deposit_accounts_lending_market_authority.key,
            keys.deposit_accounts_lending_market_authority,
        ),
        (*accounts.deposit_accounts_reserve.key, keys.deposit_accounts_reserve),
        (
            *accounts.deposit_accounts_reserve_liquidity_mint.key,
            keys.deposit_accounts_reserve_liquidity_mint,
        ),
        (
            *accounts.deposit_accounts_reserve_liquidity_supply.key,
            keys.deposit_accounts_reserve_liquidity_supply,
        ),
        (
            *accounts.deposit_accounts_reserve_collateral_mint.key,
            keys.deposit_accounts_reserve_collateral_mint,
        ),
        (
            *accounts.deposit_accounts_reserve_destination_deposit_collateral.key,
            keys.deposit_accounts_reserve_destination_deposit_collateral,
        ),
        (
            *accounts.deposit_accounts_user_source_liquidity.key,
            keys.deposit_accounts_user_source_liquidity,
        ),
        (
            *accounts.deposit_accounts_placeholder_user_destination_collateral.key,
            keys.deposit_accounts_placeholder_user_destination_collateral,
        ),
        (
            *accounts.deposit_accounts_collateral_token_program.key,
            keys.deposit_accounts_collateral_token_program,
        ),
        (
            *accounts.deposit_accounts_liquidity_token_program.key,
            keys.deposit_accounts_liquidity_token_program,
        ),
        (
            *accounts.deposit_accounts_instruction_sysvar_account.key,
            keys.deposit_accounts_instruction_sysvar_account,
        ),
        (
            *accounts.farms_accounts_obligation_farm_user_state.key,
            keys.farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.farms_accounts_reserve_farm_state.key,
            keys.farms_accounts_reserve_farm_state,
        ),
        (*accounts.farms_program.key, keys.farms_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_v2_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.deposit_accounts_owner,
        accounts.deposit_accounts_obligation,
        accounts.deposit_accounts_reserve,
        accounts.deposit_accounts_reserve_liquidity_supply,
        accounts.deposit_accounts_reserve_collateral_mint,
        accounts.deposit_accounts_reserve_destination_deposit_collateral,
        accounts.deposit_accounts_user_source_liquidity,
        accounts.farms_accounts_obligation_farm_user_state,
        accounts.farms_accounts_reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_v2_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.deposit_accounts_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deposit_reserve_liquidity_and_obligation_collateral_v2_verify_account_privileges<
    'me,
    'info,
>(
    accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deposit_reserve_liquidity_and_obligation_collateral_v2_verify_writable_privileges(
        accounts,
    )?;
    deposit_reserve_liquidity_and_obligation_collateral_v2_verify_signer_privileges(
        accounts,
    )?;
    Ok(())
}
pub const WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub withdraw_reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub reserve_source_collateral: &'me AccountInfo<'info>,
    pub reserve_collateral_mint: &'me AccountInfo<'info>,
    pub reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub user_destination_liquidity: &'me AccountInfo<'info>,
    pub placeholder_user_destination_collateral: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub liquidity_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawObligationCollateralAndRedeemReserveCollateralKeys {
    pub owner: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub withdraw_reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_source_collateral: Pubkey,
    pub reserve_collateral_mint: Pubkey,
    pub reserve_liquidity_supply: Pubkey,
    pub user_destination_liquidity: Pubkey,
    pub placeholder_user_destination_collateral: Pubkey,
    pub collateral_token_program: Pubkey,
    pub liquidity_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'_, '_>>
for WithdrawObligationCollateralAndRedeemReserveCollateralKeys {
    fn from(
        accounts: WithdrawObligationCollateralAndRedeemReserveCollateralAccounts,
    ) -> Self {
        Self {
            owner: *accounts.owner.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            withdraw_reserve: *accounts.withdraw_reserve.key,
            reserve_liquidity_mint: *accounts.reserve_liquidity_mint.key,
            reserve_source_collateral: *accounts.reserve_source_collateral.key,
            reserve_collateral_mint: *accounts.reserve_collateral_mint.key,
            reserve_liquidity_supply: *accounts.reserve_liquidity_supply.key,
            user_destination_liquidity: *accounts.user_destination_liquidity.key,
            placeholder_user_destination_collateral: *accounts
                .placeholder_user_destination_collateral
                .key,
            collateral_token_program: *accounts.collateral_token_program.key,
            liquidity_token_program: *accounts.liquidity_token_program.key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<WithdrawObligationCollateralAndRedeemReserveCollateralKeys>
for [AccountMeta; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawObligationCollateralAndRedeemReserveCollateralKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_source_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.placeholder_user_destination_collateral,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<
    [Pubkey; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN],
> for WithdrawObligationCollateralAndRedeemReserveCollateralKeys {
    fn from(
        pubkeys: [Pubkey; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: pubkeys[0],
            obligation: pubkeys[1],
            lending_market: pubkeys[2],
            lending_market_authority: pubkeys[3],
            withdraw_reserve: pubkeys[4],
            reserve_liquidity_mint: pubkeys[5],
            reserve_source_collateral: pubkeys[6],
            reserve_collateral_mint: pubkeys[7],
            reserve_liquidity_supply: pubkeys[8],
            user_destination_liquidity: pubkeys[9],
            placeholder_user_destination_collateral: pubkeys[10],
            collateral_token_program: pubkeys[11],
            liquidity_token_program: pubkeys[12],
            instruction_sysvar_account: pubkeys[13],
        }
    }
}
impl<
    'info,
> From<WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'_, 'info>>
for [AccountInfo<
    'info,
>; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<
            '_,
            'info,
        >,
    ) -> Self {
        [
            accounts.owner.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.withdraw_reserve.clone(),
            accounts.reserve_liquidity_mint.clone(),
            accounts.reserve_source_collateral.clone(),
            accounts.reserve_collateral_mint.clone(),
            accounts.reserve_liquidity_supply.clone(),
            accounts.user_destination_liquidity.clone(),
            accounts.placeholder_user_destination_collateral.clone(),
            accounts.collateral_token_program.clone(),
            accounts.liquidity_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN],
> for WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            obligation: &arr[1],
            lending_market: &arr[2],
            lending_market_authority: &arr[3],
            withdraw_reserve: &arr[4],
            reserve_liquidity_mint: &arr[5],
            reserve_source_collateral: &arr[6],
            reserve_collateral_mint: &arr[7],
            reserve_liquidity_supply: &arr[8],
            user_destination_liquidity: &arr[9],
            placeholder_user_destination_collateral: &arr[10],
            collateral_token_program: &arr[11],
            liquidity_token_program: &arr[12],
            instruction_sysvar_account: &arr[13],
        }
    }
}
pub const WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM: [u8; 8] = [
    75,
    93,
    93,
    220,
    34,
    150,
    218,
    196,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs {
    pub collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawObligationCollateralAndRedeemReserveCollateralIxData(
    pub WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs,
);
impl From<WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs>
for WithdrawObligationCollateralAndRedeemReserveCollateralIxData {
    fn from(args: WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs) -> Self {
        Self(args)
    }
}
impl WithdrawObligationCollateralAndRedeemReserveCollateralIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm
            != WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM
        {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM,
                        maybe_discm
                    ),
                ),
            );
        }
        Ok(
            Self(
                WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs::deserialize(
                    &mut reader,
                )?,
            ),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer
            .write_all(
                &WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM,
            )?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawObligationCollateralAndRedeemReserveCollateralKeys,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawObligationCollateralAndRedeemReserveCollateralIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_ix(
    keys: WithdrawObligationCollateralAndRedeemReserveCollateralKeys,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs,
) -> std::io::Result<Instruction> {
    withdraw_obligation_collateral_and_redeem_reserve_collateral_ix_with_program_id(
        crate::ID,
        keys,
        args,
    )
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'_, '_>,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs,
) -> ProgramResult {
    let keys: WithdrawObligationCollateralAndRedeemReserveCollateralKeys = accounts
        .into();
    let ix = withdraw_obligation_collateral_and_redeem_reserve_collateral_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_invoke(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'_, '_>,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs,
) -> ProgramResult {
    withdraw_obligation_collateral_and_redeem_reserve_collateral_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'_, '_>,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawObligationCollateralAndRedeemReserveCollateralKeys = accounts
        .into();
    let ix = withdraw_obligation_collateral_and_redeem_reserve_collateral_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_invoke_signed(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'_, '_>,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_obligation_collateral_and_redeem_reserve_collateral_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_verify_account_keys(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'_, '_>,
    keys: WithdrawObligationCollateralAndRedeemReserveCollateralKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.withdraw_reserve.key, keys.withdraw_reserve),
        (*accounts.reserve_liquidity_mint.key, keys.reserve_liquidity_mint),
        (*accounts.reserve_source_collateral.key, keys.reserve_source_collateral),
        (*accounts.reserve_collateral_mint.key, keys.reserve_collateral_mint),
        (*accounts.reserve_liquidity_supply.key, keys.reserve_liquidity_supply),
        (*accounts.user_destination_liquidity.key, keys.user_destination_liquidity),
        (
            *accounts.placeholder_user_destination_collateral.key,
            keys.placeholder_user_destination_collateral,
        ),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (*accounts.liquidity_token_program.key, keys.liquidity_token_program),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.obligation,
        accounts.withdraw_reserve,
        accounts.reserve_source_collateral,
        accounts.reserve_collateral_mint,
        accounts.reserve_liquidity_supply,
        accounts.user_destination_liquidity,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_verify_account_privileges<
    'me,
    'info,
>(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_obligation_collateral_and_redeem_reserve_collateral_verify_writable_privileges(
        accounts,
    )?;
    withdraw_obligation_collateral_and_redeem_reserve_collateral_verify_signer_privileges(
        accounts,
    )?;
    Ok(())
}
pub const WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<'me, 'info> {
    pub withdraw_accounts_owner: &'me AccountInfo<'info>,
    pub withdraw_accounts_obligation: &'me AccountInfo<'info>,
    pub withdraw_accounts_lending_market: &'me AccountInfo<'info>,
    pub withdraw_accounts_lending_market_authority: &'me AccountInfo<'info>,
    pub withdraw_accounts_withdraw_reserve: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_source_collateral: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_collateral_mint: &'me AccountInfo<'info>,
    pub withdraw_accounts_reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub withdraw_accounts_user_destination_liquidity: &'me AccountInfo<'info>,
    pub withdraw_accounts_placeholder_user_destination_collateral: &'me AccountInfo<
        'info,
    >,
    pub withdraw_accounts_collateral_token_program: &'me AccountInfo<'info>,
    pub withdraw_accounts_liquidity_token_program: &'me AccountInfo<'info>,
    pub withdraw_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawObligationCollateralAndRedeemReserveCollateralV2Keys {
    pub withdraw_accounts_owner: Pubkey,
    pub withdraw_accounts_obligation: Pubkey,
    pub withdraw_accounts_lending_market: Pubkey,
    pub withdraw_accounts_lending_market_authority: Pubkey,
    pub withdraw_accounts_withdraw_reserve: Pubkey,
    pub withdraw_accounts_reserve_liquidity_mint: Pubkey,
    pub withdraw_accounts_reserve_source_collateral: Pubkey,
    pub withdraw_accounts_reserve_collateral_mint: Pubkey,
    pub withdraw_accounts_reserve_liquidity_supply: Pubkey,
    pub withdraw_accounts_user_destination_liquidity: Pubkey,
    pub withdraw_accounts_placeholder_user_destination_collateral: Pubkey,
    pub withdraw_accounts_collateral_token_program: Pubkey,
    pub withdraw_accounts_liquidity_token_program: Pubkey,
    pub withdraw_accounts_instruction_sysvar_account: Pubkey,
    pub farms_accounts_obligation_farm_user_state: Pubkey,
    pub farms_accounts_reserve_farm_state: Pubkey,
    pub farms_program: Pubkey,
}
impl From<WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<'_, '_>>
for WithdrawObligationCollateralAndRedeemReserveCollateralV2Keys {
    fn from(
        accounts: WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts,
    ) -> Self {
        Self {
            withdraw_accounts_owner: *accounts.withdraw_accounts_owner.key,
            withdraw_accounts_obligation: *accounts.withdraw_accounts_obligation.key,
            withdraw_accounts_lending_market: *accounts
                .withdraw_accounts_lending_market
                .key,
            withdraw_accounts_lending_market_authority: *accounts
                .withdraw_accounts_lending_market_authority
                .key,
            withdraw_accounts_withdraw_reserve: *accounts
                .withdraw_accounts_withdraw_reserve
                .key,
            withdraw_accounts_reserve_liquidity_mint: *accounts
                .withdraw_accounts_reserve_liquidity_mint
                .key,
            withdraw_accounts_reserve_source_collateral: *accounts
                .withdraw_accounts_reserve_source_collateral
                .key,
            withdraw_accounts_reserve_collateral_mint: *accounts
                .withdraw_accounts_reserve_collateral_mint
                .key,
            withdraw_accounts_reserve_liquidity_supply: *accounts
                .withdraw_accounts_reserve_liquidity_supply
                .key,
            withdraw_accounts_user_destination_liquidity: *accounts
                .withdraw_accounts_user_destination_liquidity
                .key,
            withdraw_accounts_placeholder_user_destination_collateral: *accounts
                .withdraw_accounts_placeholder_user_destination_collateral
                .key,
            withdraw_accounts_collateral_token_program: *accounts
                .withdraw_accounts_collateral_token_program
                .key,
            withdraw_accounts_liquidity_token_program: *accounts
                .withdraw_accounts_liquidity_token_program
                .key,
            withdraw_accounts_instruction_sysvar_account: *accounts
                .withdraw_accounts_instruction_sysvar_account
                .key,
            farms_accounts_obligation_farm_user_state: *accounts
                .farms_accounts_obligation_farm_user_state
                .key,
            farms_accounts_reserve_farm_state: *accounts
                .farms_accounts_reserve_farm_state
                .key,
            farms_program: *accounts.farms_program.key,
        }
    }
}
impl From<WithdrawObligationCollateralAndRedeemReserveCollateralV2Keys>
for [AccountMeta; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawObligationCollateralAndRedeemReserveCollateralV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.withdraw_accounts_owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_withdraw_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_source_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_user_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_placeholder_user_destination_collateral,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<
    [Pubkey; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN],
> for WithdrawObligationCollateralAndRedeemReserveCollateralV2Keys {
    fn from(
        pubkeys: [Pubkey; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            withdraw_accounts_owner: pubkeys[0],
            withdraw_accounts_obligation: pubkeys[1],
            withdraw_accounts_lending_market: pubkeys[2],
            withdraw_accounts_lending_market_authority: pubkeys[3],
            withdraw_accounts_withdraw_reserve: pubkeys[4],
            withdraw_accounts_reserve_liquidity_mint: pubkeys[5],
            withdraw_accounts_reserve_source_collateral: pubkeys[6],
            withdraw_accounts_reserve_collateral_mint: pubkeys[7],
            withdraw_accounts_reserve_liquidity_supply: pubkeys[8],
            withdraw_accounts_user_destination_liquidity: pubkeys[9],
            withdraw_accounts_placeholder_user_destination_collateral: pubkeys[10],
            withdraw_accounts_collateral_token_program: pubkeys[11],
            withdraw_accounts_liquidity_token_program: pubkeys[12],
            withdraw_accounts_instruction_sysvar_account: pubkeys[13],
            farms_accounts_obligation_farm_user_state: pubkeys[14],
            farms_accounts_reserve_farm_state: pubkeys[15],
            farms_program: pubkeys[16],
        }
    }
}
impl<
    'info,
> From<WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<'_, 'info>>
for [AccountInfo<
    'info,
>; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<
            '_,
            'info,
        >,
    ) -> Self {
        [
            accounts.withdraw_accounts_owner.clone(),
            accounts.withdraw_accounts_obligation.clone(),
            accounts.withdraw_accounts_lending_market.clone(),
            accounts.withdraw_accounts_lending_market_authority.clone(),
            accounts.withdraw_accounts_withdraw_reserve.clone(),
            accounts.withdraw_accounts_reserve_liquidity_mint.clone(),
            accounts.withdraw_accounts_reserve_source_collateral.clone(),
            accounts.withdraw_accounts_reserve_collateral_mint.clone(),
            accounts.withdraw_accounts_reserve_liquidity_supply.clone(),
            accounts.withdraw_accounts_user_destination_liquidity.clone(),
            accounts.withdraw_accounts_placeholder_user_destination_collateral.clone(),
            accounts.withdraw_accounts_collateral_token_program.clone(),
            accounts.withdraw_accounts_liquidity_token_program.clone(),
            accounts.withdraw_accounts_instruction_sysvar_account.clone(),
            accounts.farms_accounts_obligation_farm_user_state.clone(),
            accounts.farms_accounts_reserve_farm_state.clone(),
            accounts.farms_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN],
> for WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            withdraw_accounts_owner: &arr[0],
            withdraw_accounts_obligation: &arr[1],
            withdraw_accounts_lending_market: &arr[2],
            withdraw_accounts_lending_market_authority: &arr[3],
            withdraw_accounts_withdraw_reserve: &arr[4],
            withdraw_accounts_reserve_liquidity_mint: &arr[5],
            withdraw_accounts_reserve_source_collateral: &arr[6],
            withdraw_accounts_reserve_collateral_mint: &arr[7],
            withdraw_accounts_reserve_liquidity_supply: &arr[8],
            withdraw_accounts_user_destination_liquidity: &arr[9],
            withdraw_accounts_placeholder_user_destination_collateral: &arr[10],
            withdraw_accounts_collateral_token_program: &arr[11],
            withdraw_accounts_liquidity_token_program: &arr[12],
            withdraw_accounts_instruction_sysvar_account: &arr[13],
            farms_accounts_obligation_farm_user_state: &arr[14],
            farms_accounts_reserve_farm_state: &arr[15],
            farms_program: &arr[16],
        }
    }
}
pub const WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM: [u8; 8] = [
    235,
    52,
    119,
    152,
    149,
    197,
    20,
    7,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs {
    pub collateral_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawObligationCollateralAndRedeemReserveCollateralV2IxData(
    pub WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs,
);
impl From<WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs>
for WithdrawObligationCollateralAndRedeemReserveCollateralV2IxData {
    fn from(
        args: WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs,
    ) -> Self {
        Self(args)
    }
}
impl WithdrawObligationCollateralAndRedeemReserveCollateralV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm
            != WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM
        {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM,
                        maybe_discm
                    ),
                ),
            );
        }
        Ok(
            Self(
                WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs::deserialize(
                    &mut reader,
                )?,
            ),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer
            .write_all(
                &WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM,
            )?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawObligationCollateralAndRedeemReserveCollateralV2Keys,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_OBLIGATION_COLLATERAL_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: WithdrawObligationCollateralAndRedeemReserveCollateralV2IxData = args
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_ix(
    keys: WithdrawObligationCollateralAndRedeemReserveCollateralV2Keys,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs,
) -> std::io::Result<Instruction> {
    withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_ix_with_program_id(
        crate::ID,
        keys,
        args,
    )
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<'_, '_>,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs,
) -> ProgramResult {
    let keys: WithdrawObligationCollateralAndRedeemReserveCollateralV2Keys = accounts
        .into();
    let ix = withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_invoke(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<'_, '_>,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs,
) -> ProgramResult {
    withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<'_, '_>,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawObligationCollateralAndRedeemReserveCollateralV2Keys = accounts
        .into();
    let ix = withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_invoke_signed(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<'_, '_>,
    args: WithdrawObligationCollateralAndRedeemReserveCollateralV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_verify_account_keys(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<'_, '_>,
    keys: WithdrawObligationCollateralAndRedeemReserveCollateralV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.withdraw_accounts_owner.key, keys.withdraw_accounts_owner),
        (*accounts.withdraw_accounts_obligation.key, keys.withdraw_accounts_obligation),
        (
            *accounts.withdraw_accounts_lending_market.key,
            keys.withdraw_accounts_lending_market,
        ),
        (
            *accounts.withdraw_accounts_lending_market_authority.key,
            keys.withdraw_accounts_lending_market_authority,
        ),
        (
            *accounts.withdraw_accounts_withdraw_reserve.key,
            keys.withdraw_accounts_withdraw_reserve,
        ),
        (
            *accounts.withdraw_accounts_reserve_liquidity_mint.key,
            keys.withdraw_accounts_reserve_liquidity_mint,
        ),
        (
            *accounts.withdraw_accounts_reserve_source_collateral.key,
            keys.withdraw_accounts_reserve_source_collateral,
        ),
        (
            *accounts.withdraw_accounts_reserve_collateral_mint.key,
            keys.withdraw_accounts_reserve_collateral_mint,
        ),
        (
            *accounts.withdraw_accounts_reserve_liquidity_supply.key,
            keys.withdraw_accounts_reserve_liquidity_supply,
        ),
        (
            *accounts.withdraw_accounts_user_destination_liquidity.key,
            keys.withdraw_accounts_user_destination_liquidity,
        ),
        (
            *accounts.withdraw_accounts_placeholder_user_destination_collateral.key,
            keys.withdraw_accounts_placeholder_user_destination_collateral,
        ),
        (
            *accounts.withdraw_accounts_collateral_token_program.key,
            keys.withdraw_accounts_collateral_token_program,
        ),
        (
            *accounts.withdraw_accounts_liquidity_token_program.key,
            keys.withdraw_accounts_liquidity_token_program,
        ),
        (
            *accounts.withdraw_accounts_instruction_sysvar_account.key,
            keys.withdraw_accounts_instruction_sysvar_account,
        ),
        (
            *accounts.farms_accounts_obligation_farm_user_state.key,
            keys.farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.farms_accounts_reserve_farm_state.key,
            keys.farms_accounts_reserve_farm_state,
        ),
        (*accounts.farms_program.key, keys.farms_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<
        'me,
        'info,
    >,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.withdraw_accounts_owner,
        accounts.withdraw_accounts_obligation,
        accounts.withdraw_accounts_withdraw_reserve,
        accounts.withdraw_accounts_reserve_source_collateral,
        accounts.withdraw_accounts_reserve_collateral_mint,
        accounts.withdraw_accounts_reserve_liquidity_supply,
        accounts.withdraw_accounts_user_destination_liquidity,
        accounts.farms_accounts_obligation_farm_user_state,
        accounts.farms_accounts_reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<
        'me,
        'info,
    >,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.withdraw_accounts_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_verify_account_privileges<
    'me,
    'info,
>(
    accounts: WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts<
        'me,
        'info,
    >,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_verify_writable_privileges(
        accounts,
    )?;
    withdraw_obligation_collateral_and_redeem_reserve_collateral_v2_verify_signer_privileges(
        accounts,
    )?;
    Ok(())
}
pub const LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateObligationAndRedeemReserveCollateralAccounts<'me, 'info> {
    pub liquidator: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub repay_reserve: &'me AccountInfo<'info>,
    pub repay_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub repay_reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub withdraw_reserve: &'me AccountInfo<'info>,
    pub withdraw_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub withdraw_reserve_collateral_mint: &'me AccountInfo<'info>,
    pub withdraw_reserve_collateral_supply: &'me AccountInfo<'info>,
    pub withdraw_reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub withdraw_reserve_liquidity_fee_receiver: &'me AccountInfo<'info>,
    pub user_source_liquidity: &'me AccountInfo<'info>,
    pub user_destination_collateral: &'me AccountInfo<'info>,
    pub user_destination_liquidity: &'me AccountInfo<'info>,
    pub collateral_token_program: &'me AccountInfo<'info>,
    pub repay_liquidity_token_program: &'me AccountInfo<'info>,
    pub withdraw_liquidity_token_program: &'me AccountInfo<'info>,
    pub instruction_sysvar_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidateObligationAndRedeemReserveCollateralKeys {
    pub liquidator: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub repay_reserve: Pubkey,
    pub repay_reserve_liquidity_mint: Pubkey,
    pub repay_reserve_liquidity_supply: Pubkey,
    pub withdraw_reserve: Pubkey,
    pub withdraw_reserve_liquidity_mint: Pubkey,
    pub withdraw_reserve_collateral_mint: Pubkey,
    pub withdraw_reserve_collateral_supply: Pubkey,
    pub withdraw_reserve_liquidity_supply: Pubkey,
    pub withdraw_reserve_liquidity_fee_receiver: Pubkey,
    pub user_source_liquidity: Pubkey,
    pub user_destination_collateral: Pubkey,
    pub user_destination_liquidity: Pubkey,
    pub collateral_token_program: Pubkey,
    pub repay_liquidity_token_program: Pubkey,
    pub withdraw_liquidity_token_program: Pubkey,
    pub instruction_sysvar_account: Pubkey,
}
impl From<LiquidateObligationAndRedeemReserveCollateralAccounts<'_, '_>>
for LiquidateObligationAndRedeemReserveCollateralKeys {
    fn from(accounts: LiquidateObligationAndRedeemReserveCollateralAccounts) -> Self {
        Self {
            liquidator: *accounts.liquidator.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            repay_reserve: *accounts.repay_reserve.key,
            repay_reserve_liquidity_mint: *accounts.repay_reserve_liquidity_mint.key,
            repay_reserve_liquidity_supply: *accounts.repay_reserve_liquidity_supply.key,
            withdraw_reserve: *accounts.withdraw_reserve.key,
            withdraw_reserve_liquidity_mint: *accounts
                .withdraw_reserve_liquidity_mint
                .key,
            withdraw_reserve_collateral_mint: *accounts
                .withdraw_reserve_collateral_mint
                .key,
            withdraw_reserve_collateral_supply: *accounts
                .withdraw_reserve_collateral_supply
                .key,
            withdraw_reserve_liquidity_supply: *accounts
                .withdraw_reserve_liquidity_supply
                .key,
            withdraw_reserve_liquidity_fee_receiver: *accounts
                .withdraw_reserve_liquidity_fee_receiver
                .key,
            user_source_liquidity: *accounts.user_source_liquidity.key,
            user_destination_collateral: *accounts.user_destination_collateral.key,
            user_destination_liquidity: *accounts.user_destination_liquidity.key,
            collateral_token_program: *accounts.collateral_token_program.key,
            repay_liquidity_token_program: *accounts.repay_liquidity_token_program.key,
            withdraw_liquidity_token_program: *accounts
                .withdraw_liquidity_token_program
                .key,
            instruction_sysvar_account: *accounts.instruction_sysvar_account.key,
        }
    }
}
impl From<LiquidateObligationAndRedeemReserveCollateralKeys>
for [AccountMeta; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidateObligationAndRedeemReserveCollateralKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.repay_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_reserve_collateral_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_reserve_liquidity_fee_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.repay_liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN]>
for LiquidateObligationAndRedeemReserveCollateralKeys {
    fn from(
        pubkeys: [Pubkey; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidator: pubkeys[0],
            obligation: pubkeys[1],
            lending_market: pubkeys[2],
            lending_market_authority: pubkeys[3],
            repay_reserve: pubkeys[4],
            repay_reserve_liquidity_mint: pubkeys[5],
            repay_reserve_liquidity_supply: pubkeys[6],
            withdraw_reserve: pubkeys[7],
            withdraw_reserve_liquidity_mint: pubkeys[8],
            withdraw_reserve_collateral_mint: pubkeys[9],
            withdraw_reserve_collateral_supply: pubkeys[10],
            withdraw_reserve_liquidity_supply: pubkeys[11],
            withdraw_reserve_liquidity_fee_receiver: pubkeys[12],
            user_source_liquidity: pubkeys[13],
            user_destination_collateral: pubkeys[14],
            user_destination_liquidity: pubkeys[15],
            collateral_token_program: pubkeys[16],
            repay_liquidity_token_program: pubkeys[17],
            withdraw_liquidity_token_program: pubkeys[18],
            instruction_sysvar_account: pubkeys[19],
        }
    }
}
impl<'info> From<LiquidateObligationAndRedeemReserveCollateralAccounts<'_, 'info>>
for [AccountInfo<
    'info,
>; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: LiquidateObligationAndRedeemReserveCollateralAccounts<'_, 'info>,
    ) -> Self {
        [
            accounts.liquidator.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.repay_reserve.clone(),
            accounts.repay_reserve_liquidity_mint.clone(),
            accounts.repay_reserve_liquidity_supply.clone(),
            accounts.withdraw_reserve.clone(),
            accounts.withdraw_reserve_liquidity_mint.clone(),
            accounts.withdraw_reserve_collateral_mint.clone(),
            accounts.withdraw_reserve_collateral_supply.clone(),
            accounts.withdraw_reserve_liquidity_supply.clone(),
            accounts.withdraw_reserve_liquidity_fee_receiver.clone(),
            accounts.user_source_liquidity.clone(),
            accounts.user_destination_collateral.clone(),
            accounts.user_destination_liquidity.clone(),
            accounts.collateral_token_program.clone(),
            accounts.repay_liquidity_token_program.clone(),
            accounts.withdraw_liquidity_token_program.clone(),
            accounts.instruction_sysvar_account.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN],
> for LiquidateObligationAndRedeemReserveCollateralAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidator: &arr[0],
            obligation: &arr[1],
            lending_market: &arr[2],
            lending_market_authority: &arr[3],
            repay_reserve: &arr[4],
            repay_reserve_liquidity_mint: &arr[5],
            repay_reserve_liquidity_supply: &arr[6],
            withdraw_reserve: &arr[7],
            withdraw_reserve_liquidity_mint: &arr[8],
            withdraw_reserve_collateral_mint: &arr[9],
            withdraw_reserve_collateral_supply: &arr[10],
            withdraw_reserve_liquidity_supply: &arr[11],
            withdraw_reserve_liquidity_fee_receiver: &arr[12],
            user_source_liquidity: &arr[13],
            user_destination_collateral: &arr[14],
            user_destination_liquidity: &arr[15],
            collateral_token_program: &arr[16],
            repay_liquidity_token_program: &arr[17],
            withdraw_liquidity_token_program: &arr[18],
            instruction_sysvar_account: &arr[19],
        }
    }
}
pub const LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM: [u8; 8] = [
    177,
    71,
    154,
    188,
    226,
    133,
    74,
    55,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidateObligationAndRedeemReserveCollateralIxArgs {
    pub liquidity_amount: u64,
    pub min_acceptable_received_liquidity_amount: u64,
    pub max_allowed_ltv_override_percent: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidateObligationAndRedeemReserveCollateralIxData(
    pub LiquidateObligationAndRedeemReserveCollateralIxArgs,
);
impl From<LiquidateObligationAndRedeemReserveCollateralIxArgs>
for LiquidateObligationAndRedeemReserveCollateralIxData {
    fn from(args: LiquidateObligationAndRedeemReserveCollateralIxArgs) -> Self {
        Self(args)
    }
}
impl LiquidateObligationAndRedeemReserveCollateralIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM,
                        maybe_discm
                    ),
                ),
            );
        }
        Ok(
            Self(
                LiquidateObligationAndRedeemReserveCollateralIxArgs::deserialize(
                    &mut reader,
                )?,
            ),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidateObligationAndRedeemReserveCollateralKeys,
    args: LiquidateObligationAndRedeemReserveCollateralIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LiquidateObligationAndRedeemReserveCollateralIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_ix(
    keys: LiquidateObligationAndRedeemReserveCollateralKeys,
    args: LiquidateObligationAndRedeemReserveCollateralIxArgs,
) -> std::io::Result<Instruction> {
    liquidate_obligation_and_redeem_reserve_collateral_ix_with_program_id(
        crate::ID,
        keys,
        args,
    )
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateObligationAndRedeemReserveCollateralAccounts<'_, '_>,
    args: LiquidateObligationAndRedeemReserveCollateralIxArgs,
) -> ProgramResult {
    let keys: LiquidateObligationAndRedeemReserveCollateralKeys = accounts.into();
    let ix = liquidate_obligation_and_redeem_reserve_collateral_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_invoke(
    accounts: LiquidateObligationAndRedeemReserveCollateralAccounts<'_, '_>,
    args: LiquidateObligationAndRedeemReserveCollateralIxArgs,
) -> ProgramResult {
    liquidate_obligation_and_redeem_reserve_collateral_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateObligationAndRedeemReserveCollateralAccounts<'_, '_>,
    args: LiquidateObligationAndRedeemReserveCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidateObligationAndRedeemReserveCollateralKeys = accounts.into();
    let ix = liquidate_obligation_and_redeem_reserve_collateral_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_invoke_signed(
    accounts: LiquidateObligationAndRedeemReserveCollateralAccounts<'_, '_>,
    args: LiquidateObligationAndRedeemReserveCollateralIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquidate_obligation_and_redeem_reserve_collateral_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_verify_account_keys(
    accounts: LiquidateObligationAndRedeemReserveCollateralAccounts<'_, '_>,
    keys: LiquidateObligationAndRedeemReserveCollateralKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.liquidator.key, keys.liquidator),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.repay_reserve.key, keys.repay_reserve),
        (*accounts.repay_reserve_liquidity_mint.key, keys.repay_reserve_liquidity_mint),
        (
            *accounts.repay_reserve_liquidity_supply.key,
            keys.repay_reserve_liquidity_supply,
        ),
        (*accounts.withdraw_reserve.key, keys.withdraw_reserve),
        (
            *accounts.withdraw_reserve_liquidity_mint.key,
            keys.withdraw_reserve_liquidity_mint,
        ),
        (
            *accounts.withdraw_reserve_collateral_mint.key,
            keys.withdraw_reserve_collateral_mint,
        ),
        (
            *accounts.withdraw_reserve_collateral_supply.key,
            keys.withdraw_reserve_collateral_supply,
        ),
        (
            *accounts.withdraw_reserve_liquidity_supply.key,
            keys.withdraw_reserve_liquidity_supply,
        ),
        (
            *accounts.withdraw_reserve_liquidity_fee_receiver.key,
            keys.withdraw_reserve_liquidity_fee_receiver,
        ),
        (*accounts.user_source_liquidity.key, keys.user_source_liquidity),
        (*accounts.user_destination_collateral.key, keys.user_destination_collateral),
        (*accounts.user_destination_liquidity.key, keys.user_destination_liquidity),
        (*accounts.collateral_token_program.key, keys.collateral_token_program),
        (
            *accounts.repay_liquidity_token_program.key,
            keys.repay_liquidity_token_program,
        ),
        (
            *accounts.withdraw_liquidity_token_program.key,
            keys.withdraw_liquidity_token_program,
        ),
        (*accounts.instruction_sysvar_account.key, keys.instruction_sysvar_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: LiquidateObligationAndRedeemReserveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.obligation,
        accounts.repay_reserve,
        accounts.repay_reserve_liquidity_supply,
        accounts.withdraw_reserve,
        accounts.withdraw_reserve_collateral_mint,
        accounts.withdraw_reserve_collateral_supply,
        accounts.withdraw_reserve_liquidity_supply,
        accounts.withdraw_reserve_liquidity_fee_receiver,
        accounts.user_source_liquidity,
        accounts.user_destination_collateral,
        accounts.user_destination_liquidity,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: LiquidateObligationAndRedeemReserveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.liquidator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_verify_account_privileges<
    'me,
    'info,
>(
    accounts: LiquidateObligationAndRedeemReserveCollateralAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquidate_obligation_and_redeem_reserve_collateral_verify_writable_privileges(
        accounts,
    )?;
    liquidate_obligation_and_redeem_reserve_collateral_verify_signer_privileges(
        accounts,
    )?;
    Ok(())
}
pub const LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN: usize = 25;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateObligationAndRedeemReserveCollateralV2Accounts<'me, 'info> {
    pub liquidation_accounts_liquidator: &'me AccountInfo<'info>,
    pub liquidation_accounts_obligation: &'me AccountInfo<'info>,
    pub liquidation_accounts_lending_market: &'me AccountInfo<'info>,
    pub liquidation_accounts_lending_market_authority: &'me AccountInfo<'info>,
    pub liquidation_accounts_repay_reserve: &'me AccountInfo<'info>,
    pub liquidation_accounts_repay_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub liquidation_accounts_repay_reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub liquidation_accounts_withdraw_reserve: &'me AccountInfo<'info>,
    pub liquidation_accounts_withdraw_reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub liquidation_accounts_withdraw_reserve_collateral_mint: &'me AccountInfo<'info>,
    pub liquidation_accounts_withdraw_reserve_collateral_supply: &'me AccountInfo<'info>,
    pub liquidation_accounts_withdraw_reserve_liquidity_supply: &'me AccountInfo<'info>,
    pub liquidation_accounts_withdraw_reserve_liquidity_fee_receiver: &'me AccountInfo<
        'info,
    >,
    pub liquidation_accounts_user_source_liquidity: &'me AccountInfo<'info>,
    pub liquidation_accounts_user_destination_collateral: &'me AccountInfo<'info>,
    pub liquidation_accounts_user_destination_liquidity: &'me AccountInfo<'info>,
    pub liquidation_accounts_collateral_token_program: &'me AccountInfo<'info>,
    pub liquidation_accounts_repay_liquidity_token_program: &'me AccountInfo<'info>,
    pub liquidation_accounts_withdraw_liquidity_token_program: &'me AccountInfo<'info>,
    pub liquidation_accounts_instruction_sysvar_account: &'me AccountInfo<'info>,
    pub collateral_farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub collateral_farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub debt_farms_accounts_obligation_farm_user_state: &'me AccountInfo<'info>,
    pub debt_farms_accounts_reserve_farm_state: &'me AccountInfo<'info>,
    pub farms_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiquidateObligationAndRedeemReserveCollateralV2Keys {
    pub liquidation_accounts_liquidator: Pubkey,
    pub liquidation_accounts_obligation: Pubkey,
    pub liquidation_accounts_lending_market: Pubkey,
    pub liquidation_accounts_lending_market_authority: Pubkey,
    pub liquidation_accounts_repay_reserve: Pubkey,
    pub liquidation_accounts_repay_reserve_liquidity_mint: Pubkey,
    pub liquidation_accounts_repay_reserve_liquidity_supply: Pubkey,
    pub liquidation_accounts_withdraw_reserve: Pubkey,
    pub liquidation_accounts_withdraw_reserve_liquidity_mint: Pubkey,
    pub liquidation_accounts_withdraw_reserve_collateral_mint: Pubkey,
    pub liquidation_accounts_withdraw_reserve_collateral_supply: Pubkey,
    pub liquidation_accounts_withdraw_reserve_liquidity_supply: Pubkey,
    pub liquidation_accounts_withdraw_reserve_liquidity_fee_receiver: Pubkey,
    pub liquidation_accounts_user_source_liquidity: Pubkey,
    pub liquidation_accounts_user_destination_collateral: Pubkey,
    pub liquidation_accounts_user_destination_liquidity: Pubkey,
    pub liquidation_accounts_collateral_token_program: Pubkey,
    pub liquidation_accounts_repay_liquidity_token_program: Pubkey,
    pub liquidation_accounts_withdraw_liquidity_token_program: Pubkey,
    pub liquidation_accounts_instruction_sysvar_account: Pubkey,
    pub collateral_farms_accounts_obligation_farm_user_state: Pubkey,
    pub collateral_farms_accounts_reserve_farm_state: Pubkey,
    pub debt_farms_accounts_obligation_farm_user_state: Pubkey,
    pub debt_farms_accounts_reserve_farm_state: Pubkey,
    pub farms_program: Pubkey,
}
impl From<LiquidateObligationAndRedeemReserveCollateralV2Accounts<'_, '_>>
for LiquidateObligationAndRedeemReserveCollateralV2Keys {
    fn from(accounts: LiquidateObligationAndRedeemReserveCollateralV2Accounts) -> Self {
        Self {
            liquidation_accounts_liquidator: *accounts
                .liquidation_accounts_liquidator
                .key,
            liquidation_accounts_obligation: *accounts
                .liquidation_accounts_obligation
                .key,
            liquidation_accounts_lending_market: *accounts
                .liquidation_accounts_lending_market
                .key,
            liquidation_accounts_lending_market_authority: *accounts
                .liquidation_accounts_lending_market_authority
                .key,
            liquidation_accounts_repay_reserve: *accounts
                .liquidation_accounts_repay_reserve
                .key,
            liquidation_accounts_repay_reserve_liquidity_mint: *accounts
                .liquidation_accounts_repay_reserve_liquidity_mint
                .key,
            liquidation_accounts_repay_reserve_liquidity_supply: *accounts
                .liquidation_accounts_repay_reserve_liquidity_supply
                .key,
            liquidation_accounts_withdraw_reserve: *accounts
                .liquidation_accounts_withdraw_reserve
                .key,
            liquidation_accounts_withdraw_reserve_liquidity_mint: *accounts
                .liquidation_accounts_withdraw_reserve_liquidity_mint
                .key,
            liquidation_accounts_withdraw_reserve_collateral_mint: *accounts
                .liquidation_accounts_withdraw_reserve_collateral_mint
                .key,
            liquidation_accounts_withdraw_reserve_collateral_supply: *accounts
                .liquidation_accounts_withdraw_reserve_collateral_supply
                .key,
            liquidation_accounts_withdraw_reserve_liquidity_supply: *accounts
                .liquidation_accounts_withdraw_reserve_liquidity_supply
                .key,
            liquidation_accounts_withdraw_reserve_liquidity_fee_receiver: *accounts
                .liquidation_accounts_withdraw_reserve_liquidity_fee_receiver
                .key,
            liquidation_accounts_user_source_liquidity: *accounts
                .liquidation_accounts_user_source_liquidity
                .key,
            liquidation_accounts_user_destination_collateral: *accounts
                .liquidation_accounts_user_destination_collateral
                .key,
            liquidation_accounts_user_destination_liquidity: *accounts
                .liquidation_accounts_user_destination_liquidity
                .key,
            liquidation_accounts_collateral_token_program: *accounts
                .liquidation_accounts_collateral_token_program
                .key,
            liquidation_accounts_repay_liquidity_token_program: *accounts
                .liquidation_accounts_repay_liquidity_token_program
                .key,
            liquidation_accounts_withdraw_liquidity_token_program: *accounts
                .liquidation_accounts_withdraw_liquidity_token_program
                .key,
            liquidation_accounts_instruction_sysvar_account: *accounts
                .liquidation_accounts_instruction_sysvar_account
                .key,
            collateral_farms_accounts_obligation_farm_user_state: *accounts
                .collateral_farms_accounts_obligation_farm_user_state
                .key,
            collateral_farms_accounts_reserve_farm_state: *accounts
                .collateral_farms_accounts_reserve_farm_state
                .key,
            debt_farms_accounts_obligation_farm_user_state: *accounts
                .debt_farms_accounts_obligation_farm_user_state
                .key,
            debt_farms_accounts_reserve_farm_state: *accounts
                .debt_farms_accounts_reserve_farm_state
                .key,
            farms_program: *accounts.farms_program.key,
        }
    }
}
impl From<LiquidateObligationAndRedeemReserveCollateralV2Keys>
for [AccountMeta; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: LiquidateObligationAndRedeemReserveCollateralV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.liquidation_accounts_liquidator,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_repay_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_repay_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_repay_reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_withdraw_reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_withdraw_reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_withdraw_reserve_collateral_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_withdraw_reserve_collateral_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_withdraw_reserve_liquidity_supply,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys
                    .liquidation_accounts_withdraw_reserve_liquidity_fee_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_user_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_user_destination_collateral,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_user_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_collateral_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_repay_liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_withdraw_liquidity_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.liquidation_accounts_instruction_sysvar_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.debt_farms_accounts_obligation_farm_user_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.debt_farms_accounts_reserve_farm_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.farms_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<
    [Pubkey; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN],
> for LiquidateObligationAndRedeemReserveCollateralV2Keys {
    fn from(
        pubkeys: [Pubkey; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidation_accounts_liquidator: pubkeys[0],
            liquidation_accounts_obligation: pubkeys[1],
            liquidation_accounts_lending_market: pubkeys[2],
            liquidation_accounts_lending_market_authority: pubkeys[3],
            liquidation_accounts_repay_reserve: pubkeys[4],
            liquidation_accounts_repay_reserve_liquidity_mint: pubkeys[5],
            liquidation_accounts_repay_reserve_liquidity_supply: pubkeys[6],
            liquidation_accounts_withdraw_reserve: pubkeys[7],
            liquidation_accounts_withdraw_reserve_liquidity_mint: pubkeys[8],
            liquidation_accounts_withdraw_reserve_collateral_mint: pubkeys[9],
            liquidation_accounts_withdraw_reserve_collateral_supply: pubkeys[10],
            liquidation_accounts_withdraw_reserve_liquidity_supply: pubkeys[11],
            liquidation_accounts_withdraw_reserve_liquidity_fee_receiver: pubkeys[12],
            liquidation_accounts_user_source_liquidity: pubkeys[13],
            liquidation_accounts_user_destination_collateral: pubkeys[14],
            liquidation_accounts_user_destination_liquidity: pubkeys[15],
            liquidation_accounts_collateral_token_program: pubkeys[16],
            liquidation_accounts_repay_liquidity_token_program: pubkeys[17],
            liquidation_accounts_withdraw_liquidity_token_program: pubkeys[18],
            liquidation_accounts_instruction_sysvar_account: pubkeys[19],
            collateral_farms_accounts_obligation_farm_user_state: pubkeys[20],
            collateral_farms_accounts_reserve_farm_state: pubkeys[21],
            debt_farms_accounts_obligation_farm_user_state: pubkeys[22],
            debt_farms_accounts_reserve_farm_state: pubkeys[23],
            farms_program: pubkeys[24],
        }
    }
}
impl<'info> From<LiquidateObligationAndRedeemReserveCollateralV2Accounts<'_, 'info>>
for [AccountInfo<
    'info,
>; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN] {
    fn from(
        accounts: LiquidateObligationAndRedeemReserveCollateralV2Accounts<'_, 'info>,
    ) -> Self {
        [
            accounts.liquidation_accounts_liquidator.clone(),
            accounts.liquidation_accounts_obligation.clone(),
            accounts.liquidation_accounts_lending_market.clone(),
            accounts.liquidation_accounts_lending_market_authority.clone(),
            accounts.liquidation_accounts_repay_reserve.clone(),
            accounts.liquidation_accounts_repay_reserve_liquidity_mint.clone(),
            accounts.liquidation_accounts_repay_reserve_liquidity_supply.clone(),
            accounts.liquidation_accounts_withdraw_reserve.clone(),
            accounts.liquidation_accounts_withdraw_reserve_liquidity_mint.clone(),
            accounts.liquidation_accounts_withdraw_reserve_collateral_mint.clone(),
            accounts.liquidation_accounts_withdraw_reserve_collateral_supply.clone(),
            accounts.liquidation_accounts_withdraw_reserve_liquidity_supply.clone(),
            accounts
                .liquidation_accounts_withdraw_reserve_liquidity_fee_receiver
                .clone(),
            accounts.liquidation_accounts_user_source_liquidity.clone(),
            accounts.liquidation_accounts_user_destination_collateral.clone(),
            accounts.liquidation_accounts_user_destination_liquidity.clone(),
            accounts.liquidation_accounts_collateral_token_program.clone(),
            accounts.liquidation_accounts_repay_liquidity_token_program.clone(),
            accounts.liquidation_accounts_withdraw_liquidity_token_program.clone(),
            accounts.liquidation_accounts_instruction_sysvar_account.clone(),
            accounts.collateral_farms_accounts_obligation_farm_user_state.clone(),
            accounts.collateral_farms_accounts_reserve_farm_state.clone(),
            accounts.debt_farms_accounts_obligation_farm_user_state.clone(),
            accounts.debt_farms_accounts_reserve_farm_state.clone(),
            accounts.farms_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<
    &'me [AccountInfo<
        'info,
    >; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN],
> for LiquidateObligationAndRedeemReserveCollateralV2Accounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            liquidation_accounts_liquidator: &arr[0],
            liquidation_accounts_obligation: &arr[1],
            liquidation_accounts_lending_market: &arr[2],
            liquidation_accounts_lending_market_authority: &arr[3],
            liquidation_accounts_repay_reserve: &arr[4],
            liquidation_accounts_repay_reserve_liquidity_mint: &arr[5],
            liquidation_accounts_repay_reserve_liquidity_supply: &arr[6],
            liquidation_accounts_withdraw_reserve: &arr[7],
            liquidation_accounts_withdraw_reserve_liquidity_mint: &arr[8],
            liquidation_accounts_withdraw_reserve_collateral_mint: &arr[9],
            liquidation_accounts_withdraw_reserve_collateral_supply: &arr[10],
            liquidation_accounts_withdraw_reserve_liquidity_supply: &arr[11],
            liquidation_accounts_withdraw_reserve_liquidity_fee_receiver: &arr[12],
            liquidation_accounts_user_source_liquidity: &arr[13],
            liquidation_accounts_user_destination_collateral: &arr[14],
            liquidation_accounts_user_destination_liquidity: &arr[15],
            liquidation_accounts_collateral_token_program: &arr[16],
            liquidation_accounts_repay_liquidity_token_program: &arr[17],
            liquidation_accounts_withdraw_liquidity_token_program: &arr[18],
            liquidation_accounts_instruction_sysvar_account: &arr[19],
            collateral_farms_accounts_obligation_farm_user_state: &arr[20],
            collateral_farms_accounts_reserve_farm_state: &arr[21],
            debt_farms_accounts_obligation_farm_user_state: &arr[22],
            debt_farms_accounts_reserve_farm_state: &arr[23],
            farms_program: &arr[24],
        }
    }
}
pub const LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM: [u8; 8] = [
    162,
    161,
    35,
    143,
    30,
    187,
    185,
    103,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidateObligationAndRedeemReserveCollateralV2IxArgs {
    pub liquidity_amount: u64,
    pub min_acceptable_received_liquidity_amount: u64,
    pub max_allowed_ltv_override_percent: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidateObligationAndRedeemReserveCollateralV2IxData(
    pub LiquidateObligationAndRedeemReserveCollateralV2IxArgs,
);
impl From<LiquidateObligationAndRedeemReserveCollateralV2IxArgs>
for LiquidateObligationAndRedeemReserveCollateralV2IxData {
    fn from(args: LiquidateObligationAndRedeemReserveCollateralV2IxArgs) -> Self {
        Self(args)
    }
}
impl LiquidateObligationAndRedeemReserveCollateralV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM
        {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM,
                        maybe_discm
                    ),
                ),
            );
        }
        Ok(
            Self(
                LiquidateObligationAndRedeemReserveCollateralV2IxArgs::deserialize(
                    &mut reader,
                )?,
            ),
        )
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer
            .write_all(&LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: LiquidateObligationAndRedeemReserveCollateralV2Keys,
    args: LiquidateObligationAndRedeemReserveCollateralV2IxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIQUIDATE_OBLIGATION_AND_REDEEM_RESERVE_COLLATERAL_V2_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: LiquidateObligationAndRedeemReserveCollateralV2IxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_v2_ix(
    keys: LiquidateObligationAndRedeemReserveCollateralV2Keys,
    args: LiquidateObligationAndRedeemReserveCollateralV2IxArgs,
) -> std::io::Result<Instruction> {
    liquidate_obligation_and_redeem_reserve_collateral_v2_ix_with_program_id(
        crate::ID,
        keys,
        args,
    )
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateObligationAndRedeemReserveCollateralV2Accounts<'_, '_>,
    args: LiquidateObligationAndRedeemReserveCollateralV2IxArgs,
) -> ProgramResult {
    let keys: LiquidateObligationAndRedeemReserveCollateralV2Keys = accounts.into();
    let ix = liquidate_obligation_and_redeem_reserve_collateral_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_v2_invoke(
    accounts: LiquidateObligationAndRedeemReserveCollateralV2Accounts<'_, '_>,
    args: LiquidateObligationAndRedeemReserveCollateralV2IxArgs,
) -> ProgramResult {
    liquidate_obligation_and_redeem_reserve_collateral_v2_invoke_with_program_id(
        crate::ID,
        accounts,
        args,
    )
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LiquidateObligationAndRedeemReserveCollateralV2Accounts<'_, '_>,
    args: LiquidateObligationAndRedeemReserveCollateralV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LiquidateObligationAndRedeemReserveCollateralV2Keys = accounts.into();
    let ix = liquidate_obligation_and_redeem_reserve_collateral_v2_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_v2_invoke_signed(
    accounts: LiquidateObligationAndRedeemReserveCollateralV2Accounts<'_, '_>,
    args: LiquidateObligationAndRedeemReserveCollateralV2IxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    liquidate_obligation_and_redeem_reserve_collateral_v2_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_v2_verify_account_keys(
    accounts: LiquidateObligationAndRedeemReserveCollateralV2Accounts<'_, '_>,
    keys: LiquidateObligationAndRedeemReserveCollateralV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            *accounts.liquidation_accounts_liquidator.key,
            keys.liquidation_accounts_liquidator,
        ),
        (
            *accounts.liquidation_accounts_obligation.key,
            keys.liquidation_accounts_obligation,
        ),
        (
            *accounts.liquidation_accounts_lending_market.key,
            keys.liquidation_accounts_lending_market,
        ),
        (
            *accounts.liquidation_accounts_lending_market_authority.key,
            keys.liquidation_accounts_lending_market_authority,
        ),
        (
            *accounts.liquidation_accounts_repay_reserve.key,
            keys.liquidation_accounts_repay_reserve,
        ),
        (
            *accounts.liquidation_accounts_repay_reserve_liquidity_mint.key,
            keys.liquidation_accounts_repay_reserve_liquidity_mint,
        ),
        (
            *accounts.liquidation_accounts_repay_reserve_liquidity_supply.key,
            keys.liquidation_accounts_repay_reserve_liquidity_supply,
        ),
        (
            *accounts.liquidation_accounts_withdraw_reserve.key,
            keys.liquidation_accounts_withdraw_reserve,
        ),
        (
            *accounts.liquidation_accounts_withdraw_reserve_liquidity_mint.key,
            keys.liquidation_accounts_withdraw_reserve_liquidity_mint,
        ),
        (
            *accounts.liquidation_accounts_withdraw_reserve_collateral_mint.key,
            keys.liquidation_accounts_withdraw_reserve_collateral_mint,
        ),
        (
            *accounts.liquidation_accounts_withdraw_reserve_collateral_supply.key,
            keys.liquidation_accounts_withdraw_reserve_collateral_supply,
        ),
        (
            *accounts.liquidation_accounts_withdraw_reserve_liquidity_supply.key,
            keys.liquidation_accounts_withdraw_reserve_liquidity_supply,
        ),
        (
            *accounts.liquidation_accounts_withdraw_reserve_liquidity_fee_receiver.key,
            keys.liquidation_accounts_withdraw_reserve_liquidity_fee_receiver,
        ),
        (
            *accounts.liquidation_accounts_user_source_liquidity.key,
            keys.liquidation_accounts_user_source_liquidity,
        ),
        (
            *accounts.liquidation_accounts_user_destination_collateral.key,
            keys.liquidation_accounts_user_destination_collateral,
        ),
        (
            *accounts.liquidation_accounts_user_destination_liquidity.key,
            keys.liquidation_accounts_user_destination_liquidity,
        ),
        (
            *accounts.liquidation_accounts_collateral_token_program.key,
            keys.liquidation_accounts_collateral_token_program,
        ),
        (
            *accounts.liquidation_accounts_repay_liquidity_token_program.key,
            keys.liquidation_accounts_repay_liquidity_token_program,
        ),
        (
            *accounts.liquidation_accounts_withdraw_liquidity_token_program.key,
            keys.liquidation_accounts_withdraw_liquidity_token_program,
        ),
        (
            *accounts.liquidation_accounts_instruction_sysvar_account.key,
            keys.liquidation_accounts_instruction_sysvar_account,
        ),
        (
            *accounts.collateral_farms_accounts_obligation_farm_user_state.key,
            keys.collateral_farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.collateral_farms_accounts_reserve_farm_state.key,
            keys.collateral_farms_accounts_reserve_farm_state,
        ),
        (
            *accounts.debt_farms_accounts_obligation_farm_user_state.key,
            keys.debt_farms_accounts_obligation_farm_user_state,
        ),
        (
            *accounts.debt_farms_accounts_reserve_farm_state.key,
            keys.debt_farms_accounts_reserve_farm_state,
        ),
        (*accounts.farms_program.key, keys.farms_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_v2_verify_writable_privileges<
    'me,
    'info,
>(
    accounts: LiquidateObligationAndRedeemReserveCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.liquidation_accounts_obligation,
        accounts.liquidation_accounts_repay_reserve,
        accounts.liquidation_accounts_repay_reserve_liquidity_supply,
        accounts.liquidation_accounts_withdraw_reserve,
        accounts.liquidation_accounts_withdraw_reserve_collateral_mint,
        accounts.liquidation_accounts_withdraw_reserve_collateral_supply,
        accounts.liquidation_accounts_withdraw_reserve_liquidity_supply,
        accounts.liquidation_accounts_withdraw_reserve_liquidity_fee_receiver,
        accounts.liquidation_accounts_user_source_liquidity,
        accounts.liquidation_accounts_user_destination_collateral,
        accounts.liquidation_accounts_user_destination_liquidity,
        accounts.collateral_farms_accounts_obligation_farm_user_state,
        accounts.collateral_farms_accounts_reserve_farm_state,
        accounts.debt_farms_accounts_obligation_farm_user_state,
        accounts.debt_farms_accounts_reserve_farm_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_v2_verify_signer_privileges<
    'me,
    'info,
>(
    accounts: LiquidateObligationAndRedeemReserveCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.liquidation_accounts_liquidator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn liquidate_obligation_and_redeem_reserve_collateral_v2_verify_account_privileges<
    'me,
    'info,
>(
    accounts: LiquidateObligationAndRedeemReserveCollateralV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    liquidate_obligation_and_redeem_reserve_collateral_v2_verify_writable_privileges(
        accounts,
    )?;
    liquidate_obligation_and_redeem_reserve_collateral_v2_verify_signer_privileges(
        accounts,
    )?;
    Ok(())
}
pub const FLASH_REPAY_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct FlashRepayReserveLiquidityAccounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub reserve_destination_liquidity: &'me AccountInfo<'info>,
    pub user_source_liquidity: &'me AccountInfo<'info>,
    pub reserve_liquidity_fee_receiver: &'me AccountInfo<'info>,
    pub referrer_token_state: &'me AccountInfo<'info>,
    pub referrer_account: &'me AccountInfo<'info>,
    pub sysvar_info: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlashRepayReserveLiquidityKeys {
    pub user_transfer_authority: Pubkey,
    pub lending_market_authority: Pubkey,
    pub lending_market: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_destination_liquidity: Pubkey,
    pub user_source_liquidity: Pubkey,
    pub reserve_liquidity_fee_receiver: Pubkey,
    pub referrer_token_state: Pubkey,
    pub referrer_account: Pubkey,
    pub sysvar_info: Pubkey,
    pub token_program: Pubkey,
}
impl From<FlashRepayReserveLiquidityAccounts<'_, '_>>
for FlashRepayReserveLiquidityKeys {
    fn from(accounts: FlashRepayReserveLiquidityAccounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            lending_market: *accounts.lending_market.key,
            reserve: *accounts.reserve.key,
            reserve_liquidity_mint: *accounts.reserve_liquidity_mint.key,
            reserve_destination_liquidity: *accounts.reserve_destination_liquidity.key,
            user_source_liquidity: *accounts.user_source_liquidity.key,
            reserve_liquidity_fee_receiver: *accounts.reserve_liquidity_fee_receiver.key,
            referrer_token_state: *accounts.referrer_token_state.key,
            referrer_account: *accounts.referrer_account.key,
            sysvar_info: *accounts.sysvar_info.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<FlashRepayReserveLiquidityKeys>
for [AccountMeta; FLASH_REPAY_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: FlashRepayReserveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_fee_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_token_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sysvar_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FLASH_REPAY_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for FlashRepayReserveLiquidityKeys {
    fn from(pubkeys: [Pubkey; FLASH_REPAY_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            lending_market_authority: pubkeys[1],
            lending_market: pubkeys[2],
            reserve: pubkeys[3],
            reserve_liquidity_mint: pubkeys[4],
            reserve_destination_liquidity: pubkeys[5],
            user_source_liquidity: pubkeys[6],
            reserve_liquidity_fee_receiver: pubkeys[7],
            referrer_token_state: pubkeys[8],
            referrer_account: pubkeys[9],
            sysvar_info: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<FlashRepayReserveLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; FLASH_REPAY_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: FlashRepayReserveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.lending_market_authority.clone(),
            accounts.lending_market.clone(),
            accounts.reserve.clone(),
            accounts.reserve_liquidity_mint.clone(),
            accounts.reserve_destination_liquidity.clone(),
            accounts.user_source_liquidity.clone(),
            accounts.reserve_liquidity_fee_receiver.clone(),
            accounts.referrer_token_state.clone(),
            accounts.referrer_account.clone(),
            accounts.sysvar_info.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; FLASH_REPAY_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for FlashRepayReserveLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FLASH_REPAY_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            lending_market_authority: &arr[1],
            lending_market: &arr[2],
            reserve: &arr[3],
            reserve_liquidity_mint: &arr[4],
            reserve_destination_liquidity: &arr[5],
            user_source_liquidity: &arr[6],
            reserve_liquidity_fee_receiver: &arr[7],
            referrer_token_state: &arr[8],
            referrer_account: &arr[9],
            sysvar_info: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const FLASH_REPAY_RESERVE_LIQUIDITY_IX_DISCM: [u8; 8] = [
    185,
    117,
    0,
    203,
    96,
    245,
    180,
    186,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlashRepayReserveLiquidityIxArgs {
    pub liquidity_amount: u64,
    pub borrow_instruction_index: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FlashRepayReserveLiquidityIxData(pub FlashRepayReserveLiquidityIxArgs);
impl From<FlashRepayReserveLiquidityIxArgs> for FlashRepayReserveLiquidityIxData {
    fn from(args: FlashRepayReserveLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl FlashRepayReserveLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FLASH_REPAY_RESERVE_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FLASH_REPAY_RESERVE_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(FlashRepayReserveLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FLASH_REPAY_RESERVE_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn flash_repay_reserve_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: FlashRepayReserveLiquidityKeys,
    args: FlashRepayReserveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FLASH_REPAY_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: FlashRepayReserveLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn flash_repay_reserve_liquidity_ix(
    keys: FlashRepayReserveLiquidityKeys,
    args: FlashRepayReserveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    flash_repay_reserve_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn flash_repay_reserve_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FlashRepayReserveLiquidityAccounts<'_, '_>,
    args: FlashRepayReserveLiquidityIxArgs,
) -> ProgramResult {
    let keys: FlashRepayReserveLiquidityKeys = accounts.into();
    let ix = flash_repay_reserve_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn flash_repay_reserve_liquidity_invoke(
    accounts: FlashRepayReserveLiquidityAccounts<'_, '_>,
    args: FlashRepayReserveLiquidityIxArgs,
) -> ProgramResult {
    flash_repay_reserve_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn flash_repay_reserve_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FlashRepayReserveLiquidityAccounts<'_, '_>,
    args: FlashRepayReserveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FlashRepayReserveLiquidityKeys = accounts.into();
    let ix = flash_repay_reserve_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn flash_repay_reserve_liquidity_invoke_signed(
    accounts: FlashRepayReserveLiquidityAccounts<'_, '_>,
    args: FlashRepayReserveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    flash_repay_reserve_liquidity_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn flash_repay_reserve_liquidity_verify_account_keys(
    accounts: FlashRepayReserveLiquidityAccounts<'_, '_>,
    keys: FlashRepayReserveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_liquidity_mint.key, keys.reserve_liquidity_mint),
        (
            *accounts.reserve_destination_liquidity.key,
            keys.reserve_destination_liquidity,
        ),
        (*accounts.user_source_liquidity.key, keys.user_source_liquidity),
        (
            *accounts.reserve_liquidity_fee_receiver.key,
            keys.reserve_liquidity_fee_receiver,
        ),
        (*accounts.referrer_token_state.key, keys.referrer_token_state),
        (*accounts.referrer_account.key, keys.referrer_account),
        (*accounts.sysvar_info.key, keys.sysvar_info),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn flash_repay_reserve_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: FlashRepayReserveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.reserve,
        accounts.reserve_destination_liquidity,
        accounts.user_source_liquidity,
        accounts.reserve_liquidity_fee_receiver,
        accounts.referrer_token_state,
        accounts.referrer_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn flash_repay_reserve_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: FlashRepayReserveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn flash_repay_reserve_liquidity_verify_account_privileges<'me, 'info>(
    accounts: FlashRepayReserveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    flash_repay_reserve_liquidity_verify_writable_privileges(accounts)?;
    flash_repay_reserve_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FLASH_BORROW_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct FlashBorrowReserveLiquidityAccounts<'me, 'info> {
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub reserve_source_liquidity: &'me AccountInfo<'info>,
    pub user_destination_liquidity: &'me AccountInfo<'info>,
    pub reserve_liquidity_fee_receiver: &'me AccountInfo<'info>,
    pub referrer_token_state: &'me AccountInfo<'info>,
    pub referrer_account: &'me AccountInfo<'info>,
    pub sysvar_info: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlashBorrowReserveLiquidityKeys {
    pub user_transfer_authority: Pubkey,
    pub lending_market_authority: Pubkey,
    pub lending_market: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_source_liquidity: Pubkey,
    pub user_destination_liquidity: Pubkey,
    pub reserve_liquidity_fee_receiver: Pubkey,
    pub referrer_token_state: Pubkey,
    pub referrer_account: Pubkey,
    pub sysvar_info: Pubkey,
    pub token_program: Pubkey,
}
impl From<FlashBorrowReserveLiquidityAccounts<'_, '_>>
for FlashBorrowReserveLiquidityKeys {
    fn from(accounts: FlashBorrowReserveLiquidityAccounts) -> Self {
        Self {
            user_transfer_authority: *accounts.user_transfer_authority.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            lending_market: *accounts.lending_market.key,
            reserve: *accounts.reserve.key,
            reserve_liquidity_mint: *accounts.reserve_liquidity_mint.key,
            reserve_source_liquidity: *accounts.reserve_source_liquidity.key,
            user_destination_liquidity: *accounts.user_destination_liquidity.key,
            reserve_liquidity_fee_receiver: *accounts.reserve_liquidity_fee_receiver.key,
            referrer_token_state: *accounts.referrer_token_state.key,
            referrer_account: *accounts.referrer_account.key,
            sysvar_info: *accounts.sysvar_info.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<FlashBorrowReserveLiquidityKeys>
for [AccountMeta; FLASH_BORROW_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: FlashBorrowReserveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_source_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_fee_receiver,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_token_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sysvar_info,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; FLASH_BORROW_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for FlashBorrowReserveLiquidityKeys {
    fn from(pubkeys: [Pubkey; FLASH_BORROW_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            user_transfer_authority: pubkeys[0],
            lending_market_authority: pubkeys[1],
            lending_market: pubkeys[2],
            reserve: pubkeys[3],
            reserve_liquidity_mint: pubkeys[4],
            reserve_source_liquidity: pubkeys[5],
            user_destination_liquidity: pubkeys[6],
            reserve_liquidity_fee_receiver: pubkeys[7],
            referrer_token_state: pubkeys[8],
            referrer_account: pubkeys[9],
            sysvar_info: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<FlashBorrowReserveLiquidityAccounts<'_, 'info>>
for [AccountInfo<'info>; FLASH_BORROW_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(accounts: FlashBorrowReserveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.user_transfer_authority.clone(),
            accounts.lending_market_authority.clone(),
            accounts.lending_market.clone(),
            accounts.reserve.clone(),
            accounts.reserve_liquidity_mint.clone(),
            accounts.reserve_source_liquidity.clone(),
            accounts.user_destination_liquidity.clone(),
            accounts.reserve_liquidity_fee_receiver.clone(),
            accounts.referrer_token_state.clone(),
            accounts.referrer_account.clone(),
            accounts.sysvar_info.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; FLASH_BORROW_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
for FlashBorrowReserveLiquidityAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; FLASH_BORROW_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            user_transfer_authority: &arr[0],
            lending_market_authority: &arr[1],
            lending_market: &arr[2],
            reserve: &arr[3],
            reserve_liquidity_mint: &arr[4],
            reserve_source_liquidity: &arr[5],
            user_destination_liquidity: &arr[6],
            reserve_liquidity_fee_receiver: &arr[7],
            referrer_token_state: &arr[8],
            referrer_account: &arr[9],
            sysvar_info: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const FLASH_BORROW_RESERVE_LIQUIDITY_IX_DISCM: [u8; 8] = [
    135,
    231,
    52,
    167,
    7,
    52,
    212,
    193,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlashBorrowReserveLiquidityIxArgs {
    pub liquidity_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FlashBorrowReserveLiquidityIxData(pub FlashBorrowReserveLiquidityIxArgs);
impl From<FlashBorrowReserveLiquidityIxArgs> for FlashBorrowReserveLiquidityIxData {
    fn from(args: FlashBorrowReserveLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl FlashBorrowReserveLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FLASH_BORROW_RESERVE_LIQUIDITY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FLASH_BORROW_RESERVE_LIQUIDITY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(FlashBorrowReserveLiquidityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FLASH_BORROW_RESERVE_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn flash_borrow_reserve_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: FlashBorrowReserveLiquidityKeys,
    args: FlashBorrowReserveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FLASH_BORROW_RESERVE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: FlashBorrowReserveLiquidityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn flash_borrow_reserve_liquidity_ix(
    keys: FlashBorrowReserveLiquidityKeys,
    args: FlashBorrowReserveLiquidityIxArgs,
) -> std::io::Result<Instruction> {
    flash_borrow_reserve_liquidity_ix_with_program_id(crate::ID, keys, args)
}
pub fn flash_borrow_reserve_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FlashBorrowReserveLiquidityAccounts<'_, '_>,
    args: FlashBorrowReserveLiquidityIxArgs,
) -> ProgramResult {
    let keys: FlashBorrowReserveLiquidityKeys = accounts.into();
    let ix = flash_borrow_reserve_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn flash_borrow_reserve_liquidity_invoke(
    accounts: FlashBorrowReserveLiquidityAccounts<'_, '_>,
    args: FlashBorrowReserveLiquidityIxArgs,
) -> ProgramResult {
    flash_borrow_reserve_liquidity_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn flash_borrow_reserve_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FlashBorrowReserveLiquidityAccounts<'_, '_>,
    args: FlashBorrowReserveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FlashBorrowReserveLiquidityKeys = accounts.into();
    let ix = flash_borrow_reserve_liquidity_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn flash_borrow_reserve_liquidity_invoke_signed(
    accounts: FlashBorrowReserveLiquidityAccounts<'_, '_>,
    args: FlashBorrowReserveLiquidityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    flash_borrow_reserve_liquidity_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn flash_borrow_reserve_liquidity_verify_account_keys(
    accounts: FlashBorrowReserveLiquidityAccounts<'_, '_>,
    keys: FlashBorrowReserveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.user_transfer_authority.key, keys.user_transfer_authority),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_liquidity_mint.key, keys.reserve_liquidity_mint),
        (*accounts.reserve_source_liquidity.key, keys.reserve_source_liquidity),
        (*accounts.user_destination_liquidity.key, keys.user_destination_liquidity),
        (
            *accounts.reserve_liquidity_fee_receiver.key,
            keys.reserve_liquidity_fee_receiver,
        ),
        (*accounts.referrer_token_state.key, keys.referrer_token_state),
        (*accounts.referrer_account.key, keys.referrer_account),
        (*accounts.sysvar_info.key, keys.sysvar_info),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn flash_borrow_reserve_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: FlashBorrowReserveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.reserve,
        accounts.reserve_source_liquidity,
        accounts.user_destination_liquidity,
        accounts.reserve_liquidity_fee_receiver,
        accounts.referrer_token_state,
        accounts.referrer_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn flash_borrow_reserve_liquidity_verify_signer_privileges<'me, 'info>(
    accounts: FlashBorrowReserveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn flash_borrow_reserve_liquidity_verify_account_privileges<'me, 'info>(
    accounts: FlashBorrowReserveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    flash_borrow_reserve_liquidity_verify_writable_privileges(accounts)?;
    flash_borrow_reserve_liquidity_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REQUEST_ELEVATION_GROUP_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct RequestElevationGroupAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub obligation: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RequestElevationGroupKeys {
    pub owner: Pubkey,
    pub obligation: Pubkey,
    pub lending_market: Pubkey,
}
impl From<RequestElevationGroupAccounts<'_, '_>> for RequestElevationGroupKeys {
    fn from(accounts: RequestElevationGroupAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            obligation: *accounts.obligation.key,
            lending_market: *accounts.lending_market.key,
        }
    }
}
impl From<RequestElevationGroupKeys>
for [AccountMeta; REQUEST_ELEVATION_GROUP_IX_ACCOUNTS_LEN] {
    fn from(keys: RequestElevationGroupKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.obligation,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REQUEST_ELEVATION_GROUP_IX_ACCOUNTS_LEN]>
for RequestElevationGroupKeys {
    fn from(pubkeys: [Pubkey; REQUEST_ELEVATION_GROUP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            obligation: pubkeys[1],
            lending_market: pubkeys[2],
        }
    }
}
impl<'info> From<RequestElevationGroupAccounts<'_, 'info>>
for [AccountInfo<'info>; REQUEST_ELEVATION_GROUP_IX_ACCOUNTS_LEN] {
    fn from(accounts: RequestElevationGroupAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.obligation.clone(),
            accounts.lending_market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REQUEST_ELEVATION_GROUP_IX_ACCOUNTS_LEN]>
for RequestElevationGroupAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; REQUEST_ELEVATION_GROUP_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            owner: &arr[0],
            obligation: &arr[1],
            lending_market: &arr[2],
        }
    }
}
pub const REQUEST_ELEVATION_GROUP_IX_DISCM: [u8; 8] = [
    36,
    119,
    251,
    129,
    34,
    240,
    7,
    147,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequestElevationGroupIxArgs {
    pub elevation_group: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestElevationGroupIxData(pub RequestElevationGroupIxArgs);
impl From<RequestElevationGroupIxArgs> for RequestElevationGroupIxData {
    fn from(args: RequestElevationGroupIxArgs) -> Self {
        Self(args)
    }
}
impl RequestElevationGroupIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REQUEST_ELEVATION_GROUP_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REQUEST_ELEVATION_GROUP_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RequestElevationGroupIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_ELEVATION_GROUP_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn request_elevation_group_ix_with_program_id(
    program_id: Pubkey,
    keys: RequestElevationGroupKeys,
    args: RequestElevationGroupIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REQUEST_ELEVATION_GROUP_IX_ACCOUNTS_LEN] = keys.into();
    let data: RequestElevationGroupIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn request_elevation_group_ix(
    keys: RequestElevationGroupKeys,
    args: RequestElevationGroupIxArgs,
) -> std::io::Result<Instruction> {
    request_elevation_group_ix_with_program_id(crate::ID, keys, args)
}
pub fn request_elevation_group_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RequestElevationGroupAccounts<'_, '_>,
    args: RequestElevationGroupIxArgs,
) -> ProgramResult {
    let keys: RequestElevationGroupKeys = accounts.into();
    let ix = request_elevation_group_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn request_elevation_group_invoke(
    accounts: RequestElevationGroupAccounts<'_, '_>,
    args: RequestElevationGroupIxArgs,
) -> ProgramResult {
    request_elevation_group_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn request_elevation_group_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RequestElevationGroupAccounts<'_, '_>,
    args: RequestElevationGroupIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RequestElevationGroupKeys = accounts.into();
    let ix = request_elevation_group_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn request_elevation_group_invoke_signed(
    accounts: RequestElevationGroupAccounts<'_, '_>,
    args: RequestElevationGroupIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    request_elevation_group_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn request_elevation_group_verify_account_keys(
    accounts: RequestElevationGroupAccounts<'_, '_>,
    keys: RequestElevationGroupKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.obligation.key, keys.obligation),
        (*accounts.lending_market.key, keys.lending_market),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn request_elevation_group_verify_writable_privileges<'me, 'info>(
    accounts: RequestElevationGroupAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.obligation] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn request_elevation_group_verify_signer_privileges<'me, 'info>(
    accounts: RequestElevationGroupAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn request_elevation_group_verify_account_privileges<'me, 'info>(
    accounts: RequestElevationGroupAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    request_elevation_group_verify_writable_privileges(accounts)?;
    request_elevation_group_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_REFERRER_TOKEN_STATE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct InitReferrerTokenStateAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub referrer: &'me AccountInfo<'info>,
    pub referrer_token_state: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitReferrerTokenStateKeys {
    pub payer: Pubkey,
    pub lending_market: Pubkey,
    pub reserve: Pubkey,
    pub referrer: Pubkey,
    pub referrer_token_state: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitReferrerTokenStateAccounts<'_, '_>> for InitReferrerTokenStateKeys {
    fn from(accounts: InitReferrerTokenStateAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            lending_market: *accounts.lending_market.key,
            reserve: *accounts.reserve.key,
            referrer: *accounts.referrer.key,
            referrer_token_state: *accounts.referrer_token_state.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitReferrerTokenStateKeys>
for [AccountMeta; INIT_REFERRER_TOKEN_STATE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitReferrerTokenStateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referrer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.referrer_token_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_REFERRER_TOKEN_STATE_IX_ACCOUNTS_LEN]>
for InitReferrerTokenStateKeys {
    fn from(pubkeys: [Pubkey; INIT_REFERRER_TOKEN_STATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            lending_market: pubkeys[1],
            reserve: pubkeys[2],
            referrer: pubkeys[3],
            referrer_token_state: pubkeys[4],
            rent: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<InitReferrerTokenStateAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_REFERRER_TOKEN_STATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitReferrerTokenStateAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.lending_market.clone(),
            accounts.reserve.clone(),
            accounts.referrer.clone(),
            accounts.referrer_token_state.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INIT_REFERRER_TOKEN_STATE_IX_ACCOUNTS_LEN]>
for InitReferrerTokenStateAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_REFERRER_TOKEN_STATE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            payer: &arr[0],
            lending_market: &arr[1],
            reserve: &arr[2],
            referrer: &arr[3],
            referrer_token_state: &arr[4],
            rent: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const INIT_REFERRER_TOKEN_STATE_IX_DISCM: [u8; 8] = [
    116,
    45,
    66,
    148,
    58,
    13,
    218,
    115,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitReferrerTokenStateIxData;
impl InitReferrerTokenStateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_REFERRER_TOKEN_STATE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INIT_REFERRER_TOKEN_STATE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_REFERRER_TOKEN_STATE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_referrer_token_state_ix_with_program_id(
    program_id: Pubkey,
    keys: InitReferrerTokenStateKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_REFERRER_TOKEN_STATE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitReferrerTokenStateIxData.try_to_vec()?,
    })
}
pub fn init_referrer_token_state_ix(
    keys: InitReferrerTokenStateKeys,
) -> std::io::Result<Instruction> {
    init_referrer_token_state_ix_with_program_id(crate::ID, keys)
}
pub fn init_referrer_token_state_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitReferrerTokenStateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitReferrerTokenStateKeys = accounts.into();
    let ix = init_referrer_token_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_referrer_token_state_invoke(
    accounts: InitReferrerTokenStateAccounts<'_, '_>,
) -> ProgramResult {
    init_referrer_token_state_invoke_with_program_id(crate::ID, accounts)
}
pub fn init_referrer_token_state_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitReferrerTokenStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitReferrerTokenStateKeys = accounts.into();
    let ix = init_referrer_token_state_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_referrer_token_state_invoke_signed(
    accounts: InitReferrerTokenStateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_referrer_token_state_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn init_referrer_token_state_verify_account_keys(
    accounts: InitReferrerTokenStateAccounts<'_, '_>,
    keys: InitReferrerTokenStateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.referrer.key, keys.referrer),
        (*accounts.referrer_token_state.key, keys.referrer_token_state),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_referrer_token_state_verify_writable_privileges<'me, 'info>(
    accounts: InitReferrerTokenStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.payer, accounts.referrer_token_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_referrer_token_state_verify_signer_privileges<'me, 'info>(
    accounts: InitReferrerTokenStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_referrer_token_state_verify_account_privileges<'me, 'info>(
    accounts: InitReferrerTokenStateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_referrer_token_state_verify_writable_privileges(accounts)?;
    init_referrer_token_state_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_USER_METADATA_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitUserMetadataAccounts<'me, 'info> {
    pub owner: &'me AccountInfo<'info>,
    pub fee_payer: &'me AccountInfo<'info>,
    pub user_metadata: &'me AccountInfo<'info>,
    pub referrer_user_metadata: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitUserMetadataKeys {
    pub owner: Pubkey,
    pub fee_payer: Pubkey,
    pub user_metadata: Pubkey,
    pub referrer_user_metadata: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitUserMetadataAccounts<'_, '_>> for InitUserMetadataKeys {
    fn from(accounts: InitUserMetadataAccounts) -> Self {
        Self {
            owner: *accounts.owner.key,
            fee_payer: *accounts.fee_payer.key,
            user_metadata: *accounts.user_metadata.key,
            referrer_user_metadata: *accounts.referrer_user_metadata.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitUserMetadataKeys> for [AccountMeta; INIT_USER_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: InitUserMetadataKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_user_metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_USER_METADATA_IX_ACCOUNTS_LEN]> for InitUserMetadataKeys {
    fn from(pubkeys: [Pubkey; INIT_USER_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: pubkeys[0],
            fee_payer: pubkeys[1],
            user_metadata: pubkeys[2],
            referrer_user_metadata: pubkeys[3],
            rent: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<InitUserMetadataAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_USER_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitUserMetadataAccounts<'_, 'info>) -> Self {
        [
            accounts.owner.clone(),
            accounts.fee_payer.clone(),
            accounts.user_metadata.clone(),
            accounts.referrer_user_metadata.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_USER_METADATA_IX_ACCOUNTS_LEN]>
for InitUserMetadataAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INIT_USER_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            owner: &arr[0],
            fee_payer: &arr[1],
            user_metadata: &arr[2],
            referrer_user_metadata: &arr[3],
            rent: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const INIT_USER_METADATA_IX_DISCM: [u8; 8] = [117, 169, 176, 69, 197, 23, 15, 162];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitUserMetadataIxArgs {
    pub user_lookup_table: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitUserMetadataIxData(pub InitUserMetadataIxArgs);
impl From<InitUserMetadataIxArgs> for InitUserMetadataIxData {
    fn from(args: InitUserMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl InitUserMetadataIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_USER_METADATA_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INIT_USER_METADATA_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitUserMetadataIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_USER_METADATA_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_user_metadata_ix_with_program_id(
    program_id: Pubkey,
    keys: InitUserMetadataKeys,
    args: InitUserMetadataIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_USER_METADATA_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitUserMetadataIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_user_metadata_ix(
    keys: InitUserMetadataKeys,
    args: InitUserMetadataIxArgs,
) -> std::io::Result<Instruction> {
    init_user_metadata_ix_with_program_id(crate::ID, keys, args)
}
pub fn init_user_metadata_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitUserMetadataAccounts<'_, '_>,
    args: InitUserMetadataIxArgs,
) -> ProgramResult {
    let keys: InitUserMetadataKeys = accounts.into();
    let ix = init_user_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_user_metadata_invoke(
    accounts: InitUserMetadataAccounts<'_, '_>,
    args: InitUserMetadataIxArgs,
) -> ProgramResult {
    init_user_metadata_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn init_user_metadata_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitUserMetadataAccounts<'_, '_>,
    args: InitUserMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitUserMetadataKeys = accounts.into();
    let ix = init_user_metadata_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_user_metadata_invoke_signed(
    accounts: InitUserMetadataAccounts<'_, '_>,
    args: InitUserMetadataIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_user_metadata_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn init_user_metadata_verify_account_keys(
    accounts: InitUserMetadataAccounts<'_, '_>,
    keys: InitUserMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.owner.key, keys.owner),
        (*accounts.fee_payer.key, keys.fee_payer),
        (*accounts.user_metadata.key, keys.user_metadata),
        (*accounts.referrer_user_metadata.key, keys.referrer_user_metadata),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_user_metadata_verify_writable_privileges<'me, 'info>(
    accounts: InitUserMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.fee_payer, accounts.user_metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_user_metadata_verify_signer_privileges<'me, 'info>(
    accounts: InitUserMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner, accounts.fee_payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_user_metadata_verify_account_privileges<'me, 'info>(
    accounts: InitUserMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_user_metadata_verify_writable_privileges(accounts)?;
    init_user_metadata_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_REFERRER_FEES_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawReferrerFeesAccounts<'me, 'info> {
    pub referrer: &'me AccountInfo<'info>,
    pub referrer_token_state: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
    pub reserve_liquidity_mint: &'me AccountInfo<'info>,
    pub reserve_supply_liquidity: &'me AccountInfo<'info>,
    pub referrer_token_account: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub lending_market_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawReferrerFeesKeys {
    pub referrer: Pubkey,
    pub referrer_token_state: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_supply_liquidity: Pubkey,
    pub referrer_token_account: Pubkey,
    pub lending_market: Pubkey,
    pub lending_market_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<WithdrawReferrerFeesAccounts<'_, '_>> for WithdrawReferrerFeesKeys {
    fn from(accounts: WithdrawReferrerFeesAccounts) -> Self {
        Self {
            referrer: *accounts.referrer.key,
            referrer_token_state: *accounts.referrer_token_state.key,
            reserve: *accounts.reserve.key,
            reserve_liquidity_mint: *accounts.reserve_liquidity_mint.key,
            reserve_supply_liquidity: *accounts.reserve_supply_liquidity.key,
            referrer_token_account: *accounts.referrer_token_account.key,
            lending_market: *accounts.lending_market.key,
            lending_market_authority: *accounts.lending_market_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<WithdrawReferrerFeesKeys>
for [AccountMeta; WITHDRAW_REFERRER_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawReferrerFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.referrer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_token_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_liquidity_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_supply_liquidity,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_REFERRER_FEES_IX_ACCOUNTS_LEN]>
for WithdrawReferrerFeesKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_REFERRER_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            referrer: pubkeys[0],
            referrer_token_state: pubkeys[1],
            reserve: pubkeys[2],
            reserve_liquidity_mint: pubkeys[3],
            reserve_supply_liquidity: pubkeys[4],
            referrer_token_account: pubkeys[5],
            lending_market: pubkeys[6],
            lending_market_authority: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<WithdrawReferrerFeesAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_REFERRER_FEES_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawReferrerFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.referrer.clone(),
            accounts.referrer_token_state.clone(),
            accounts.reserve.clone(),
            accounts.reserve_liquidity_mint.clone(),
            accounts.reserve_supply_liquidity.clone(),
            accounts.referrer_token_account.clone(),
            accounts.lending_market.clone(),
            accounts.lending_market_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_REFERRER_FEES_IX_ACCOUNTS_LEN]>
for WithdrawReferrerFeesAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; WITHDRAW_REFERRER_FEES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            referrer: &arr[0],
            referrer_token_state: &arr[1],
            reserve: &arr[2],
            reserve_liquidity_mint: &arr[3],
            reserve_supply_liquidity: &arr[4],
            referrer_token_account: &arr[5],
            lending_market: &arr[6],
            lending_market_authority: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const WITHDRAW_REFERRER_FEES_IX_DISCM: [u8; 8] = [
    171,
    118,
    121,
    201,
    233,
    140,
    23,
    228,
];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawReferrerFeesIxData;
impl WithdrawReferrerFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_REFERRER_FEES_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_REFERRER_FEES_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_REFERRER_FEES_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_referrer_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawReferrerFeesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_REFERRER_FEES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawReferrerFeesIxData.try_to_vec()?,
    })
}
pub fn withdraw_referrer_fees_ix(
    keys: WithdrawReferrerFeesKeys,
) -> std::io::Result<Instruction> {
    withdraw_referrer_fees_ix_with_program_id(crate::ID, keys)
}
pub fn withdraw_referrer_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawReferrerFeesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawReferrerFeesKeys = accounts.into();
    let ix = withdraw_referrer_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_referrer_fees_invoke(
    accounts: WithdrawReferrerFeesAccounts<'_, '_>,
) -> ProgramResult {
    withdraw_referrer_fees_invoke_with_program_id(crate::ID, accounts)
}
pub fn withdraw_referrer_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawReferrerFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawReferrerFeesKeys = accounts.into();
    let ix = withdraw_referrer_fees_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_referrer_fees_invoke_signed(
    accounts: WithdrawReferrerFeesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_referrer_fees_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn withdraw_referrer_fees_verify_account_keys(
    accounts: WithdrawReferrerFeesAccounts<'_, '_>,
    keys: WithdrawReferrerFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.referrer.key, keys.referrer),
        (*accounts.referrer_token_state.key, keys.referrer_token_state),
        (*accounts.reserve.key, keys.reserve),
        (*accounts.reserve_liquidity_mint.key, keys.reserve_liquidity_mint),
        (*accounts.reserve_supply_liquidity.key, keys.reserve_supply_liquidity),
        (*accounts.referrer_token_account.key, keys.referrer_token_account),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.lending_market_authority.key, keys.lending_market_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_referrer_fees_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawReferrerFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.referrer,
        accounts.referrer_token_state,
        accounts.reserve,
        accounts.reserve_supply_liquidity,
        accounts.referrer_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_referrer_fees_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawReferrerFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.referrer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_referrer_fees_verify_account_privileges<'me, 'info>(
    accounts: WithdrawReferrerFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_referrer_fees_verify_writable_privileges(accounts)?;
    withdraw_referrer_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct InitReferrerStateAndShortUrlAccounts<'me, 'info> {
    pub referrer: &'me AccountInfo<'info>,
    pub referrer_state: &'me AccountInfo<'info>,
    pub referrer_short_url: &'me AccountInfo<'info>,
    pub referrer_user_metadata: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitReferrerStateAndShortUrlKeys {
    pub referrer: Pubkey,
    pub referrer_state: Pubkey,
    pub referrer_short_url: Pubkey,
    pub referrer_user_metadata: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitReferrerStateAndShortUrlAccounts<'_, '_>>
for InitReferrerStateAndShortUrlKeys {
    fn from(accounts: InitReferrerStateAndShortUrlAccounts) -> Self {
        Self {
            referrer: *accounts.referrer.key,
            referrer_state: *accounts.referrer_state.key,
            referrer_short_url: *accounts.referrer_short_url.key,
            referrer_user_metadata: *accounts.referrer_user_metadata.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitReferrerStateAndShortUrlKeys>
for [AccountMeta; INIT_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN] {
    fn from(keys: InitReferrerStateAndShortUrlKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.referrer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_short_url,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_user_metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN]>
for InitReferrerStateAndShortUrlKeys {
    fn from(
        pubkeys: [Pubkey; INIT_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            referrer: pubkeys[0],
            referrer_state: pubkeys[1],
            referrer_short_url: pubkeys[2],
            referrer_user_metadata: pubkeys[3],
            rent: pubkeys[4],
            system_program: pubkeys[5],
        }
    }
}
impl<'info> From<InitReferrerStateAndShortUrlAccounts<'_, 'info>>
for [AccountInfo<'info>; INIT_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitReferrerStateAndShortUrlAccounts<'_, 'info>) -> Self {
        [
            accounts.referrer.clone(),
            accounts.referrer_state.clone(),
            accounts.referrer_short_url.clone(),
            accounts.referrer_user_metadata.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; INIT_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN]>
for InitReferrerStateAndShortUrlAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INIT_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            referrer: &arr[0],
            referrer_state: &arr[1],
            referrer_short_url: &arr[2],
            referrer_user_metadata: &arr[3],
            rent: &arr[4],
            system_program: &arr[5],
        }
    }
}
pub const INIT_REFERRER_STATE_AND_SHORT_URL_IX_DISCM: [u8; 8] = [
    165,
    19,
    25,
    127,
    100,
    55,
    31,
    90,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitReferrerStateAndShortUrlIxArgs {
    pub short_url: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitReferrerStateAndShortUrlIxData(pub InitReferrerStateAndShortUrlIxArgs);
impl From<InitReferrerStateAndShortUrlIxArgs> for InitReferrerStateAndShortUrlIxData {
    fn from(args: InitReferrerStateAndShortUrlIxArgs) -> Self {
        Self(args)
    }
}
impl InitReferrerStateAndShortUrlIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_REFERRER_STATE_AND_SHORT_URL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INIT_REFERRER_STATE_AND_SHORT_URL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitReferrerStateAndShortUrlIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_REFERRER_STATE_AND_SHORT_URL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_referrer_state_and_short_url_ix_with_program_id(
    program_id: Pubkey,
    keys: InitReferrerStateAndShortUrlKeys,
    args: InitReferrerStateAndShortUrlIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: InitReferrerStateAndShortUrlIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_referrer_state_and_short_url_ix(
    keys: InitReferrerStateAndShortUrlKeys,
    args: InitReferrerStateAndShortUrlIxArgs,
) -> std::io::Result<Instruction> {
    init_referrer_state_and_short_url_ix_with_program_id(crate::ID, keys, args)
}
pub fn init_referrer_state_and_short_url_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitReferrerStateAndShortUrlAccounts<'_, '_>,
    args: InitReferrerStateAndShortUrlIxArgs,
) -> ProgramResult {
    let keys: InitReferrerStateAndShortUrlKeys = accounts.into();
    let ix = init_referrer_state_and_short_url_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction(&ix, accounts)
}
pub fn init_referrer_state_and_short_url_invoke(
    accounts: InitReferrerStateAndShortUrlAccounts<'_, '_>,
    args: InitReferrerStateAndShortUrlIxArgs,
) -> ProgramResult {
    init_referrer_state_and_short_url_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn init_referrer_state_and_short_url_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitReferrerStateAndShortUrlAccounts<'_, '_>,
    args: InitReferrerStateAndShortUrlIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitReferrerStateAndShortUrlKeys = accounts.into();
    let ix = init_referrer_state_and_short_url_ix_with_program_id(
        program_id,
        keys,
        args,
    )?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_referrer_state_and_short_url_invoke_signed(
    accounts: InitReferrerStateAndShortUrlAccounts<'_, '_>,
    args: InitReferrerStateAndShortUrlIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_referrer_state_and_short_url_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn init_referrer_state_and_short_url_verify_account_keys(
    accounts: InitReferrerStateAndShortUrlAccounts<'_, '_>,
    keys: InitReferrerStateAndShortUrlKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.referrer.key, keys.referrer),
        (*accounts.referrer_state.key, keys.referrer_state),
        (*accounts.referrer_short_url.key, keys.referrer_short_url),
        (*accounts.referrer_user_metadata.key, keys.referrer_user_metadata),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_referrer_state_and_short_url_verify_writable_privileges<'me, 'info>(
    accounts: InitReferrerStateAndShortUrlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.referrer,
        accounts.referrer_state,
        accounts.referrer_short_url,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_referrer_state_and_short_url_verify_signer_privileges<'me, 'info>(
    accounts: InitReferrerStateAndShortUrlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.referrer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_referrer_state_and_short_url_verify_account_privileges<'me, 'info>(
    accounts: InitReferrerStateAndShortUrlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_referrer_state_and_short_url_verify_writable_privileges(accounts)?;
    init_referrer_state_and_short_url_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DELETE_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct DeleteReferrerStateAndShortUrlAccounts<'me, 'info> {
    pub referrer: &'me AccountInfo<'info>,
    pub referrer_state: &'me AccountInfo<'info>,
    pub short_url: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeleteReferrerStateAndShortUrlKeys {
    pub referrer: Pubkey,
    pub referrer_state: Pubkey,
    pub short_url: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl From<DeleteReferrerStateAndShortUrlAccounts<'_, '_>>
for DeleteReferrerStateAndShortUrlKeys {
    fn from(accounts: DeleteReferrerStateAndShortUrlAccounts) -> Self {
        Self {
            referrer: *accounts.referrer.key,
            referrer_state: *accounts.referrer_state.key,
            short_url: *accounts.short_url.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<DeleteReferrerStateAndShortUrlKeys>
for [AccountMeta; DELETE_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN] {
    fn from(keys: DeleteReferrerStateAndShortUrlKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.referrer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.short_url,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DELETE_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN]>
for DeleteReferrerStateAndShortUrlKeys {
    fn from(
        pubkeys: [Pubkey; DELETE_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            referrer: pubkeys[0],
            referrer_state: pubkeys[1],
            short_url: pubkeys[2],
            rent: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<DeleteReferrerStateAndShortUrlAccounts<'_, 'info>>
for [AccountInfo<'info>; DELETE_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN] {
    fn from(accounts: DeleteReferrerStateAndShortUrlAccounts<'_, 'info>) -> Self {
        [
            accounts.referrer.clone(),
            accounts.referrer_state.clone(),
            accounts.short_url.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; DELETE_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN]>
for DeleteReferrerStateAndShortUrlAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DELETE_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            referrer: &arr[0],
            referrer_state: &arr[1],
            short_url: &arr[2],
            rent: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const DELETE_REFERRER_STATE_AND_SHORT_URL_IX_DISCM: [u8; 8] = [
    153,
    185,
    99,
    28,
    228,
    179,
    187,
    150,
];
#[derive(Clone, Debug, PartialEq)]
pub struct DeleteReferrerStateAndShortUrlIxData;
impl DeleteReferrerStateAndShortUrlIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELETE_REFERRER_STATE_AND_SHORT_URL_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DELETE_REFERRER_STATE_AND_SHORT_URL_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELETE_REFERRER_STATE_AND_SHORT_URL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn delete_referrer_state_and_short_url_ix_with_program_id(
    program_id: Pubkey,
    keys: DeleteReferrerStateAndShortUrlKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DELETE_REFERRER_STATE_AND_SHORT_URL_IX_ACCOUNTS_LEN] = keys
        .into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DeleteReferrerStateAndShortUrlIxData.try_to_vec()?,
    })
}
pub fn delete_referrer_state_and_short_url_ix(
    keys: DeleteReferrerStateAndShortUrlKeys,
) -> std::io::Result<Instruction> {
    delete_referrer_state_and_short_url_ix_with_program_id(crate::ID, keys)
}
pub fn delete_referrer_state_and_short_url_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeleteReferrerStateAndShortUrlAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DeleteReferrerStateAndShortUrlKeys = accounts.into();
    let ix = delete_referrer_state_and_short_url_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn delete_referrer_state_and_short_url_invoke(
    accounts: DeleteReferrerStateAndShortUrlAccounts<'_, '_>,
) -> ProgramResult {
    delete_referrer_state_and_short_url_invoke_with_program_id(crate::ID, accounts)
}
pub fn delete_referrer_state_and_short_url_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeleteReferrerStateAndShortUrlAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeleteReferrerStateAndShortUrlKeys = accounts.into();
    let ix = delete_referrer_state_and_short_url_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn delete_referrer_state_and_short_url_invoke_signed(
    accounts: DeleteReferrerStateAndShortUrlAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    delete_referrer_state_and_short_url_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        seeds,
    )
}
pub fn delete_referrer_state_and_short_url_verify_account_keys(
    accounts: DeleteReferrerStateAndShortUrlAccounts<'_, '_>,
    keys: DeleteReferrerStateAndShortUrlKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.referrer.key, keys.referrer),
        (*accounts.referrer_state.key, keys.referrer_state),
        (*accounts.short_url.key, keys.short_url),
        (*accounts.rent.key, keys.rent),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn delete_referrer_state_and_short_url_verify_writable_privileges<'me, 'info>(
    accounts: DeleteReferrerStateAndShortUrlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.referrer,
        accounts.referrer_state,
        accounts.short_url,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn delete_referrer_state_and_short_url_verify_signer_privileges<'me, 'info>(
    accounts: DeleteReferrerStateAndShortUrlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.referrer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn delete_referrer_state_and_short_url_verify_account_privileges<'me, 'info>(
    accounts: DeleteReferrerStateAndShortUrlAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    delete_referrer_state_and_short_url_verify_writable_privileges(accounts)?;
    delete_referrer_state_and_short_url_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const IDL_MISSING_TYPES_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct IdlMissingTypesAccounts<'me, 'info> {
    pub lending_market_owner: &'me AccountInfo<'info>,
    pub lending_market: &'me AccountInfo<'info>,
    pub reserve: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IdlMissingTypesKeys {
    pub lending_market_owner: Pubkey,
    pub lending_market: Pubkey,
    pub reserve: Pubkey,
}
impl From<IdlMissingTypesAccounts<'_, '_>> for IdlMissingTypesKeys {
    fn from(accounts: IdlMissingTypesAccounts) -> Self {
        Self {
            lending_market_owner: *accounts.lending_market_owner.key,
            lending_market: *accounts.lending_market.key,
            reserve: *accounts.reserve.key,
        }
    }
}
impl From<IdlMissingTypesKeys> for [AccountMeta; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN] {
    fn from(keys: IdlMissingTypesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.lending_market_owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lending_market,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN]> for IdlMissingTypesKeys {
    fn from(pubkeys: [Pubkey; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market_owner: pubkeys[0],
            lending_market: pubkeys[1],
            reserve: pubkeys[2],
        }
    }
}
impl<'info> From<IdlMissingTypesAccounts<'_, 'info>>
for [AccountInfo<'info>; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN] {
    fn from(accounts: IdlMissingTypesAccounts<'_, 'info>) -> Self {
        [
            accounts.lending_market_owner.clone(),
            accounts.lending_market.clone(),
            accounts.reserve.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN]>
for IdlMissingTypesAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            lending_market_owner: &arr[0],
            lending_market: &arr[1],
            reserve: &arr[2],
        }
    }
}
pub const IDL_MISSING_TYPES_IX_DISCM: [u8; 8] = [130, 80, 38, 153, 80, 212, 182, 253];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdlMissingTypesIxArgs {
    pub reserve_farm_kind: ReserveFarmKind,
    pub asset_tier: AssetTier,
    pub fee_calculation: FeeCalculation,
    pub reserve_status: ReserveStatus,
    pub update_config_mode: UpdateConfigMode,
    pub update_lending_market_config_value: UpdateLendingMarketConfigValue,
    pub update_lending_market_config_mode: UpdateLendingMarketMode,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IdlMissingTypesIxData(pub IdlMissingTypesIxArgs);
impl From<IdlMissingTypesIxArgs> for IdlMissingTypesIxData {
    fn from(args: IdlMissingTypesIxArgs) -> Self {
        Self(args)
    }
}
impl IdlMissingTypesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != IDL_MISSING_TYPES_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        IDL_MISSING_TYPES_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(IdlMissingTypesIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&IDL_MISSING_TYPES_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn idl_missing_types_ix_with_program_id(
    program_id: Pubkey,
    keys: IdlMissingTypesKeys,
    args: IdlMissingTypesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; IDL_MISSING_TYPES_IX_ACCOUNTS_LEN] = keys.into();
    let data: IdlMissingTypesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn idl_missing_types_ix(
    keys: IdlMissingTypesKeys,
    args: IdlMissingTypesIxArgs,
) -> std::io::Result<Instruction> {
    idl_missing_types_ix_with_program_id(crate::ID, keys, args)
}
pub fn idl_missing_types_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IdlMissingTypesAccounts<'_, '_>,
    args: IdlMissingTypesIxArgs,
) -> ProgramResult {
    let keys: IdlMissingTypesKeys = accounts.into();
    let ix = idl_missing_types_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn idl_missing_types_invoke(
    accounts: IdlMissingTypesAccounts<'_, '_>,
    args: IdlMissingTypesIxArgs,
) -> ProgramResult {
    idl_missing_types_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn idl_missing_types_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IdlMissingTypesAccounts<'_, '_>,
    args: IdlMissingTypesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IdlMissingTypesKeys = accounts.into();
    let ix = idl_missing_types_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn idl_missing_types_invoke_signed(
    accounts: IdlMissingTypesAccounts<'_, '_>,
    args: IdlMissingTypesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    idl_missing_types_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn idl_missing_types_verify_account_keys(
    accounts: IdlMissingTypesAccounts<'_, '_>,
    keys: IdlMissingTypesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.lending_market_owner.key, keys.lending_market_owner),
        (*accounts.lending_market.key, keys.lending_market),
        (*accounts.reserve.key, keys.reserve),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn idl_missing_types_verify_writable_privileges<'me, 'info>(
    accounts: IdlMissingTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.reserve] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn idl_missing_types_verify_signer_privileges<'me, 'info>(
    accounts: IdlMissingTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.lending_market_owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn idl_missing_types_verify_account_privileges<'me, 'info>(
    accounts: IdlMissingTypesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    idl_missing_types_verify_writable_privileges(accounts)?;
    idl_missing_types_verify_signer_privileges(accounts)?;
    Ok(())
}
