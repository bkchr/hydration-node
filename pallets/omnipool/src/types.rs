use super::*;
use frame_support::{
	pallet_prelude::*,
	traits::{fungible, tokens::BalanceConversion},
};
use sp_runtime::{traits::Convert, FixedPointNumber, FixedPointOperand, FixedU128};

pub type Price = FixedU128;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct AssetState<Balance> {
	/// Quantity of asset in omnipool
	pub(super) reserve: Balance,
	/// Quantity of Hub Asset matching this asset
	pub(super) hub_reserve: Balance,
	/// Quantity of LP shares for this asset
	pub(super) shares: Balance,
	/// Quantity of LP shares for this asset owned by protocol
	pub(super) protocol_shares: Balance,
	/// TVL of asset
	pub(super) tvl: Balance,
}

/// Position representation
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct PositionId<InstanceId>(InstanceId);

/// Position in Omnipool represents a moment when LP provided liquidity of an asset at that moment’s price.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct Position<Balance, AssetId> {
	/// Provided Asset
	pub(super) asset_id: AssetId,
	/// Amount of asset added to omnipool
	pub(super) amount: Balance,
	/// Quantity of LP shares owned by LP
	pub(super) shares: Balance,
	/// Price at which liquidity was provided
	pub(super) price: Balance,
}

// Using FixedU128 to represent a price which uses u128 as inner type, so let's convert `Balance` into FixedU128
impl<Balance, AssetId> Position<Balance, AssetId>
where
	Balance: Clone + From<u128> + Into<u128>,
{
	fn fixed_price(&self) -> Price {
		Price::from_inner(self.price.clone().into())
	}

	fn price_to_balance(price: Price) -> Balance {
		price.into_inner().into()
	}
}
