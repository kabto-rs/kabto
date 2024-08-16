pub(crate) struct IndexMap<K: Index, V> where [(); K::MAX as usize]: {
    indexes: [u8; K::MAX as usize],
    values:  Vec<(K, V)>,
}

pub(crate) trait Index: Copy {
    const MAX: u8;
    fn n(self) -> u8;
}

impl<K: Index, V> IndexMap<K, V> where [(); K::MAX as usize]: {
    const NULL: u8 = u8::MAX;

    pub(crate) fn with_capacity(cap: usize) -> Self {
        Self {
            indexes: [Self::NULL; K::MAX as usize],
            values:  Vec::with_capacity(cap)
        }
    }

    pub(crate) fn get(&self, key: K) -> Option<&V> {
        unsafe {match *self.indexes.get_unchecked(key.n() as usize) {
            Self::NULL => None,
            index      => Some(&self.values.get_unchecked(index as usize).1)
        }}
    }

    pub(crate) fn set(&mut self, key: K, value: V) {
        unsafe {match *self.indexes.get_unchecked(key.n() as usize) {
            Self::NULL => {
                let index = self.values.len();
                *self.indexes.get_unchecked_mut(key.n() as usize) = index as u8;
                self.values.push((key, value));
            },
            index => {
                *self.values.get_unchecked_mut(index as usize) = (key, value)
            }
        }}
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.values.iter()
            .filter(|(k, _)| Self::NULL != *unsafe {self.indexes.get_unchecked(k.n() as usize)})
    }
}
