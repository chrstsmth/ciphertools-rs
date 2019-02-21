use super::*;
use std::collections::VecDeque;

pub struct Node {
	next: [Option<Box<Node>>; 26],
	freq: u32,
}

pub struct LanguageModel {
	head: Node,
}

impl Node {
	pub fn new() -> Node {
		Node {
			next: Default::default(), //TODO would be nice if this was explicitly None
			freq: 0,
		}
	}
}

impl LanguageModel {
	pub fn new() -> LanguageModel {
		LanguageModel {
			head: Node::new(),
		}
	}

	pub fn insert_words<S>(&mut self, s: &mut S, depth: usize) where
		S: Iterator<Item = Alphabet>,
	{
		let mut v: VecDeque<Alphabet> = VecDeque::with_capacity(depth);

		for c in s.take(v.capacity()).by_ref() {
			v.push_back(c);
		}
		self.insert_word(&mut v.iter().cloned());

		for c in s.by_ref() {
			v.pop_front(); //TODO can't overwrite in one function call
			v.push_back(c);
			self.insert_word(&mut v.iter().cloned());
		}
		()
	}

	pub fn insert_word<S: Iterator>(&mut self, s: &mut S) where
		S: Iterator<Item = Alphabet>,
	{
		let mut cursor: &mut Node = &mut self.head;

		for c in s {
			let next: &mut Option<Box<Node>> = &mut cursor.next[c as usize];
			match *next {
				None => {
					*next = Some(Box::new(Node::new())); // TODO Errors on new?
				}
				_ => (),
			}
			cursor = next.as_mut().unwrap();
			cursor.freq += 1;
		}
	}
}

impl Serialize for LanguageModel {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let mut s = serializer.serialize_struct("LanguageModel:", 1)?;
			s.serialize_field("Node", &self.head)?;
			s.end()
		}

}
