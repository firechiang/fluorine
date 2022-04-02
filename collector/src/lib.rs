pub mod blockchain;

#[cfg(test)]
mod test {

    use crate::blockchain::define::BlockchainClient;
    use crate::blockchain::ethereum::client::EthereumBlockchainClient;

    #[test]
    fn test_ethereum_get_otransaction_by_height() {
        let client = EthereumBlockchainClient::new("https://rinkeby.infura.io/v3/9f7af0c27f6042ceb778598253ec9679");
        let res = BlockchainClient::get_otransaction_by_height(&client,10431858);
        println!("res={:#?}",res);
    }
}