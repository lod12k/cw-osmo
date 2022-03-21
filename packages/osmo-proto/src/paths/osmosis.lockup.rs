use crate::proto_ext::ProtoUrl;

impl ProtoUrl for MsgLockTokens {
    fn path(&self) -> &str {
        "/osmosis.lockup.MsgLockTokens"
    }
}

impl ProtoUrl for MsgBeginUnlocking {
    fn path(&self) -> &str {
        "/osmosis.lockup.MsgBeginUnlocking"
    }
}
