pub mod types;
pub mod interface;
pub mod protos;
pub mod net;

#[cfg(test)]
mod test_types {
    use crate::types::ProtoType;

    #[test]
    fn test_proto_type() {
        let str = ProtoType::Str("test");
        println!("str={}",str);
    }
}






