use crate::{Class, ClassEventListener, parse::InputParser};

pub struct JsonParser<'a> {
	pub json: String,
	pub class_listeners: Vec<&'a mut dyn ClassEventListener>,
}

impl<'a> JsonParser<'a> {
	pub fn new() -> JsonParser<'a> {
		JsonParser {
			json: "".to_string(),
			class_listeners: Vec::new(),
		}
	}
}

impl<'a> InputParser<'a> for JsonParser<'a> {
	fn attach(&mut self, listener: &'a mut dyn ClassEventListener) {
		self.class_listeners.push(listener);
	}

	fn parse(&mut self, json: &'a str) {
		match serde_json::from_str::<Class>(json) {
			Ok(cls) => {
				for c in self.class_listeners.iter_mut() {
					c.handle_class(&cls);
				}
			},
			Err(e) => println!("Deserialization Error: {:?}", e),
		}
	}
}
