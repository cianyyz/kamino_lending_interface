use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::*;
pub const USER_STATE_ACCOUNT_DISCM: [u8; 8] = [72, 177, 85, 249, 76, 167, 186, 126];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserState {
    pub user_id: u64,
    pub farm_state: Pubkey,
    pub owner: Pubkey,
    pub is_farm_delegated: u8,
    pub padding0: [u8; 7],
    pub rewards_tally_scaled: [u128; 10],
    pub rewards_issued_unclaimed: [u64; 10],
    pub last_claim_ts: [u64; 10],
    pub active_stake_scaled: u128,
    pub pending_deposit_stake_scaled: u128,
    pub pending_deposit_stake_ts: u64,
    pub pending_withdrawal_unstake_scaled: u128,
    pub pending_withdrawal_unstake_ts: u64,
    pub bump: u64,
    pub delegatee: Pubkey,
    pub last_stake_ts: u64,
    pub padding1: [u64; 50],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserStateAccount(pub UserState);
impl UserStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_STATE_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        USER_STATE_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UserState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const LENDING_MARKET_ACCOUNT_DISCM: [u8; 8] = [246, 114, 50, 98, 72, 157, 28, 120];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LendingMarket {
    pub version: u64,
    pub bump_seed: u64,
    pub lending_market_owner: Pubkey,
    pub lending_market_owner_cached: Pubkey,
    pub quote_currency: [u8; 32],
    pub referral_fee_bps: u16,
    pub emergency_mode: u8,
    pub autodeleverage_enabled: u8,
    pub borrow_disabled: u8,
    pub price_refresh_trigger_to_max_age_pct: u8,
    pub liquidation_max_debt_close_factor_pct: u8,
    pub insolvency_risk_unhealthy_ltv_pct: u8,
    pub min_full_liquidation_value_threshold: u64,
    pub max_liquidatable_debt_market_value_at_once: u64,
    pub reserved0: [u8; 8],
    pub global_allowed_borrow_value: u64,
    pub risk_council: Pubkey,
    pub reserved1: [u8; 8],
    pub elevation_groups: [ElevationGroup; 32],
    pub elevation_group_padding: [u64; 90],
    pub min_net_value_in_obligation_sf: u128,
    pub min_value_skip_liquidation_ltv_checks: u64,
    pub name: [u8; 32],
    pub min_value_skip_liquidation_bf_checks: u64,
    pub individual_autodeleverage_margin_call_period_secs: u64,
    pub padding1: [u64; 171],
}
#[derive(Clone, Debug, PartialEq)]
pub struct LendingMarketAccount(pub LendingMarket);
impl LendingMarketAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LENDING_MARKET_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        LENDING_MARKET_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(LendingMarket::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LENDING_MARKET_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const OBLIGATION_ACCOUNT_DISCM: [u8; 8] = [168, 206, 141, 106, 88, 76, 172, 167];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Obligation {
    pub tag: u64,
    pub last_update: LastUpdate,
    pub lending_market: Pubkey,
    pub owner: Pubkey,
    pub deposits: [ObligationCollateral; 8],
    pub lowest_reserve_deposit_liquidation_ltv: u64,
    pub deposited_value_sf: u128,
    pub borrows: [ObligationLiquidity; 5],
    pub borrow_factor_adjusted_debt_value_sf: u128,
    pub borrowed_assets_market_value_sf: u128,
    pub allowed_borrow_value_sf: u128,
    pub unhealthy_borrow_value_sf: u128,
    pub deposits_asset_tiers: [u8; 8],
    pub borrows_asset_tiers: [u8; 5],
    pub elevation_group: u8,
    pub num_of_obsolete_reserves: u8,
    pub has_debt: u8,
    pub referrer: Pubkey,
    pub borrowing_disabled: u8,
    pub autodeleverage_target_ltv_pct: u8,
    pub lowest_reserve_deposit_max_ltv_pct: u8,
    pub reserved: [u8; 5],
    pub highest_borrow_factor_pct: u64,
    pub autodeleverage_margin_call_started_timestamp: u64,
    pub padding3: [u64; 125],
}
#[derive(Clone, Debug, PartialEq)]
pub struct ObligationAccount(pub Obligation);
impl ObligationAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OBLIGATION_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        OBLIGATION_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Obligation::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OBLIGATION_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REFERRER_STATE_ACCOUNT_DISCM: [u8; 8] = [194, 81, 217, 103, 12, 19, 12, 66];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReferrerState {
    pub short_url: Pubkey,
    pub owner: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReferrerStateAccount(pub ReferrerState);
impl ReferrerStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFERRER_STATE_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REFERRER_STATE_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ReferrerState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFERRER_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const REFERRER_TOKEN_STATE_ACCOUNT_DISCM: [u8; 8] = [
    39,
    15,
    208,
    77,
    32,
    195,
    105,
    56,
];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReferrerTokenState {
    pub referrer: Pubkey,
    pub mint: Pubkey,
    pub amount_unclaimed_sf: u128,
    pub amount_cumulative_sf: u128,
    pub bump: u64,
    pub padding: [u64; 31],
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReferrerTokenStateAccount(pub ReferrerTokenState);
impl ReferrerTokenStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFERRER_TOKEN_STATE_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REFERRER_TOKEN_STATE_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ReferrerTokenState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFERRER_TOKEN_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const SHORT_URL_ACCOUNT_DISCM: [u8; 8] = [28, 89, 174, 25, 226, 124, 126, 212];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShortUrl {
    pub referrer: Pubkey,
    pub short_url: String,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ShortUrlAccount(pub ShortUrl);
impl ShortUrlAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHORT_URL_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SHORT_URL_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ShortUrl::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHORT_URL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const USER_METADATA_ACCOUNT_DISCM: [u8; 8] = [157, 214, 220, 235, 98, 135, 171, 28];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserMetadata {
    pub referrer: Pubkey,
    pub bump: u64,
    pub user_lookup_table: Pubkey,
    pub owner: Pubkey,
    pub padding1: [u64; 51],
    pub padding2: [u64; 64],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserMetadataAccount(pub UserMetadata);
impl UserMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != USER_METADATA_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        USER_METADATA_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UserMetadata::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&USER_METADATA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const RESERVE_ACCOUNT_DISCM: [u8; 8] = [43, 242, 204, 202, 26, 247, 59, 127];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reserve {
    pub version: u64,
    pub last_update: LastUpdate,
    pub lending_market: Pubkey,
    pub farm_collateral: Pubkey,
    pub farm_debt: Pubkey,
    pub liquidity: ReserveLiquidity,
    pub reserve_liquidity_padding: [u64; 150],
    pub collateral: ReserveCollateral,
    pub reserve_collateral_padding: [u64; 150],
    pub config: ReserveConfig,
    pub config_padding: [u64; 116],
    pub borrowed_amount_outside_elevation_group: u64,
    pub borrowed_amounts_against_this_reserve_in_elevation_groups: [u64; 32],
    pub padding: [u64; 207],
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReserveAccount(pub Reserve);
impl ReserveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RESERVE_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        RESERVE_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Reserve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RESERVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
