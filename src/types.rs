
#[derive(Debug)]
pub struct FrameworkOptions {
  pub subscribe_pending_txs: bool,
  pub subscribe_blocks: bool,
  pub log_filters: Vec<ethers::types::Filter>
}

impl FrameworkOptions {

  pub fn new(
    subscribe_pending_txs: bool,
    subscribe_blocks: bool,
    log_filters: Vec<ethers::types::Filter>
  ) -> Self {
    FrameworkOptions {
      subscribe_pending_txs,
      subscribe_blocks,
      log_filters
    }
  }
  
}

#[derive(Debug)]
pub enum EventType {
  PendingTx(ethers::types::Transaction),
  Block(ethers::types::Block<ethers::types::Transaction>),
  Log(ethers::types::Log)
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum LogType {
  H160,
  H256,
  U256
}
