use {
    reqwest::header::HeaderMap,
    std::time::Duration,
    commons::types::ProtoType,
    super::super::define::*,
    serde::de::DeserializeOwned,
    serde::{Deserialize, Serialize},
};

mod eth_method {
    pub const ETH_GET_BLOCK_BY_NUMBER:&str = "eth_getBlockByNumber";
}

#[derive(Serialize,Deserialize,Debug)]
struct Data<'a> {
    jsonrpc:String,
    id:u32,
    method:String,
    #[serde(borrow)]
    params:Vec<ProtoType<'a>>,
}

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
    pub fn new(rpc_url:&str) -> EthereumBlockchainClient {
        return EthereumBlockchainClient {
            rpc_url:String::from(rpc_url)
        }
    }
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    fn exec_request<T: DeserializeOwned>(&self,method: &str,params: Vec<ProtoType>) ->Result<RpcResponse<T>,reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        // header
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type","application/json".parse().unwrap());
        // params
        let req_data = Data {
            jsonrpc: String::from("2.0"),
            id: 1,
            method: String::from(method),
            params: params,
        };
        let response = client.post(self.rpc_url()).timeout(Duration::from_secs(5)).headers(headers).json(&req_data).send()?.json::<RpcResponse<T>>()?;
        Ok(response)
    }
}

/**
 * Implements interface
 */
impl BlockchainClient for EthereumBlockchainClient {

    type Error = reqwest::Error;

    fn get_otransaction_by_height(&self, height: u32) -> Result<Vec<OTransaction>,Self::Error> {
        let height_hex = std::format!("{:#0x}",height);
        let params = vec![ProtoType::String(height_hex),ProtoType::Bool(true)];
        let rpc_response = self.exec_request::<RpcResult<Transaction>>(eth_method::ETH_GET_BLOCK_BY_NUMBER,params)?;
        match rpc_response.result {
            Some(result) => {
                let timestamp: u32 = result.timestamp.self_to_number();
                let mut ts:Vec<OTransaction> = Vec::new();
                for t in result.transactions {
                    let ot = OTransaction {
                        block_hash: t.blockHash,
                        tx_hash: t.hash,
                        height: t.blockNumber.self_to_number(),
                        contract: String::from(""),
                        from: t.from,
                        to: match t.to {
                            Some(to) => to,
                            _=>String::from("")
                        },
                        gas_fee: String::from(""),
                        value: t.value.self_to_number(),
                        token_id: 0,
                        token_name: String::from(""),
                        date_time: timestamp
                    };
                    ts.push(ot);
                }
                Ok(ts)
            }
            _=> Ok(vec![])
        }
    }
}

#[derive(Deserialize,Debug)]
#[allow(dead_code)]
struct RpcResponse<T> {
    jsonrpc: String,
    id: u32,
    result: Option<T>
}

#[derive(Deserialize,Debug)]
#[allow(dead_code,non_snake_case)]
struct RpcResult<T> {
    number: String,
    timestamp: String,
    gasLimit: String,
    gasUsed: String,
    transactions: Vec<T>,
}

#[derive(Deserialize,Debug)]
#[allow(dead_code,non_snake_case)]
struct Transaction {
    hash: String,
    blockHash: String,
    blockNumber: String,
    chainId: Option<String>,
    from: String,
    to: Option<String>,
    gas: String,
    value: String,
    gasPrice: String,
    input: Option<String>,
}



trait FromToNumber<T> where T:{

    fn self_to_number(&self) -> T;
}

impl FromToNumber<u32> for String {
    fn self_to_number(&self) -> u32 {
        u32::from_str_radix(&self[2..],16).unwrap()
    }
}

impl FromToNumber<u64> for String {
    fn self_to_number(&self) -> u64 {
        u64::from_str_radix(&self[2..],16).unwrap()
    }
}







