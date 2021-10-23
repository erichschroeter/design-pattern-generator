use std::{collections::HashSet, io::Write};
use unindent::unindent;
use serde::{Deserialize, Serialize};

pub mod parse;

#[derive(Debug, Clone, Hash, PartialEq, Deserialize, Serialize)]
pub struct Variable {
	pub the_type: String,
	pub name: String,
}

impl Eq for Variable {}

#[derive(Debug, Clone, Hash, PartialEq, Deserialize, Serialize)]
pub struct Function {
	pub return_type: String,
	pub name: String,
	#[serde(default)]
	pub args: Vec<Variable>,
}

impl Eq for Function {}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Class {
	pub name: String,
	#[serde(default)]
	pub members: Vec<Variable>,
	#[serde(default)]
	pub functions: Vec<Function>,
}

impl PartialEq for Class {
	fn eq(&self, other: &Class) -> bool {
		let mut is_equal = true;
		if self.name == other.name {
			if self.members.len() == other.members.len() {
				let set1: HashSet<_> = self.members.iter().collect();
				let set2: HashSet<_> = other.members.iter().collect();
				let diff: Vec<_> = set1.difference(&set2).collect();
				if diff.len() > 0 {
					is_equal = false;
				}
				if self.functions.len() == other.functions.len() {
					let set1: HashSet<_> = self.functions.iter().collect();
					let set2: HashSet<_> = other.functions.iter().collect();
					let diff: Vec<_> = set1.difference(&set2).collect();
					if diff.len() > 0 {
						is_equal = false;
					}
				} else {
					is_equal = false;
				}
			} else {
				is_equal = false;
			}
		} else {
			is_equal = false;
		}
		is_equal
	}
}

pub trait ClassEventListener {
	fn handle_class(&mut self, cls: &Class);
}

pub trait CodeGenerator<'a, T: Write> {
	fn generate(&self, writer: &mut T);
}

#[derive(PartialEq)]
pub struct CppHeaderBuilderDesignPatternCodeGenerator {
	pub cpp: String,
}

impl CppHeaderBuilderDesignPatternCodeGenerator {
	pub fn new() -> CppHeaderBuilderDesignPatternCodeGenerator {
		CppHeaderBuilderDesignPatternCodeGenerator {
			cpp: "".to_string(),
		}
	}
}

impl<'a, T: Write> CodeGenerator<'a, T> for CppHeaderBuilderDesignPatternCodeGenerator {
	fn generate(&self, writer: &mut T) {
		write!(writer, "{}", self.cpp).unwrap();
	}
}

impl ClassEventListener for CppHeaderBuilderDesignPatternCodeGenerator {
	fn handle_class(&mut self, cls: &Class) {
		for member in &cls.members {
			self.cpp.push_str(&unindent(&format!("{} & with_{}({} val);", cls.name, member.name, member.the_type)))
		}
	}
}

#[derive(PartialEq)]
pub struct CppSourceBuilderDesignPatternCodeGenerator {
	pub cpp: String,
}

impl CppSourceBuilderDesignPatternCodeGenerator {
	pub fn new() -> CppSourceBuilderDesignPatternCodeGenerator {
		CppSourceBuilderDesignPatternCodeGenerator {
			cpp: "".to_string(),
		}
	}
}

impl<'a, T: Write> CodeGenerator<'a, T> for CppSourceBuilderDesignPatternCodeGenerator {
	fn generate(&self, writer: &mut T) {
		write!(writer, "{}", self.cpp).unwrap();
	}
}

impl ClassEventListener for CppSourceBuilderDesignPatternCodeGenerator {
	fn handle_class(&mut self, cls: &Class) {
		for member in &cls.members {
			self.cpp.push_str(&unindent(&format!(r#"{} & {}::with_{}({} val) {{
				m_{} = val;
				return *this;
			}}"#, cls.name, cls.name, member.name, member.the_type, member.name)))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	mod variable {
		use super::*;

		#[test]
		fn inequality_with_mismatching_name() {
			assert_ne!(
				Variable { the_type: "".to_string(), name: "x".to_string() },
				Variable { the_type: "".to_string(), name: "y".to_string() }
			);
		}

		#[test]
		fn inequality_with_mismatching_type() {
			assert_ne!(
				Variable { the_type: "x".to_string(), name: "".to_string() },
				Variable { the_type: "y".to_string(), name: "".to_string() }
			);
		}
	}

	mod function {
		use super::*;

		#[test]
		fn inequality_with_mismatching_name() {
			assert_ne!(
				Function { return_type: "".to_string(), name: "x".to_string(), args: vec![], },
				Function { return_type: "".to_string(), name: "y".to_string(), args: vec![] }
			);
		}

		#[test]
		fn inequality_with_mismatching_return_type() {
			assert_ne!(
				Function { return_type: "x".to_string(), name: "".to_string(), args: vec![], },
				Function { return_type: "y".to_string(), name: "".to_string(), args: vec![] }
			);
		}

		#[test]
		fn inequality_with_mismatching_arg_count() {
			assert_ne!(
				Function { return_type: "".to_string(), name: "".to_string(), args: vec![], },
				Function { return_type: "".to_string(), name: "".to_string(), args: vec![Variable{ name: "y".to_string(), the_type: "".to_string()}] }
			);
		}

		#[test]
		fn inequality_with_matching_arg_count() {
			assert_ne!(
				Function { return_type: "".to_string(), name: "".to_string(), args: vec![Variable{ name: "x".to_string(), the_type: "".to_string()}], },
				Function { return_type: "".to_string(), name: "".to_string(), args: vec![Variable{ name: "y".to_string(), the_type: "".to_string()}] }
			);
		}
	}

	mod class {
		use super::*;

		#[test]
		fn inequality_with_mismatching_name() {
			assert_ne!(
				Class { name: "x".to_string(), members: vec![], functions: vec![] },
				Class { name: "y".to_string(), members: vec![], functions: vec![] }
			);
		}

		#[test]
		fn inequality_with_mismatching_member_count() {
			assert_ne!(
				Class { name: "".to_string(), functions: vec![], members: vec![]},
				Class { name: "".to_string(), functions: vec![], members: vec![
					Variable{ name: "".to_string(), the_type: "".to_string() },
					Variable{ name: "".to_string(), the_type: "".to_string() },
				]}
			);
		}

		#[test]
		fn inequality_with_mismatching_function_count() {
			assert_ne!(
				Class { name: "".to_string(), members: vec![], functions: vec![]},
				Class { name: "".to_string(), members: vec![], functions: vec![
					Function{ name: "".to_string(), return_type: "".to_string(), args: vec![] },
					Function{ name: "".to_string(), return_type: "".to_string(), args: vec![] },
				]}
			);
		}
	}

	mod generators {
		use super::*;

		struct Setup {
			example_class: Class,
		}

		impl Setup {
			fn new() -> Self {
				Self {
					example_class: Class {
						name: "MyClass".to_string(),
						members: vec![
							Variable{
								name: "my_var".to_string(),
								the_type: "int".to_string(),
							}
						],
						functions: vec![
							Function{
								name: "my_func".to_string(),
								return_type: "void".to_string(),
								args: vec![Variable{
									name: "x".to_string(),
									the_type: "int *".to_string(),
								}]
							}
						]
					},
				}
			}
		}

		mod cpp_header {
			use super::*;

			#[test]
			fn builder_pattern() {
				let setup = Setup::new();
				let mut gen = CppHeaderBuilderDesignPatternCodeGenerator::new();
				gen.handle_class(&setup.example_class);
				let mut actual = Vec::new();
				gen.generate(&mut actual);
				assert_eq!(std::str::from_utf8(&actual).unwrap(), unindent(r#"MyClass & with_my_var(int val);"#));
			}
		}

		mod cpp_source {
			use super::*;

			#[test]
			fn builder_pattern() {
				let setup = Setup::new();
				let mut gen = CppSourceBuilderDesignPatternCodeGenerator::new();
				gen.handle_class(&setup.example_class);
				let mut actual = Vec::new();
				gen.generate(&mut actual);
				assert_eq!(std::str::from_utf8(&actual).unwrap(), unindent(r#"MyClass & MyClass::with_my_var(int val) {
					m_my_var = val;
					return *this;
				}"#));
			}
		}
	}
}
