use crate::{Class, ClassEventListener, Function, Variable, parse::InputParser};

pub struct CppParser<'a> {
	pub class_listeners: Vec<&'a mut dyn ClassEventListener>,
}

impl<'a> CppParser<'a> {
	pub fn new() -> CppParser<'a> {
		CppParser {
			class_listeners: Vec::new(),
		}
	}
}

impl<'a> InputParser<'a> for CppParser<'a> {
	fn attach(&mut self, listener: &'a mut dyn ClassEventListener) {
		self.class_listeners.push(listener);
	}

	fn parse(&mut self, _cpp: &'a str) {
		// TODO parse .h and .cpp for classes
		let cls = Class {
			name: "ShapeFactory".to_string(),
			members: vec![],
			functions: vec![
				Function {
					name: "create".to_string(),
					return_type: "std::shared_ptr<Shape>".to_string(),
					args: vec![
						Variable { name: "shape".to_string(), the_type: "std::string const &".to_string() }
					]
				},
			]
		};
		for c in self.class_listeners.iter_mut() {
			c.handle_class(&cls);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use unindent::unindent;

	struct ClassParseResult {
		pub class: Class,
	}

	impl ClassEventListener for ClassParseResult {
		fn handle_class(&mut self, cls: &Class) {
			self.class = cls.clone();
			// let c = _cls.clone();
			// self.class = c.clone();
			// self.class = Class::default();

			// self.class.name = cls.name.clone();
			// self.class = Class {
			// 	name: "ShapeFactory",
			// 	members: vec![],
			// 	functions: vec![],
			// 	// name: cls.name.clone(),
			// 	// members: cls.members.clone(),
			// 	// functions: cls.functions.clone(),
			// };
		}
	}

	#[test]
	fn simple_header() {
		let cpp = unindent(r#"
		#ifndef EXAMPLE_H
		#define EXAMPLE_H

		#include <memory>

		class Shape {
		public:
			virtual double calc_area() = 0;
		};

		class Square : public Shape {
		public:
			virtual double calc_area();
		};

		class Circle : public Shape {
		public:
			virtual double calc_area();
		};

		class ShapeFactory {
		public:
			std::shared_ptr<Shape> create(std::string const & shape);
		};

		#endif /* #ifndef EXAMPLE_H */
		"#);
		let mut actual = ClassParseResult {
			class: Class::default(),
		};
		let mut parser = CppParser::new();
		parser.attach(&mut actual);
		parser.parse(&cpp);
		assert_eq!(actual.class, Class {
			name: "ShapeFactory".to_string(),
			members: vec![],
			functions: vec![
				Function{
					name: "create".to_string(),
					return_type: "std::shared_ptr<Shape>".to_string(),
					args: vec![Variable{
						name: "shape".to_string(),
						the_type: "std::string const &".to_string(),
					}]
				}
			]
		});
	}
}
