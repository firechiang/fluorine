/**
 * Interface
 */
pub trait BlockchainClient {
    fn get_otransaction_by_height(&self,height:u32) -> Result<Vec<OTransaction>, Box<dyn std::error::Error>> ;
}

/**
 * Entity
 */
#[derive(Debug)]
pub struct OTransaction {
    tx_hash:String,
    height:u32,
    contract:String,
    from:String,
    to:String,
    gas_fee:String,
    value:String,
    token_id:u32,
    token_name:String,
    time:u32
}

impl OTransaction {
    pub fn tx_hash(&self) -> &str {
        &self.tx_hash
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn contract(&self) -> &str {
        &self.contract
    }
    pub fn from(&self) -> &str {
        &self.from
    }
    pub fn to(&self) -> &str {
        &self.to
    }
    pub fn gas_fee(&self) -> &str {
        &self.gas_fee
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn token_id(&self) -> u32 {
        self.token_id
    }
    pub fn token_name(&self) -> &str {
        &self.token_name
    }
    pub fn time(&self) -> u32 {
        self.time
    }
}