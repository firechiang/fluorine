/**
 * Interface
 */
pub trait BlockchainClient {
    type Error;
    fn get_otransaction_by_height(&self,height:u32) -> Result<Vec<OTransaction>,Self::Error>;
}

/**
 * Entity
 */
#[derive(Debug)]
#[allow(dead_code)]
pub struct OTransaction {
    pub block_hash:String,
    pub tx_hash:String,
    pub height:u32,
    pub contract:String,
    pub from:String,
    pub to:String,
    pub gas_fee:String,
    pub value:u64,
    pub token_id:u32,
    pub token_name:String,
    pub date_time:u32
}