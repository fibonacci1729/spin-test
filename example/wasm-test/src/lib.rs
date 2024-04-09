use bindings::{
    fermyon::spin_test::http_helper::{new_request, new_response},
    fermyon::{spin::key_value, spin_test_virt::key_value_calls},
    wasi::http::incoming_handler,
};

#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::Guest for Component {
    fn run() {
        // Configure the test
        let user = r#"{"id":123,"name":"Ryan"}"#;

        let key_value_config = key_value::Store::open("cache").unwrap();
        // Set state of the key-value store
        key_value_config.set("123", user.as_bytes()).unwrap();

        let request = new_request();
        let response_out = new_response();
        incoming_handler::handle(request, response_out);

        let calls = key_value_calls::get()
            .into_iter()
            .find_map(|(key, value)| (key == "cache").then_some(value))
            .unwrap()
            .into_iter()
            .map(|c| c.key)
            .collect::<Vec<_>>();
        assert_eq!(calls, vec!["123".to_owned()]);
    }
}

bindings::export!(Component with_types_in bindings);
