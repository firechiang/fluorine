pub mod protos;
pub mod utils;
pub mod event;

#[cfg(test)]
mod test_protos {
    use protobuf::Message;
    use crate::protos;

    #[test]
    fn test_convert() {
        let mut pub_node = protos::models::PubNode::new();
        pub_node.set_ip(String::from("127.0.0.1"));
        pub_node.set_port(8080);
        let bytes = pub_node.write_to_bytes();
        let red_node = protos::models::PubNode::parse_from_bytes(&bytes.unwrap()).unwrap();
        println!("{:#?}",red_node);
    }

}

#[cfg(test)]
mod test_utile {
    use crate::utils;

    #[test]
    fn test_can_connect() {
        let is = utils::net::can_connect("124.220.202.187",8080);
        println!("{:#?}",is);
    }

    #[test]
    fn test_get_pub_ip() {
        let res = utils::net::get_pub_ip();
        match res {
            Ok(r) => {
                println!("成功={:#?}",r);
            },
            Err(_e) => {}
        }
    }
}

#[cfg(test)]
mod test_event {

    use crate::event::*;

    struct GossEvent;
    struct GossEventListener;

    impl EventObject<EventUser> for GossEvent {
        fn name(&self) -> &str {
            "GossEvent"
        }

        fn body(&self) -> EventUser {
            EventUser{}
        }
    }

    impl EventListener for GossEventListener {
        fn on_event(&self, event: Box<dyn EventObject<EventUser>>) {
            let body = event.body();
            println!("触发{}事件",event.name());
        }
    }

    #[test]
    fn test_event() {
        let event = GossEvent{};
        let listener = GossEventListener{};
        let mut context = EventContext::new();
        context.add_listener("GossEvent",Box::new(listener));
        context.event(Box::new(event));
        println!("测试");
    }

}






