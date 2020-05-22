use serde::ser::{Serialize, Serializer, SerializeMap, SerializeSeq};
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess, SeqAccess};
use std::borrow::Borrow;
use std::fmt;
use crate::alphabet::latin::*;
use enum_map::*;
use std::collections::HashMap;
use std::iter::FromIterator;
use crate::analysis::ngram_frequencies::*;

use std::collections::VecDeque;

#[cfg(test)]
mod tests;

struct NextNode {
	node: EnumMap<Latin, Option<Box<Node>>>,
	pop: usize,
}

pub struct Node {
	freq: u32,
	prob: f64,
	is_word: bool,
	next: NextNode,
}

pub struct LanguageModel {
	head: Node,
}

#[derive(Clone)]
pub struct LanguageModelTraverser<'a> {
	cursor: &'a Node,
}

impl Node {
	pub fn new() -> Node {
		Node {
			freq: 0,
			prob: 0.0,
			is_word: false,
			next: NextNode::new(),
		}
	}

	pub fn is_word(&self) -> bool {
		self.is_word
	}

	pub fn freq(&self) -> u32 {
		self.freq
	}

	pub fn prob(&self) -> f64 {
		self.prob
	}

	pub fn ngram_frequencies(&self, n: u32) -> NgramFrequency {
		NgramFrequency::from(HashMap::from_iter(self.ngrams(n)))
	}

	fn ngrams(&self, n: u32) -> Vec<(Vec<Latin>, u32)> {
		let mut ngrams = self.ngrams_in_reverse(n);
		for (ngram, _) in &mut ngrams {
			ngram.reverse();
		}
		ngrams
	}

	// This could be improved so that the ngrams aren't in reverse.
	fn ngrams_in_reverse(&self, n: u32) -> Vec<(Vec<Latin>, u32)> {
		let mut ngrams = Vec::new();

		if n < 1 {
			return ngrams;
		}

		for (c, next) in &self.next.node {
			if let Some(node) = next {
				if n == 1 {
					ngrams.push((vec![c], node.freq));
				} else {
					let mut ngrams_from_node = node.ngrams_in_reverse(n - 1);
					for (ngram, _) in &mut ngrams_from_node {
						ngram.push(c);
					}
					ngrams.append(&mut ngrams_from_node);
				}
			}
		}
		ngrams
	}
}

impl NextNode {
	pub fn new() -> NextNode {
		NextNode {
			node: enum_map!{ _ => None},
			pop: 0,
		}
	}
}

impl<'a> LanguageModelTraverser<'a> {
	pub fn next(&mut self, c: Latin) -> Option<&'a Node> {
		let next: &Option<Box<Node>> = &self.cursor.next.node[c];
		match next {
			None => return None,
			Some(boxed_node) => {
				self.cursor = boxed_node.borrow();
				Some(boxed_node.borrow())
			}
		}
	}
}

impl LanguageModel {
	pub fn new() -> LanguageModel {
		LanguageModel {
			head: Node::new(),
		}
	}

	pub fn insert_word<S>(&mut self, s: S) where
		S: Iterator<Item = Latin>,
	{
		self.insert_word_n_times(s, 1);
	}

	pub fn insert_word_n_times<S>(&mut self, s: S, n: u32) where
		S: Iterator<Item = Latin>,
	{
		self.insert(s, true, n);
	}

	pub fn windowed_insert_word<S>(&mut self, s: S) where
		S: Iterator<Item = Latin> + Clone,
	{
		self.windowed_insert_word_n_times(s, 1);
	}

	pub fn windowed_insert_word_n_times<S>(&mut self, mut s: S, n: u32) where
		S: Iterator<Item = Latin> + Clone,
	{
		self.insert(&mut s.clone(), true, n);
		while let Some(_) = s.next() {
			self.insert(&mut s.clone(), false, n);
		}
	}

	fn insert<S>(&mut self, s: S, is_word: bool, n: u32) where
		S: Iterator<Item = Latin>,
	{
		let mut cursor: &mut Node = &mut self.head;

		for c in s {
			let next: &mut Option<Box<Node>> = &mut cursor.next.node[c];
			match *next {
				None => {
					*next = Some(Box::new(Node::new()));
					cursor.next.pop += 1;
				}
				_ => (),
			}
			cursor = next.as_mut().unwrap();
			cursor.freq += n;
		}
		cursor.is_word = is_word;
		self.head.freq += n;
	}

	pub fn generate_probabilities(&mut self) {
		LanguageModel::generate_probabilities_for_node(&mut self.head, 1.0);
	}

	fn generate_probabilities_for_node(parent: &mut Node, prob_of_reaching_parent: f64) {
		parent.prob = prob_of_reaching_parent;

		for (_, next) in &mut parent.next.node {
			if let Some(node) = next {
				let p = prob_of_reaching_parent * f64::from(node.freq) / f64::from(parent.freq);
				LanguageModel::generate_probabilities_for_node(node, p);
			}
		}
	}

	pub fn ngram_frequencies(&self, n: u32) -> NgramFrequency {
		self.ngram_frequencies_recurse(n, &self.head)
	}

	fn ngram_frequencies_recurse(&self, n: u32, node: &Node) -> NgramFrequency {
		let mut ngrams = HashMap::new();

		for (_, next) in &node.next.node {
			if let Some(node) = next {
				for (ngram, freq) in self.ngram_frequencies_recurse(n, node) {
					*ngrams.entry(ngram).or_insert(0) += freq;
				}
			}
		}

		for (ngram, freq) in node.ngrams(n) {
			*ngrams.entry(ngram).or_insert(0) += freq;
		}

		NgramFrequency::from(ngrams)
	}

	pub fn traverse(&self) -> LanguageModelTraverser {
		LanguageModelTraverser {
			cursor: &self.head,
		}
	}

	pub fn head(&self) -> &Node {
		&self.head
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
		for (lang, next) in self.node.iter() {
			match next {
				Some(x) => {
					m.serialize_entry(&lang, x)?;
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
		m.serialize_element(&self.is_word)?;
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
		node.is_word = seq.next_element()?
			.ok_or_else(|| de::Error::invalid_length(3, &self))?;
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
		while let Some((lang, node)) = access.next_entry::<Latin, Node>()? {
			next.node[lang] = Some(Box::new(node));
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
