use serde::ser::{Serialize, Serializer, SerializeMap, SerializeSeq};
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess, SeqAccess};
use std::convert::TryFrom;
use std::fmt;

use super::*;
use std::collections::VecDeque;

#[cfg(test)]
mod tests;

struct NextNode {
	node: [Option<Box<Node>>; 26],
	pop: usize,
}

pub struct Node {
	freq: u32,
	next: NextNode,
}

pub struct LanguageModel {
	head: Node,
}

impl Node {
	pub fn new() -> Node {
		Node {
			freq: 0,
			next: NextNode::new(),
		}
	}
}

impl NextNode {
	pub fn new() -> NextNode {
		NextNode{
			node: Default::default(),
			pop: 0,
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
		S: Iterator<Item = Alph>,
	{
		let mut v: VecDeque<Alph> = VecDeque::with_capacity(depth);

		for c in s.take(v.capacity()).by_ref() {
			v.push_back(c);
		}
		self.insert_word(&mut v.iter().cloned());

		for c in s.by_ref() {
			v.pop_front();
			v.push_back(c);
			self.insert_word(&mut v.iter().cloned());
		}
		()
	}

	pub fn insert_word<S: Iterator>(&mut self, s: &mut S) where
		S: Iterator<Item = Alph>,
	{
		let mut cursor: &mut Node = &mut self.head;

		for c in s {
			let next: &mut Option<Box<Node>> = &mut cursor.next.node[c as usize];
			match *next {
				None => {
					*next = Some(Box::new(Node::new()));
					cursor.next.pop += 1;
				}
				_ => (),
			}
			cursor = next.as_mut().unwrap();
			cursor.freq += 1;
		}
		self.head.freq += 1;
	}
}

impl Serialize for LanguageModel {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
		S: Serializer,
	{
		self.head.serialize(serializer)
	}
}

impl Serialize for NextNode {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
		S: Serializer,
	{
		let mut m = serializer.serialize_map(Some(self.pop))?;
		for (i, n) in self.node.iter().enumerate() {
			match n {
				Some(x) => {
					m.serialize_entry(&Alph::try_from(i).unwrap(), x)?;
				}
				_ => (),
			}
		}
		m.end()
	}
}

impl Serialize for Node {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
		S: Serializer,
	{
		let mut m = serializer.serialize_seq(Some(2))?;
		m.serialize_element(&self.freq)?;
		m.serialize_element(&self.next)?;
		m.end()
	}
}

struct NodeVisitor;

impl<'de> Visitor<'de> for NodeVisitor {
	type Value = Node;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("node sequence")
	}

	fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error> where
		V: SeqAccess<'de>,
	{
		let mut node = Node::new();
		node.freq = seq.next_element()?
			.ok_or_else(|| de::Error::invalid_length(1, &self))?;
		node.next = seq.next_element()?
			.ok_or_else(|| de::Error::invalid_length(2, &self))?;
		Ok(node)
	}
}


struct NextNodeVisitor;

impl<'de> Visitor<'de> for NextNodeVisitor {
	type Value = NextNode;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("next node map")
	}

	fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error> where
		M: MapAccess<'de>,
	{
		let mut next = NextNode::new();
		while let Some((key, value)) = access.next_entry::<Alph, Node>()? {
			let i = usize::from(key);
			next.node[i] = Some(Box::new(value));
			next.pop += 1;
		}
		Ok(next)
	}
}

impl<'de> Deserialize<'de> for LanguageModel {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
		D: Deserializer<'de>,
	{
		let mut l = LanguageModel::new();
		l.head = deserializer.deserialize_seq(NodeVisitor)?;
		Ok(l)
	}
}

impl<'de> Deserialize<'de> for Node {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_seq(NodeVisitor)
	}
}

impl<'de> Deserialize<'de> for NextNode {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_map(NextNodeVisitor)
	}
}
