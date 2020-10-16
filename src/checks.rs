pub(crate) mod private {
    /// Prevents internal traits from being implemented from dependencies.
    pub trait Sealed {}
}

/// Helper trait to check whether the size of bitfield structs
/// is a multiple of 8 to form complete bytes.
pub trait TotalSizeIsMultipleOfEightBits: private::Sealed {}

/// Helper trait to improve compile error messages.
pub trait RenameSizeType: private::Sealed {
    type CheckType;
}

/// Helper type to sum up bit size of a bitfield at compile time.
pub struct TotalSize<T>(::core::marker::PhantomData<T>);

macro_rules! impl_total_size_for {
    ( $(($n:expr, $name:ident)),* ) => {
        $(
            pub enum $name {}
            impl private::Sealed for TotalSize<[(); $n]> {}
            impl private::Sealed for $name {}
            impl RenameSizeType for TotalSize<[(); $n]> {
                type CheckType = $name;
            }
        )*
    }
}

impl_total_size_for!(
    (0, ZeroMod8),
    (1, OneMod8),
    (2, TwoMod8),
    (3, ThreeMod8),
    (4, FourMod8),
    (5, FiveMod8),
    (6, SixMod8),
    (7, SevenMod8)
);

impl TotalSizeIsMultipleOfEightBits for ZeroMod8 {}

/// Public facing trait implemented by bitfield structs in order to let the compiler
/// check if their sizes match a multiple of 8.
pub trait CheckTotalSizeMultipleOf8
where
    <Self::Size as RenameSizeType>::CheckType: TotalSizeIsMultipleOfEightBits,
{
    type Size: RenameSizeType;
}

/// Helper trait to check if an enum discriminant of a bitfield specifier
/// is within valid bounds.
pub trait DiscriminantInRange: private::Sealed {}

/// Helper type to state that something is `true`.
///
/// # Note
///
/// Used for some compile time evaluation contexts.
pub enum True {}

/// Helper type to state that something is `false`.
///
/// # Note
///
/// Used for some compile time evaluation contexts.
pub enum False {}

impl private::Sealed for True {}
impl DiscriminantInRange for True {}

/// Helper trait to improve compile time error messages.
pub trait DispatchTrueFalse: private::Sealed {
    type Out;
}

impl private::Sealed for [(); 0] {}
impl DispatchTrueFalse for [(); 0] {
    type Out = False;
}

// impl private::Sealed for [(); 1] {} // <-- Already implemented by `define_specifiers` macro!
impl DispatchTrueFalse for [(); 1] {
    type Out = True;
}

/// Public facing trait that is implemented by bitfield specifiers to
/// let the compiler check if all its variant discriminants are within
/// valid bounds.
pub trait CheckDiscriminantInRange<A>
where
    <Self::CheckType as DispatchTrueFalse>::Out: DiscriminantInRange,
{
    type CheckType: DispatchTrueFalse;
}

/// Helper type to check whether a bitfield member aligns to
/// the specified bits.
pub struct BitsCheck<A> {
    pub arr: A,
}
