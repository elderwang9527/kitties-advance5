#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::traits::{Currency, ExistenceRequirement, Randomness, ReservableCurrency};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, One};

	// 转移到runtime里
	// type KittyIndex = Config::KittyIndex;

	// #[pallet::type_value]
	// pub fn GetDefaultValue() -> KittyIndex {
	// 	0_u32
	// }

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	pub struct Kitty(pub [u8; 16]);

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		// 下面这行是说，这个关联类型KittyIndex需要实现下面的很多trait。 这语法是一个trait bound。
		// 而runtime里的 type KittyIndex = u32;是实际上让此type KittyIndex等于u32这个类型。
		// 总结就是在pallet里用关联类型先定义好一个类型，此时只有类型约束。最终类型是什么样，要靠runtime里来实现。

		type KittyIndex: Copy
			+ Member
			+ Default
			+ MaxEncodedLen
			+ Bounded
			+ AtLeast32BitUnsigned
			+ Parameter;

		type KtReserve: Get<BalanceOf<Self>>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		type MaxLength: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T: Config> = StorageValue<_, T::KittyIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn all_kts_owned)]
	pub type AllKtsOwned<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<Kitty, T::MaxLength>, ValueQuery>;

	// 存储正在销售的kittyid 及价格
	#[pallet::storage]
	#[pallet::getter(fn kitties_list_for_sales)]
	pub type KittiesShop<T: Config> =
		StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<BalanceOf<T>>, ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(T::AccountId, T::KittyIndex, Kitty),
		KittyBred(T::AccountId, T::KittyIndex, Kitty),
		KittyTransferred(T::AccountId, T::AccountId, T::KittyIndex),
		KittyInSell(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		NotOwner,
		SameKittyId,
		KittiesCountOverflow,
		TokenNotEnough,
		ExceedMaxKittyOwned,
		NoBuySelf,
		NotForSale,
		NotEnoughBalance,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			T::Currency::reserve(&who, T::KtReserve::get())
				.map_err(|_| Error::<T>::TokenNotEnough)?;

			let dna = Self::random_value(&who);
			let kitty = Kitty(dna);

			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKittyId::<T>::put(kitty_id + One::one());

			AllKtsOwned::<T>::try_mutate(&who, |kitty_vec| kitty_vec.try_push(kitty.clone()))
				.map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;

			Self::deposit_event(Event::KittyCreated(who, kitty_id, kitty));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: T::KittyIndex,
			kitty_id_2: T::KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			T::Currency::reserve(&who, T::KtReserve::get())
				.map_err(|_| Error::<T>::TokenNotEnough)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
			let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;

			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			let selector = Self::random_value(&who);

			let mut data = [0u8; 16];
			for i in 0..kitty_1.0.len() {
				data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
			}
			let new_kitty = Kitty(data);

			<Kitties<T>>::insert(kitty_id, &new_kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKittyId::<T>::put(kitty_id + One::one());

			AllKtsOwned::<T>::try_mutate(&who, |kitty_vec| kitty_vec.try_push(new_kitty.clone()))
				.map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;

			Self::deposit_event(Event::KittyCreated(who, kitty_id, new_kitty));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let prev_owner = ensure_signed(origin)?;

			let exsit_kitty = Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;

			ensure!(Self::kitty_owner(kitty_id) == Some(prev_owner.clone()), Error::<T>::NotOwner);

			T::Currency::reserve(&new_owner, T::KtReserve::get())
				.map_err(|_| Error::<T>::TokenNotEnough)?;

			AllKtsOwned::<T>::try_mutate(&prev_owner, |owned| {
				if let Some(index) = owned.iter().position(|kitty| kitty == &exsit_kitty) {
					owned.swap_remove(index);
					return Ok(());
				}
				Err(())
			})
			.map_err(|_| <Error<T>>::NotOwner)?;

			T::Currency::unreserve(&prev_owner, T::KtReserve::get());

			<KittyOwner<T>>::insert(kitty_id, &new_owner);

			AllKtsOwned::<T>::try_mutate(&new_owner, |vec| vec.try_push(exsit_kitty))
				.map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;

			Self::deposit_event(Event::KittyTransferred(prev_owner, new_owner, kitty_id));

			Ok(())
		}


		#[pallet::weight(1_000)]
		pub fn sell(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			price: Option<BalanceOf<T>>,
		) -> DispatchResultWithPostInfo {
			let seller = ensure_signed(origin)?;
			// 验证操作者是否为拥有者
			ensure!(Self::kitty_owner(kitty_id) == Some(seller.clone()), Error::<T>::NotOwner);
			// 给kitty设定价格，并保存关联有关系
			KittiesShop::<T>::mutate_exists(kitty_id, |p| *p = Some(price));

			Self::deposit_event(Event::KittyInSell(seller, kitty_id, price));
			Ok(().into())
		}

		#[pallet::weight(1_000)]
		pub fn buy(origin: OriginFor<T>, kitty_id: T::KittyIndex) -> DispatchResultWithPostInfo {
			let buyer = ensure_signed(origin)?;
			// 根据ID获取kitty所有者
			let seller = KittyOwner::<T>::get(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			// 验证购买者是否为拥有者
			ensure!(Some(buyer.clone()) != Some(seller.clone()), Error::<T>::NoBuySelf);
			// 获取kitty价格
			let price = KittiesShop::<T>::get(kitty_id).ok_or(Error::<T>::NotForSale)?;
			// 获取买家账户余额
			let buyer_balance = T::Currency::free_balance(&buyer);
			// 获取需要质押的金额配置
			let stake_amount = T::KtReserve::get();
			// 检查买家的余额是否足够用于购买和质押
			ensure!(buyer_balance > (price + stake_amount), Error::<T>::NotEnoughBalance);
			// 获取要质押的数量
			let stake_amount = T::KtReserve::get();
			// 买家质押指定的资产数量
			T::Currency::reserve(&buyer, stake_amount).map_err(|_| Error::<T>::NotEnoughBalance)?;
			// 卖家解除质押数量
			T::Currency::unreserve(&seller, stake_amount);
			// 买家支付相应价格的token数给卖家
			T::Currency::transfer(&buyer, &seller, price, ExistenceRequirement::KeepAlive)?;
			// 更新kitty所有者
			KittyOwner::<T>::insert(kitty_id, buyer.clone());
			// 通告事件
			Self::deposit_event(Event::KittyTransferred(seller, buyer, kitty_id));
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);

			payload.using_encoded(blake2_128)
		}

		fn get_next_id() -> Result<T::KittyIndex, DispatchError> {
			let kitty_id = Self::next_kitty_id();
			if kitty_id == T::KittyIndex::max_value() {
				return Err(Error::<T>::KittiesCountOverflow.into());
			}
			Ok(kitty_id)
		}

		fn get_kitty(kitty_id: T::KittyIndex) -> Result<Kitty, ()> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty),
				None => Err(()),
			}
		}
	}
}
