use std::{
    hash::Hash, ops::{
        Deref,
        DerefMut
    }
};

use hasher::SvmBuildHasher;

type SvmHashSet<K> = std::collections::HashSet<K, SvmBuildHasher>;

pub struct HashSet<K>(
    SvmHashSet<K>
);

impl<K> Deref for HashSet<K>{
    type Target = SvmHashSet<K>;

    fn deref(&self) -> &Self::Target {
        &self.0    
    }
}

impl<K> DerefMut for HashSet<K>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K> HashSet<K>{
    pub fn new() -> Self{
        Self(
            SvmHashSet::default()
        )
    }

    pub fn with_capacity(capacity:usize) -> Self{
        Self(
            SvmHashSet::with_capacity_and_hasher(
                capacity, 
                SvmBuildHasher::default()
            )
        )
    }
}

impl<K> FromIterator<K> for HashSet<K>
where K: Eq + Hash
{

    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        Self(
            SvmHashSet::from_iter(
                iter
            )
        )
    }
}

impl<K, const N:usize> From<[K;N]> for HashSet<K>
where K: Eq + Hash
{
    fn from(value: [K;N]) -> Self {
        Self(
            SvmHashSet::from_iter(
                value
            )
        )
    }
} 

