use {
    super::define::*,
};

/**
 * Class
 */
#[derive(Debug)]
pub struct EthereumBlockchainClient {
    rpc_url: String,
}

/**
 * Method
 */
impl EthereumBlockchainClient {
    fn new(rpc_url:String) -> EthereumBlockchainClient {
        return EthereumBlockchainClient {
            rpc_url
        }
    }

    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }
}

/**
 * Implements interface
 */
impl BlockchainClient for EthereumBlockchainClient {

    fn get_otransaction_by_height(&self, height: u32) -> Result<Vec<OTransaction>, Box<dyn std::error::Error>> {
        let height_hex = std::format!("{:#0x}",height);
        todo!()
    }
}





