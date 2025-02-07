use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::mem::swap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct SeqMap<K, V>(pub(crate) Vec<(K, V)>)
    where
        K: Ord;

impl<K, V> SeqMap<K, V>
    where
        K: Ord,
{
    pub fn new(mut inner: Vec<(K, V)>) -> Self {
        inner.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        inner.dedup_by(|(k1, _), (k2, _)| k1 == k2);
        Self(inner)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized,
    {
        let idx: usize = self
            .0
            .as_slice()
            .binary_search_by(|(k1, _)| k1.borrow().cmp(key))
            .ok()?;
        let item = self.0.as_slice().get(idx)?;
        Some(&item.1)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized,
    {
        let idx = (self)
            .0
            .as_slice()
            .binary_search_by(|(k1, _)| k1.borrow().cmp(key))
            .ok()?;
        let item = self.0.as_mut_slice().get_mut(idx)?;
        Some(&mut item.1)
    }

    pub fn search<Q>(&self, key: &Q) -> Result<usize, usize>
        where
            K: Borrow<Q>,
            Q: Ord + ?Sized,
    {
        self.0
            .as_slice()
            .binary_search_by(|(k1, _)| k1.borrow().cmp(key))
    }

    pub fn get_by_index(&self, idx: usize) -> Option<&V> {
        let item = self.0.as_slice().get(idx)?;
        Some(&item.1)
    }

    pub fn get_mut_by_index(&mut self, idx: usize) -> Option<&mut V> {
        let item = self.0.as_mut_slice().get_mut(idx)?;
        Some(&mut item.1)
    }

    pub fn keys(&self) -> impl Iterator<Item=&K> {
        self.0.iter().map(|item| &item.0)
    }
    pub fn inner(&self) -> &Vec<(K, V)> {
        &self.0
    }

    pub fn into_inner(self) -> Vec<(K, V)> {
        self.0
    }

    fn merge(&mut self, rhs: SeqMap<K, V>) {
        let mut inner: Vec<(K, V)> = Vec::with_capacity(self.0.len() + rhs.0.len());
        let mut old_inner = Vec::new();
        swap(&mut self.0, &mut old_inner);

        // rhs放前面保留后项V值
        rhs.0
            .into_iter()
            .chain(old_inner.into_iter())
            .for_each(|item| {
                inner.push(item);
            });
        inner.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        inner.dedup_by(|(k1, _), (k2, _)| k1 == k2);
        self.0 = inner;
    }

    fn merge_vec(&mut self, mut rhs: Vec<(K, V)>) {
        let mut inner: Vec<(K, V)> = Vec::with_capacity(self.0.len() + rhs.len());
        let mut old_inner = Vec::new();
        swap(&mut self.0, &mut old_inner);
        // 只保留后项V值
        while let Some(item) = rhs.pop() {
            inner.push(item)
        }
        inner.append(&mut self.0);
        inner.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        inner.dedup_by(|(k1, _), (k2, _)| k1 == k2);
        self.0 = inner;
    }

    fn merge_iter(&mut self, rhs: impl IntoIterator<Item=(K, V)>) {
        let rhs = rhs.into_iter().collect::<Vec<_>>();
        self.merge_vec(rhs);
    }
}

//impl From for Vec<(K, V) , HashMap<K, V>

impl<K, V> From<Vec<(K, V)>> for SeqMap<K, V>
    where
        K: Ord,
{
    fn from(vec: Vec<(K, V)>) -> Self {
        Self::new(vec)
    }
}

impl<K, V> From<HashMap<K, V>> for SeqMap<K, V>
    where
        K: Ord,
{
    fn from(map: HashMap<K, V>) -> Self {
        let mut inner = Vec::with_capacity(map.len());
        for item in map.into_iter() {
            inner.push(item);
        }
        inner.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));
        Self(inner)
    }
}

// impl clone serde*2 debug default

impl<K, V> Clone for SeqMap<K, V>
    where
        K: Ord + Clone,
        V: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<K, V> Serialize for SeqMap<K, V>
    where
        K: Ord + Serialize,
        V: Serialize,
{
    fn serialize<SER>(&self, serializer: SER) -> Result<SER::Ok, SER::Error>
        where
            SER: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, K, V> Deserialize<'de> for SeqMap<K, V>
    where
        K: Ord + Deserialize<'de>,
        V: Deserialize<'de>,
{
    fn deserialize<DES>(deserializer: DES) -> Result<Self, DES::Error>
        where
            DES: Deserializer<'de>,
    {
        let inner = Vec::deserialize(deserializer)?;
        Ok(Self::new(inner))
    }
}

//
impl<K, V> Debug for SeqMap<K, V>
    where
        K: Ord + Debug,
        V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<K, V> Default for SeqMap<K, V>
    where
        K: Ord,
{
    fn default() -> Self {
        Self(Vec::new())
    }
}
