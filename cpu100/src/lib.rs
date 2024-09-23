mod bindings;

use crate::bindings::exports::golem::component::api::*;

struct Component;

impl Guest for Component {
    fn start() -> String {
        "Hello from cpu100!".to_owned()
    }
}

bindings::export!(Component with_types_in bindings);
