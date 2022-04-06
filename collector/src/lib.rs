pub mod blockchain;

#[cfg(test)]
mod test {

    use crate::blockchain::define::BlockchainClient;
    use crate::blockchain::ethereum::client::EthereumBlockchainClient;
    use crate::blockchain::define::Token;
    use crate::blockchain::token;

    #[test]
    fn test_ethereum_get_otransaction_by_height() {
        let client = EthereumBlockchainClient::new("https://rinkeby.infura.io/v3/9f7af0c27f6042ceb778598253ec9679");
        let res = BlockchainClient::get_otransaction_by_height(&client,10431858);
        println!("res={:#?}",res);
    }

    #[test]
    fn test_token() {
        match token::get_token("0xdAC17F958D2ee523a2206206994597C13D831ec7") {
            Some(refe) => {
                let token = refe.value();
                println!("token1={:#?}",token);
            }
            _=> ()
        }

        let tether = Token {
            contract: "0",
            name: String::from("0"),
            symbol: String::from("0"),
            decimals: 6,
        };
        token::add_token(tether);

        match token::get_token("0") {
            Some(refe) => {
                let token = refe.value();
                println!("token3={:#?}",token);
            }
            _=> ()
        }

        match token::get_token("0") {
            Some(refe) => {
                let token = refe.value();
                println!("token4={:#?}",token);
            }
            _=> ()
        }
    }
}