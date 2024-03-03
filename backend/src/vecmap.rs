// This is from remuir https://github.com/ettolrach/remuir
// It's licensed under GPL, but I give myself the right to use it here :)

//! A map using a vec without hashing.
//! 
//! Performance should be fast for small lists where the hashing function of HashMap would unnecessarily slow down lookup.
//! 
//! # Examples
//! ```
//! use remuir::vecmap::VecMap;
//! let mut us_presidents: VecMap<u8, String> = VecMap::from_slice(&vec![
//!     (43, String::from("George W. Bush")),
//!     (44, String::from("Barack Obama")),
//!     (45, String::from("Donald Trump")),
//!     (46, String::from("Joe Biden")),
//! ]);
//! assert_eq!(None, us_presidents.get(&42));
//! us_presidents.update(42, String::from("Bill Clinton"));
//! assert_eq!("Bill Clinton", us_presidents.get(&42).unwrap());
//! ```
#[derive(Default, Debug, PartialEq)]
pub struct VecMap<K, V> {
    pub vec: Vec<(K, V)>
}
impl<K, V> VecMap<K, V> {
    pub fn new() -> VecMap<K, V>
    where
        K: PartialEq,
    {
        VecMap { vec: Vec::new() }
    }
    pub fn from_slice(tuples: &[(K, V)]) -> VecMap<K, V>
    where
        K: Clone,
        K: PartialEq,
        V: Clone,
    {
        VecMap { vec: Vec::from(tuples) }
    }
    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: PartialEq,
    {
        match self.position(key) {
            Some(i) => Some(&self.vec[i].1),
            None => None,
        }
    }
    fn position(&self, key: &K) -> Option<usize>
    where
        K: PartialEq,
    {
        for i in 0..(self.vec.len()) {
            if &self.vec[i].0 == key {
                return Some(i)
            }
        }
        None
    }
    pub fn update(&mut self, key: K, value: V)
    where
        K: PartialEq
    {
        match self.position(&key) {
            Some(i) => self.vec[i].1 = value,
            None => self.vec.push((key, value)),
        }
    }
    pub fn update_with_fn(&mut self, key: K, identity: V, func: impl FnOnce(&V) -> V)
    where
        K: PartialEq
    {
        match self.position(&key) {
            Some(i) => self.vec[i].1 = func(&self.vec[i].1),
            None => self.update(key, func(&identity)),
        }
    }
	pub fn remove(&mut self, key: &K) -> Option<V>
	where
		K: PartialEq,
		V: Clone
	{
		let position = self.position(key)?;
		Some(self.vec.remove(position).1)
	}
    pub fn keys(&self) -> Vec<&K> {
        self.vec.iter().map(|tuple| &tuple.0).collect()
    }
    pub fn values(&self) -> Vec<&V> {
        self.vec.iter().map(|tuple| &tuple.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::vecmap::VecMap;
    #[test]
    fn simple_update() {
        let mut us_presidents: VecMap<u8, String> = VecMap::from_slice(&vec![
            (43, String::from("George W. Bush")),
            (44, String::from("Barack Obama")),
            (45, String::from("Donald Trump")),
            (46, String::from("Joe Biden")),
        ]);
        assert_eq!(None, us_presidents.get(&42));
        us_presidents.update(42, String::from("Bill Clinton"));
        assert_eq!("Bill Clinton", us_presidents.get(&42).unwrap());
    }
}
