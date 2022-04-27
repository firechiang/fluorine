pub mod types;
pub mod interface;
pub mod protos;
pub mod event;

#[cfg(test)]
mod test_types {
    use crate::types::ProtoType;

    #[test]
    fn test_proto_type() {
        let str = ProtoType::Str("test");
        println!("str={}",str);
    }
}

#[cfg(test)]
mod test_event {

    use crate::event::*;

    struct GossEvent;
    struct GossEventListener;

    impl EventObject for GossEvent {

        fn name(&self) -> &str {
            "GossEvent"
        }

    }

    impl EventListener for GossEventListener {

        fn on_event(&self, event: Box<dyn EventObject>) {
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




