#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use pallet::*;
///
///			The module that hosts all the
///			[FRAME](https://docs.substrate.io/v3/runtime/frame)
///			types needed to add this pallet to a
///			runtime.
///
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::Randomness;
    use sp_io::hashing::blake2_128;
    type KittyIndex = u32;
    pub fn __type_value_for_get_default_value() -> KittyIndex {
        0_u32
    }
    pub struct Kitty(pub [u8; 16]);
    const _: () = {
        impl ::codec::Encode for Kitty {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::codec::Encode::encode_to(&&self.0, __codec_dest_edqy)
            }
            fn encode(&self) -> ::codec::alloc::vec::Vec<::core::primitive::u8> {
                ::codec::Encode::encode(&&self.0)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::codec::Encode::using_encoded(&&self.0, f)
            }
        }
        impl ::codec::EncodeLike for Kitty {}
    };
    const _: () = {
        impl ::codec::Decode for Kitty {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                ::core::result::Result::Ok(Kitty({
                    let __codec_res_edqy =
                        <[u8; 16] as ::codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        ::core::result::Result::Err(e) => {
                            return ::core::result::Result::Err(
                                e.chain("Could not decode `Kitty.0`"),
                            )
                        }
                        ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                }))
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Kitty {
        #[inline]
        fn clone(&self) -> Kitty {
            Kitty(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::core::marker::StructuralPartialEq for Kitty {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Kitty {
        #[inline]
        fn eq(&self, other: &Kitty) -> bool {
            self.0 == other.0
        }
        #[inline]
        fn ne(&self, other: &Kitty) -> bool {
            self.0 != other.0
        }
    }
    impl ::core::marker::StructuralEq for Kitty {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Kitty {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<[u8; 16]>;
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Kitty {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Kitty", &&self.0)
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Kitty {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Kitty", "pallet_kitties::pallet"))
                    .type_params(::alloc::vec::Vec::new())
                    .docs(&[])
                    .composite(
                        ::scale_info::build::Fields::unnamed()
                            .field(|f| f.ty::<[u8; 16]>().type_name("[u8; 16]").docs(&[])),
                    )
            }
        };
    };
    const _: () = {
        impl ::codec::MaxEncodedLen for Kitty {
            fn max_encoded_len() -> ::core::primitive::usize {
                0_usize.saturating_add(<[u8; 16]>::max_encoded_len())
            }
        }
    };
    ///
    ///			Configuration trait of this pallet.
    ///
    ///			Implement this type for a runtime in order to customize this pallet.
    ///
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
    }
    ///
    ///			The [pallet](https://docs.substrate.io/v3/runtime/frame#pallets) implementing
    ///			the on-chain logic.
    ///
    pub struct Pallet<T>(frame_support::sp_std::marker::PhantomData<(T)>);
    const _: () = {
        impl<T> core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        impl<T> core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        impl<T> core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    #[allow(type_alias_bounds)]
    pub type NextKittyId<T> = StorageValue<
        _GeneratedPrefixForStorageNextKittyId<T>,
        KittyIndex,
        ValueQuery,
        GetDefaultValue,
    >;
    #[allow(type_alias_bounds)]
    pub type Kitties<T> =
        StorageMap<_GeneratedPrefixForStorageKitties<T>, Blake2_128Concat, KittyIndex, Kitty>;
    #[allow(type_alias_bounds)]
    pub type KittyOwner<T: Config> = StorageMap<
        _GeneratedPrefixForStorageKittyOwner<T>,
        Blake2_128Concat,
        KittyIndex,
        T::AccountId,
    >;
    ///
    ///			The [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted
    ///			by this pallet.
    ///
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    pub enum Event<T: Config> {
        KittyCreated(T::AccountId, KittyIndex, Kitty),
        KittyBred(T::AccountId, KittyIndex, Kitty),
        KittyTransferred(T::AccountId, T::AccountId, KittyIndex),
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
    }
    const _: () = {
        impl<T: Config> core::clone::Clone for Event<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::KittyCreated(ref _0, ref _1, ref _2) => Self::KittyCreated(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                        core::clone::Clone::clone(_2),
                    ),
                    Self::KittyBred(ref _0, ref _1, ref _2) => Self::KittyBred(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                        core::clone::Clone::clone(_2),
                    ),
                    Self::KittyTransferred(ref _0, ref _1, ref _2) => Self::KittyTransferred(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                        core::clone::Clone::clone(_2),
                    ),
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Event<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Event<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (
                        Self::KittyCreated(_0, _1, _2),
                        Self::KittyCreated(_0_other, _1_other, _2_other),
                    ) => true && _0 == _0_other && _1 == _1_other && _2 == _2_other,
                    (
                        Self::KittyBred(_0, _1, _2),
                        Self::KittyBred(_0_other, _1_other, _2_other),
                    ) => true && _0 == _0_other && _1 == _1_other && _2 == _2_other,
                    (
                        Self::KittyTransferred(_0, _1, _2),
                        Self::KittyTransferred(_0_other, _1_other, _2_other),
                    ) => true && _0 == _0_other && _1 == _1_other && _2 == _2_other,
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::KittyCreated { .. }, Self::KittyBred { .. }) => false,
                    (Self::KittyCreated { .. }, Self::KittyTransferred { .. }) => false,
                    (Self::KittyCreated { .. }, Self::__Ignore { .. }) => false,
                    (Self::KittyBred { .. }, Self::KittyCreated { .. }) => false,
                    (Self::KittyBred { .. }, Self::KittyTransferred { .. }) => false,
                    (Self::KittyBred { .. }, Self::__Ignore { .. }) => false,
                    (Self::KittyTransferred { .. }, Self::KittyCreated { .. }) => false,
                    (Self::KittyTransferred { .. }, Self::KittyBred { .. }) => false,
                    (Self::KittyTransferred { .. }, Self::__Ignore { .. }) => false,
                    (Self::__Ignore { .. }, Self::KittyCreated { .. }) => false,
                    (Self::__Ignore { .. }, Self::KittyBred { .. }) => false,
                    (Self::__Ignore { .. }, Self::KittyTransferred { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::fmt::Debug for Event<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::KittyCreated(ref _0, ref _1, ref _2) => fmt
                        .debug_tuple("Event::KittyCreated")
                        .field(&_0)
                        .field(&_1)
                        .field(&_2)
                        .finish(),
                    Self::KittyBred(ref _0, ref _1, ref _2) => fmt
                        .debug_tuple("Event::KittyBred")
                        .field(&_0)
                        .field(&_1)
                        .field(&_2)
                        .finish(),
                    Self::KittyTransferred(ref _0, ref _1, ref _2) => fmt
                        .debug_tuple("Event::KittyTransferred")
                        .field(&_0)
                        .field(&_1)
                        .field(&_2)
                        .finish(),
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Event::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> ::codec::Encode for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::KittyCreated(ref aa, ref ba, ref ca) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ca, __codec_dest_edqy);
                    }
                    Event::KittyBred(ref aa, ref ba, ref ca) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ca, __codec_dest_edqy);
                    }
                    Event::KittyTransferred(ref aa, ref ba, ref ca) => {
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ca, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> ::codec::EncodeLike for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {
        }
    };
    const _: () = {
        impl<T: Config> ::codec::Decode for Event<T>
        where
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
        {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::KittyCreated(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyCreated.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <KittyIndex as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyCreated.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Kitty as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyCreated.2`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::KittyBred(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyBred.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <KittyIndex as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyBred.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Kitty as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyBred.2`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    __codec_x_edqy if __codec_x_edqy == 2usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::KittyTransferred(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyTransferred.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyTransferred.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <KittyIndex as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::KittyTransferred.2`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Event`, variant doesn't exist",
                    )),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Event<T>
        where
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            frame_support::sp_std::marker::PhantomData<(T)>: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new ("Event" , "pallet_kitties::pallet")) . type_params (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: scale_info :: TypeParameter :: new ("T" , :: core :: option :: Option :: None)]))) . docs_always (& ["\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]) . variant (:: scale_info :: build :: Variants :: new () . variant ("KittyCreated" , | v | v . index (0usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < T :: AccountId > () . type_name ("T::AccountId") . docs_always (& [])) . field (| f | f . ty :: < KittyIndex > () . type_name ("KittyIndex") . docs_always (& [])) . field (| f | f . ty :: < Kitty > () . type_name ("Kitty") . docs_always (& []))) . docs_always (& [])) . variant ("KittyBred" , | v | v . index (1usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < T :: AccountId > () . type_name ("T::AccountId") . docs_always (& [])) . field (| f | f . ty :: < KittyIndex > () . type_name ("KittyIndex") . docs_always (& [])) . field (| f | f . ty :: < Kitty > () . type_name ("Kitty") . docs_always (& []))) . docs_always (& [])) . variant ("KittyTransferred" , | v | v . index (2usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < T :: AccountId > () . type_name ("T::AccountId") . docs_always (& [])) . field (| f | f . ty :: < T :: AccountId > () . type_name ("T::AccountId") . docs_always (& [])) . field (| f | f . ty :: < KittyIndex > () . type_name ("KittyIndex") . docs_always (& []))) . docs_always (& [])))
            }
        };
    };
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    ///
    ///			Custom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)
    ///			of this pallet.
    ///
    pub enum Error<T> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
        InvalidKittyId,
        NotOwner,
        SameKittyId,
    }
    const _: () = {
        impl<T> ::codec::Encode for Error<T> {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Error::InvalidKittyId => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                    }
                    Error::NotOwner => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    }
                    Error::SameKittyId => {
                        __codec_dest_edqy.push_byte(2usize as ::core::primitive::u8);
                    }
                    _ => (),
                }
            }
        }
        impl<T> ::codec::EncodeLike for Error<T> {}
    };
    const _: () = {
        impl<T> ::codec::Decode for Error<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Error`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::InvalidKittyId)
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::NotOwner)
                    }
                    __codec_x_edqy if __codec_x_edqy == 2usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::SameKittyId)
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Error`, variant doesn't exist",
                    )),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T> ::scale_info::TypeInfo for Error<T>
        where
            frame_support::sp_std::marker::PhantomData<(T)>: ::scale_info::TypeInfo + 'static,
            T: 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new ("Error" , "pallet_kitties::pallet")) . type_params (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([:: scale_info :: TypeParameter :: new ("T" , :: core :: option :: Option :: None)]))) . docs_always (& ["\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]) . variant (:: scale_info :: build :: Variants :: new () . variant ("InvalidKittyId" , | v | v . index (0usize as :: core :: primitive :: u8) . docs_always (& [])) . variant ("NotOwner" , | v | v . index (1usize as :: core :: primitive :: u8) . docs_always (& [])) . variant ("SameKittyId" , | v | v . index (2usize as :: core :: primitive :: u8) . docs_always (& [])))
            }
        };
    };
    const _: () = {
        impl<T> frame_support::traits::PalletError for Error<T> {
            const MAX_ENCODED_SIZE: usize = 1;
        }
    };
    impl<T: Config> Pallet<T> {
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;
            let dna = Self::random_value(&who);
            let kitty = Kitty(dna);
            Kitties::<T>::insert(kitty_id, &kitty);
            KittyOwner::<T>::insert(kitty_id, &who);
            NextKittyId::<T>::set(kitty_id + 1);
            Self::deposit_event(Event::KittyCreated(who, kitty_id, kitty));
            Ok(())
        }
        pub fn breed(
            origin: OriginFor<T>,
            kitty_id_1: KittyIndex,
            kitty_id_2: KittyIndex,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            {
                if !(kitty_id_1 != kitty_id_2) {
                    {
                        return Err(Error::<T>::SameKittyId.into());
                    };
                }
            };
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
            NextKittyId::<T>::set(kitty_id + 1);
            Self::deposit_event(Event::KittyCreated(who, kitty_id, new_kitty));
            Ok(())
        }
        pub fn transfer(
            origin: OriginFor<T>,
            kitty_id: u32,
            new_owner: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;
            {
                if !(Self::kitty_owner(kitty_id) == Some(who.clone())) {
                    {
                        return Err(Error::<T>::NotOwner.into());
                    };
                }
            };
            <KittyOwner<T>>::insert(kitty_id, new_owner);
            Ok(())
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
        fn get_next_id() -> Result<KittyIndex, ()> {
            match Self::next_kitty_id() {
                KittyIndex::MAX => Err(()),
                val => Ok(val),
            }
        }
        fn get_kitty(kitty_id: KittyIndex) -> Result<Kitty, ()> {
            match Self::kitties(kitty_id) {
                Some(kitty) => Ok(kitty),
                None => Err(()),
            }
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata(
        ) -> frame_support::sp_std::vec::Vec<frame_support::metadata::PalletConstantMetadata>
        {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn error_metadata() -> Option<frame_support::metadata::PalletErrorMetadata> {
            Some(frame_support::metadata::PalletErrorMetadata {
                ty: frame_support::scale_info::meta_type::<Error<T>>(),
            })
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetStorageVersion for Pallet<T> {
        fn current_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::default()
        }
        fn on_chain_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version = frame_support::traits::StorageVersion::default();
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn module_name() -> &'static str {
            < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: module_name :: < Self > () . expect ("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn crate_version() -> frame_support::traits::CrateVersion {
            frame_support::traits::CrateVersion {
                major: 0u16,
                minor: 1u8,
                patch: 0u8,
            }
        }
    }
    impl<T: Config> frame_support::traits::PalletsInfoAccess for Pallet<T> {
        fn count() -> usize {
            1
        }
        fn accumulate(
            acc: &mut frame_support::sp_std::vec::Vec<frame_support::traits::PalletInfoData>,
        ) {
            use frame_support::traits::PalletInfoAccess;
            let item = frame_support::traits::PalletInfoData {
                index: Self::index(),
                name: Self::name(),
                module_name: Self::module_name(),
                crate_version: Self::crate_version(),
            };
            acc.push(item);
        }
    }
    impl<T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> frame_support::sp_std::vec::Vec<frame_support::traits::StorageInfo> {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            {
                let mut storage_info =
                    <NextKittyId<T> as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info =
                    <Kitties<T> as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info =
                    <KittyOwner<T> as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            res
        }
    }
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains one variant per dispatchable that can be called by an extrinsic.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T,)>,
            frame_support::Never,
        ),
        #[codec(index = 0u8)]
        create {},
        #[codec(index = 1u8)]
        breed {
            #[allow(missing_docs)]
            kitty_id_1: KittyIndex,
            #[allow(missing_docs)]
            kitty_id_2: KittyIndex,
        },
        #[codec(index = 2u8)]
        transfer {
            #[allow(missing_docs)]
            kitty_id: u32,
            #[allow(missing_docs)]
            new_owner: T::AccountId,
        },
    }
    const _: () = {
        impl<T: Config> core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Call::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::create {} => fmt.debug_struct("Call::create").finish(),
                    Self::breed {
                        ref kitty_id_1,
                        ref kitty_id_2,
                    } => fmt
                        .debug_struct("Call::breed")
                        .field("kitty_id_1", &kitty_id_1)
                        .field("kitty_id_2", &kitty_id_2)
                        .finish(),
                    Self::transfer {
                        ref kitty_id,
                        ref new_owner,
                    } => fmt
                        .debug_struct("Call::transfer")
                        .field("kitty_id", &kitty_id)
                        .field("new_owner", &new_owner)
                        .finish(),
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                    Self::create {} => Self::create {},
                    Self::breed {
                        ref kitty_id_1,
                        ref kitty_id_2,
                    } => Self::breed {
                        kitty_id_1: core::clone::Clone::clone(kitty_id_1),
                        kitty_id_2: core::clone::Clone::clone(kitty_id_2),
                    },
                    Self::transfer {
                        ref kitty_id,
                        ref new_owner,
                    } => Self::transfer {
                        kitty_id: core::clone::Clone::clone(kitty_id),
                        new_owner: core::clone::Clone::clone(new_owner),
                    },
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::create {}, Self::create {}) => true,
                    (
                        Self::breed {
                            kitty_id_1,
                            kitty_id_2,
                        },
                        Self::breed {
                            kitty_id_1: _0,
                            kitty_id_2: _1,
                        },
                    ) => true && kitty_id_1 == _0 && kitty_id_2 == _1,
                    (
                        Self::transfer {
                            kitty_id,
                            new_owner,
                        },
                        Self::transfer {
                            kitty_id: _0,
                            new_owner: _1,
                        },
                    ) => true && kitty_id == _0 && new_owner == _1,
                    (Self::__Ignore { .. }, Self::create { .. }) => false,
                    (Self::__Ignore { .. }, Self::breed { .. }) => false,
                    (Self::__Ignore { .. }, Self::transfer { .. }) => false,
                    (Self::create { .. }, Self::__Ignore { .. }) => false,
                    (Self::create { .. }, Self::breed { .. }) => false,
                    (Self::create { .. }, Self::transfer { .. }) => false,
                    (Self::breed { .. }, Self::__Ignore { .. }) => false,
                    (Self::breed { .. }, Self::create { .. }) => false,
                    (Self::breed { .. }, Self::transfer { .. }) => false,
                    (Self::transfer { .. }, Self::__Ignore { .. }) => false,
                    (Self::transfer { .. }, Self::create { .. }) => false,
                    (Self::transfer { .. }, Self::breed { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        impl<T: Config> ::codec::Encode for Call<T> {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::create {} => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                    }
                    Call::breed {
                        ref kitty_id_1,
                        ref kitty_id_2,
                    } => {
                        __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(kitty_id_1, __codec_dest_edqy);
                        ::codec::Encode::encode_to(kitty_id_2, __codec_dest_edqy);
                    }
                    Call::transfer {
                        ref kitty_id,
                        ref new_owner,
                    } => {
                        __codec_dest_edqy.push_byte(2u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(kitty_id, __codec_dest_edqy);
                        ::codec::Encode::encode_to(new_owner, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> ::codec::EncodeLike for Call<T> {}
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        impl<T: Config> ::codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::create {})
                    }
                    __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::breed {
                            kitty_id_1: {
                                let __codec_res_edqy =
                                    <KittyIndex as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::breed::kitty_id_1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            kitty_id_2: {
                                let __codec_res_edqy =
                                    <KittyIndex as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::breed::kitty_id_2`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 2u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::transfer {
                            kitty_id: {
                                let __codec_res_edqy =
                                    <u32 as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::transfer::kitty_id`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            new_owner: {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::transfer::new_owner`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Call`, variant doesn't exist",
                    )),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Call<T>
        where
            frame_support::sp_std::marker::PhantomData<(T,)>: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Call", "pallet_kitties::pallet"))
                    .type_params(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([::scale_info::TypeParameter::new(
                            "T",
                            ::core::option::Option::None,
                        )]),
                    ))
                    .docs_always(&[
                        "Contains one variant per dispatchable that can be called by an extrinsic.",
                    ])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant("create", |v| {
                                v.index(0u8 as ::core::primitive::u8)
                                    .fields(::scale_info::build::Fields::named())
                                    .docs_always(&[])
                            })
                            .variant("breed", |v| {
                                v.index(1u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::named()
                                            .field(|f| {
                                                f.ty::<KittyIndex>()
                                                    .name("kitty_id_1")
                                                    .type_name("KittyIndex")
                                                    .docs_always(&[])
                                            })
                                            .field(|f| {
                                                f.ty::<KittyIndex>()
                                                    .name("kitty_id_2")
                                                    .type_name("KittyIndex")
                                                    .docs_always(&[])
                                            }),
                                    )
                                    .docs_always(&[])
                            })
                            .variant("transfer", |v| {
                                v.index(2u8 as ::core::primitive::u8)
                                    .fields(
                                        ::scale_info::build::Fields::named()
                                            .field(|f| {
                                                f.ty::<u32>()
                                                    .name("kitty_id")
                                                    .type_name("u32")
                                                    .docs_always(&[])
                                            })
                                            .field(|f| {
                                                f.ty::<T::AccountId>()
                                                    .name("new_owner")
                                                    .type_name("T::AccountId")
                                                    .docs_always(&[])
                                            }),
                                    )
                                    .docs_always(&[])
                            }),
                    )
            }
        };
    };
    impl<T: Config> Call<T> {
        ///Create a call with the variant `create`.
        pub fn new_call_variant_create() -> Self {
            Self::create {}
        }
        ///Create a call with the variant `breed`.
        pub fn new_call_variant_breed(kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) -> Self {
            Self::breed {
                kitty_id_1,
                kitty_id_2,
            }
        }
        ///Create a call with the variant `transfer`.
        pub fn new_call_variant_transfer(kitty_id: u32, new_owner: T::AccountId) -> Self {
            Self::transfer {
                kitty_id,
                new_owner,
            }
        }
    }
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::create {} => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<()>>::weigh_data(
                        &__pallet_base_weight,
                        (),
                    );
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<()>>::classify_dispatch(
                            &__pallet_base_weight,
                            (),
                        );
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<()>>::pays_fee(
                        &__pallet_base_weight,
                        (),
                    );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::breed {
                    ref kitty_id_1,
                    ref kitty_id_2,
                } => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<(
                        &KittyIndex,
                        &KittyIndex,
                    )>>::weigh_data(
                        &__pallet_base_weight, (kitty_id_1, kitty_id_2)
                    );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &KittyIndex,
                        &KittyIndex,
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (kitty_id_1, kitty_id_2)
                    );
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<(
                        &KittyIndex,
                        &KittyIndex,
                    )>>::pays_fee(
                        &__pallet_base_weight, (kitty_id_1, kitty_id_2)
                    );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::transfer {
                    ref kitty_id,
                    ref new_owner,
                } => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<(
                        &u32,
                        &T::AccountId,
                    )>>::weigh_data(
                        &__pallet_base_weight, (kitty_id, new_owner)
                    );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &u32,
                        &T::AccountId,
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (kitty_id, new_owner)
                    );
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&u32, &T::AccountId)>>::pays_fee(
                            &__pallet_base_weight,
                            (kitty_id, new_owner),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &[::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(&["__Ignore cannot be used"], &[]),
                        )],
                    ))
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::create { .. } => "create",
                Self::breed { .. } => "breed",
                Self::transfer { .. } => "transfer",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &[::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(
                                &["__PhantomItem cannot be used."],
                                &[],
                            ),
                        )],
                    ))
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["create", "breed", "transfer"]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type Origin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::Origin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Self::create {} => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "create",
                                    "pallet_kitties::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/kitties/src/lib.rs"),
                                    Some(9u32),
                                    Some("pallet_kitties::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    frame_support::storage::in_storage_layer(|| {
                        <Pallet<T>>::create(origin)
                            .map(Into::into)
                            .map_err(Into::into)
                    })
                }
                Self::breed {
                    kitty_id_1,
                    kitty_id_2,
                } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "breed",
                                    "pallet_kitties::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/kitties/src/lib.rs"),
                                    Some(9u32),
                                    Some("pallet_kitties::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    frame_support::storage::in_storage_layer(|| {
                        <Pallet<T>>::breed(origin, kitty_id_1, kitty_id_2)
                            .map(Into::into)
                            .map_err(Into::into)
                    })
                }
                Self::transfer {
                    kitty_id,
                    new_owner,
                } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "transfer",
                                    "pallet_kitties::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/kitties/src/lib.rs"),
                                    Some(9u32),
                                    Some("pallet_kitties::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    frame_support::storage::in_storage_layer(|| {
                        <Pallet<T>>::transfer(origin, kitty_id, new_owner)
                            .map(Into::into)
                            .map_err(Into::into)
                    })
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &[::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(
                                &["__PhantomItem cannot be used."],
                                &[],
                            ),
                        )],
                    ));
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type Call = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn call_functions() -> frame_support::metadata::PalletCallMetadata {
            frame_support::scale_info::meta_type::<Call<T>>().into()
        }
    }
    impl<T: Config> frame_support::sp_std::fmt::Debug for Error<T> {
        fn fmt(
            &self,
            f: &mut frame_support::sp_std::fmt::Formatter<'_>,
        ) -> frame_support::sp_std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        #[doc(hidden)]
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &[::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(
                                &["`__Ignore` can never be constructed"],
                                &[],
                            ),
                        )],
                    ))
                }
                Self::InvalidKittyId => "InvalidKittyId",
                Self::NotOwner => "NotOwner",
                Self::SameKittyId => "SameKittyId",
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str {
            err.as_str()
        }
    }
    impl<T: Config> From<Error<T>> for frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            use frame_support::codec::Encode;
            let index = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: index :: < Pallet < T > > () . expect ("Every active module has an index in the runtime; qed") as u8 ;
            let mut encoded = err.encode();
            encoded.resize(frame_support::MAX_MODULE_ERROR_ENCODED_SIZE, 0);
            frame_support :: sp_runtime :: DispatchError :: Module (frame_support :: sp_runtime :: ModuleError { index , error : TryInto :: try_into (encoded) . expect ("encoded error is resized to be equal to the maximum encoded error size; qed") , message : Some (err . as_str ()) , })
        }
    }
    pub use __tt_error_token_1 as tt_error_token;
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_2 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::Event as From<Event<T>>>::from(event);
            let event =
                <<T as Config>::Event as Into<<T as frame_system::Config>::Event>>::into(event);
            <frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::metadata::PalletStorageMetadata {
            frame_support :: metadata :: PalletStorageMetadata { prefix : < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("Every active pallet has a name in the runtime; qed") , entries : { # [allow (unused_mut)] let mut entries = :: alloc :: vec :: Vec :: new () ; { < NextKittyId < T > as frame_support :: storage :: StorageEntryMetadataBuilder > :: build_metadata (:: alloc :: vec :: Vec :: new () , & mut entries) ; } { < Kitties < T > as frame_support :: storage :: StorageEntryMetadataBuilder > :: build_metadata (:: alloc :: vec :: Vec :: new () , & mut entries) ; } { < KittyOwner < T > as frame_support :: storage :: StorageEntryMetadataBuilder > :: build_metadata (:: alloc :: vec :: Vec :: new () , & mut entries) ; } entries } , }
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn next_kitty_id() -> KittyIndex {
            <NextKittyId<T> as frame_support::storage::StorageValue<KittyIndex>>::get()
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn kitties<KArg>(k: KArg) -> Option<Kitty>
        where
            KArg: frame_support::codec::EncodeLike<KittyIndex>,
        {
            <Kitties<T> as frame_support::storage::StorageMap<KittyIndex, Kitty>>::get(k)
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn kitty_owner<KArg>(k: KArg) -> Option<T::AccountId>
        where
            KArg: frame_support::codec::EncodeLike<KittyIndex>,
        {
            <KittyOwner<T> as frame_support::storage::StorageMap<KittyIndex, T::AccountId>>::get(k)
        }
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageNextKittyId<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
        for _GeneratedPrefixForStorageNextKittyId<T>
    {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "NextKittyId";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageKitties<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for _GeneratedPrefixForStorageKitties<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "Kitties";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageKittyOwner<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for _GeneratedPrefixForStorageKittyOwner<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "KittyOwner";
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    pub(super) trait Store {
        type NextKittyId;
        type Kitties;
        type KittyOwner;
    }
    impl<T: Config> Store for Pallet<T> {
        type NextKittyId = NextKittyId<T>;
        type Kitties = Kitties<T>;
        type KittyOwner = KittyOwner<T>;
    }
    impl<T: Config> frame_support::traits::Hooks<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
    }
    impl<T: Config> frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/kitties/src/lib.rs"),
                            Some(9u32),
                            Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_finalize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_idle(
            n: <T as frame_system::Config>::BlockNumber,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_idle (n , remaining_weight)
        }
    }
    impl<T: Config> frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_initialize(
            n: <T as frame_system::Config>::BlockNumber,
        ) -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/kitties/src/lib.rs"),
                            Some(9u32),
                            Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_initialize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "pallet_kitties::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/kitties/src/lib.rs"),
                            Some(9u32),
                            Some("pallet_kitties::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let pallet_name = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Self > () . unwrap_or ("<unknown pallet name>") ;
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["\u{2705} no migration for "],
                            &[::core::fmt::ArgumentV1::new_display(&pallet_name)],
                        ),
                        lvl,
                        &(
                            frame_support::LOG_TARGET,
                            "pallet_kitties::pallet",
                            "pallets/kitties/src/lib.rs",
                            9u32,
                        ),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_runtime_upgrade ()
        }
    }
    impl<T: Config> frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: offchain_worker (n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: integrity_test ()
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_4 as is_std_enabled_for_genesis;
    }
    pub struct GetDefaultValue(core::marker::PhantomData<((),)>);
    impl frame_support::traits::Get<KittyIndex> for GetDefaultValue {
        fn get() -> KittyIndex {
            __type_value_for_get_default_value()
        }
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_5 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
    }
    pub use __tt_default_parts_7 as tt_default_parts;
}
