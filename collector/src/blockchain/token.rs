use lazy_static::lazy_static;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use super::define::Token;

lazy_static! {
    static ref CACHE_TOKEN: DashMap<&'static str,Token> = {
        let mut dash_map: DashMap<&'static str,Token> = DashMap::new();
        let tether = Token {
            contract: "0xdAC17F958D2ee523a2206206994597C13D831ec7",
            name: String::from("Tether USD"),
            symbol: String::from("USDT"),
            decimals: 6,
        };
        dash_map.insert("0xdAC17F958D2ee523a2206206994597C13D831ec7",tether);
        dash_map
    };
}

pub fn get_token(contract: &str) -> Option<Ref<&'static str,Token>> {
    CACHE_TOKEN.get(contract)
}

pub fn add_token(token: Token) {
    CACHE_TOKEN.insert(token.contract, token);
}

