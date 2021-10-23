use crate::ClassEventListener;

pub mod json;
pub mod cpp;

pub trait InputParser<'a> {
	fn attach(&mut self, listener: &'a mut dyn ClassEventListener);
	fn parse(&mut self, str: &'a str);
}
