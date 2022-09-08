// aa21,这段代码望填了，填好后编译成功。
// aa22，心得分享，写pallet很可能困惑，因为用了很多宏，而且event，error等都是由框架定义好的。所以最终形成什么代码逻辑不太清楚。
// 所以可以用rust工具expand，如在template下运行cargo expand，就可以把宏展开，了解底层代码dddi
// aa23,按上个步骤所说，在template 模块下使用cargo expand >expand.rs 生成了expand.rs文件，很难看懂dddf。
#![cfg_attr(not(feature = "std"), no_std)]
// aa2，0355，粘贴进pallet通用骨架代码
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	// aa16,定义randomness trait，之后编译报错
	use frame_support::traits::Randomness;
	// aa18，定义blake2_128,此时编译成功，但注意只是kitties pallet编译成功，还未在根目录编译。
	use sp_io::hashing::blake2_128;

	// aa3，标识每个kitty
	type KittyIndex = u32;

	// aa5，0522，应该不用写这段代码，只是为了展示。
	#[pallet::type_value]
	// add220906,虽然这里函数名和StorageValue的范型参数同名，但它们不是同一个东西，cargo expand后pub fn GetDefaultValue()
	// 实际上是__type_value_for_get_default_value
	pub fn GetDefaultValue() -> KittyIndex {
		0_u32
	}

	// aa6,定义好了id，再定义类型存储kt的数据，为每个kitty生成16个单位长度的u8数组。
	// add220906,为什么需要derive这么多特征，教程没有说dddf。
	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	pub struct Kitty(pub [u8; 16]);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		// aa14，2005，因为pallet会抛出些event，所以他的event也需要些定义，所以这里定义event。
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// aa15，2022，用到random方法，此时编译会出错，显示没在cargotoml里导入包
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// aa4,对kitties建立存储，也就是说存储kt时，下个kt的id是多少，一般以0为第一个kt，所以这里有GetDefaultValue
	// add220904，StorageValue定义如下
	// pub struct StorageValue<Prefix, Value, QueryKind = OptionQuery, OnEmpty = GetDefault>(
	// 	core::marker::PhantomData<(Prefix, Value, QueryKind, OnEmpty)>,
	// );
	// QueryKind = OptionQuery代表如果没有此参数默认为OptionQuery，查询不到值返回None，这里是ValueQuery，代表如果查询不到值，返回此类型默认值，
	// 即用最后的参数GetDefaultValue查询。
	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	// add220907，这里用到了完全限定语法，<Atype as Atrait>::get(),表明这个get()方法是Atype对Atrait的get方法实现，而不是Atype对Btrait的同名方法实现或Atype本身对同名方法实现。
	// 这里Atype代表NextKittyId<T>，Atrait代表frame_support::storage::StorageValue<KittyIndex>。
	// 注意这里是frame_support::storage::StorageValue，是个trait，而frame_support::storage::types::StorageValue是个struct。
	// add220906，展开后如下
	// impl<T: Config> Pallet<T> {
	//     pub fn next_kitty_id() -> KittyIndex {
	//         <NextKittyId<T> as frame_support::storage::StorageValue<KittyIndex>>::get()
	//     }
	// }
	pub type NextKittyId<T> = StorageValue<_, KittyIndex, ValueQuery, GetDefaultValue>;

	// aa7，0807，定义map来保存aa6中的数据
	// add220906，虽然右边没有T，但是左边还是需要加上T，因为右边展开后也有T的。如下：
	// pub type Kitties<T> =
	//     StorageMap<_GeneratedPrefixForStorageKitties<T>, Blake2_128Concat, KittyIndex, Kitty>;
	// add220907,l10.此宏不仅生成了如上pub type Kitties<T>的定义，还生成了如下fn kitties的定义：
	// impl<T: Config> Pallet<T> {
	//     pub fn kitties<KArg>(k: KArg) -> Option<Kitty>
	//     where
	//         KArg: frame_support::codec::EncodeLike<KittyIndex>,
	//     {
	//         <Kitties<T> as frame_support::storage::StorageMap<KittyIndex, Kitty>>::get(k)
	//     }
	// }
	// 总结一下此宏的意思是简化了Kitties<T>的定义，且生成了一个getter(fn kitties)，即针对类型kitties的kitties getter函数，根据KittyIndex，找到Option<Kitty>，dddi
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyIndex, Kitty>;

	// aa8,0911,对kt进行transfer，transfer之前需要知道所有者，以及之后所有者更新为谁
	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyIndex, T::AccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// aa12，1930，定义event和error。
		KittyCreated(T::AccountId, KittyIndex, Kitty),
		KittyBred(T::AccountId, KittyIndex, Kitty),
		KittyTransferred(T::AccountId, T::AccountId, KittyIndex),
	}

	#[pallet::error]
	pub enum Error<T> {
		// aa13,1945,errors
		InvalidKittyId,
		NotOwner,
		SameKittyId,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// aa12，1310，写完辅助方法，看看call里面怎么写方法。第一个方法，创建一个kt。let who * 是做一个check签名？？？？。这是所有对外extrinsic需要做的？？？？dddf
		// 因为交易需要自己的签名
		// get_next_id是创建时看下下一个id是什么，
		// 1420 Kitties::*，以及后两行都是对链上的操作
		// 最后抛出event，这此event由deposit_event方法存储ddda，此event后面会定义。
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			// add220907，通过这一步后，who就应该确定是交易发送方了。
			let who = ensure_signed(origin)?;
			// add220907,以下为map_err解释，具体代码涉及到闭包，以后再了解dddf。
			// 此方法仅作用于 Result 类型，并允许将失败的值从 E 转换为其他类型的 H，但仅当值为 Err 值时才有效。
			// map_err 不兼容于 Option 类型，因为使用 None 执行任何操作都是无意义的。
			// 应用场景应该是如果结果为 OK，则直接使用它的值，不做更多处理。而如果 err 了，不报错，而是返回另一个值时用到这个。
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			let dna = Self::random_value(&who);
			let kitty = Kitty(dna);

			// add220907，以下三个实际上都是storagemap或storagevalue。为什么中间必须加T暂时不清楚，但现在记住这些storagemap或storagevalue的固定用法就是了。
			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKittyId::<T>::set(kitty_id + 1);

			Self::deposit_event(Event::KittyCreated(who, kitty_id, kitty));
			// add220907，因为返回值DispatchResult是个result类型，所以这里返回一个ok。
			Ok(())
		}

		// aa13，1526，第二个方法是繁殖一个kt。从父母随机选择生产child数据。ensure!* 检查保证父母id不同。 let kitty1* let kitty2* 检查两个id确实已有对应数据在stroge里。
		// for* 这部分未听懂，之后再细看ddda，
		// 最后三排很多冒号的是更新链上数据。再之后是event抛出。

		#[pallet::weight(10_000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: KittyIndex,
			kitty_id_2: KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// add220907，Used as ensure!(expression_to_ensure, expression_to_return_on_false).
			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);

			let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;

			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			let selector = Self::random_value(&who);

			let mut data = [0u8; 16];

			// add220907,注意，这里用到了位运算，就是把10进制数换成2进制，对应每位做and或or运算得到新的二进制，再把结果转成10进制这样。
			// kitty_1是pub struct Kitty(pub [u8; 16]);是个元组结构体，且比较特殊，元组里就一个值[u8;16],所以kitty_1.0就是[u8;16]？？？？。
			for i in 0..kitty_1.0.len() {
				data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
			}
			let new_kitty = Kitty(data);

			<Kitties<T>>::insert(kitty_id, &new_kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKittyId::<T>::set(kitty_id + 1);

			Self::deposit_event(Event::KittyCreated(who, kitty_id, new_kitty));

			Ok(())
		}

		// aa14，1827，最后看transfer方法，有了kt后转给其它人。ensure!* 检查发送交易者和kt拥有者是否为同一人。
		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			kitty_id: u32,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;

			ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);

			<KittyOwner<T>>::insert(kitty_id, new_owner);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// aa9,0940,存储数据写完了，下面写具体方法。先定义几个辅助的函数，第一个为get ramdom值。其中extrinsic_index类似eth的nonce，用来和sender一起增加随机值的随机性。
		// 之后通过blake2_128哈希函数，产生随机[u8; 16]数组。
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}

		// aa10，1133，i2,get accountid和kt数据时，根据值不同映射到一个result里去，看看是会ok或error。
		// id之前提到过是u32，如果已经到了最大值，再去设置，就会返回error，没到max就返回ok。
		// add220907，l7，注意以下result定义
		// enum Result<T, E> {
		// 	Ok(T),
		// 	Err(E),
		// }
		// 这里只是ok，err调换了顺序，且这里val相当于就是
		// 注意，这里的val是通配模式，之前有误区，通配模式必须用_，但其实是任意变量名都行，用到_的时候是表示不需要使用它所对应的值,即右侧不会有_
		fn get_next_id() -> Result<KittyIndex, ()> {
			match Self::next_kitty_id() {
				KittyIndex::MAX => Err(()),
				val => Ok(val),
			}
		}

		// aa11，1232，接上个，对于取到kt包含的数据时，根据取到的值是some或none来做区分，如果是some就直接返回里面的存储值
		fn get_kitty(kitty_id: KittyIndex) -> Result<Kitty, ()> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty),
				None => Err(()),
			}
		}
	}
}
