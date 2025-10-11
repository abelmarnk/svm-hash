use std::{
    hash::{
        Hash
    }, 
    ops::{
        Deref, 
        DerefMut
    }
};

use hasher::{
    SvmBuildHasher
};

type SvmHashMap<K,V> = std::collections::HashMap<K,V,SvmBuildHasher>;

pub struct HashMap<K,V>(
    SvmHashMap<K,V>
);

impl<K,V> Deref for HashMap<K,V>{
    type Target = SvmHashMap<K,V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K,V> DerefMut for HashMap<K,V>{

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K,V> HashMap<K,V>{
    pub fn new() -> HashMap<K, V> {
        Self(SvmHashMap::default())
    }

    pub fn with_capacity(capacity: usize) -> HashMap<K, V> {
        Self(std::collections::HashMap::
            with_capacity_and_hasher(
                capacity, 
                SvmBuildHasher::default()
        ))
    }
}

impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Eq + Hash,
{

    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> HashMap<K, V> {
        let mut map = HashMap::new();
        map.extend(iter);
        map
    }
}

impl<K, V, const N:usize> From<[(K, V);N]> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn from(value: [(K, V);N]) -> Self {
        Self(
            SvmHashMap::from_iter(
                value
            )
        )
    }
}