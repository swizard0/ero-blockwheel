use super::{
    block,
    context::Context,
};

#[derive(Debug)]
pub enum Request<C> where C: Context {
    Info(RequestInfo<C::Info>),
    Flush(RequestFlush<C::Flush>),
    LendBlock(RequestLendBlock<C::LendBlock>),
    RepayBlock(RequestRepayBlock),
    WriteBlock(RequestWriteBlock<C::WriteBlock>),
    ReadBlock(RequestReadBlock<C::ReadBlock>),
    DeleteBlock(RequestDeleteBlock<C::DeleteBlock>),
}

#[derive(Debug)]
pub struct RequestInfo<C> {
    pub context: C,
}

#[derive(Debug)]
pub struct RequestFlush<C> {
    pub context: C,
}

#[derive(Debug)]
pub struct RequestLendBlock<C> {
    pub context: C,
}

#[derive(Debug)]
pub struct RequestRepayBlock {
    pub block_bytes: block::Bytes,
}

#[derive(Debug)]
pub struct RequestWriteBlock<C> {
    pub block_bytes: block::Bytes,
    pub context: C,
}

#[derive(Debug)]
pub struct RequestReadBlock<C> {
    pub block_id: block::Id,
    pub context: C,
}

#[derive(Debug)]
pub struct RequestDeleteBlock<C> {
    pub block_id: block::Id,
    pub context: C,
}
