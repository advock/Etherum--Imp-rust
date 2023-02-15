#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "with-codec",
    derive(scale_codec::Encode, scale_codec::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExitReasons {
    Succeed(ExitSucceed),
    Error(ExitError),
    Revert(ExitRevert),
    Fatal(ExitFatal),
}
