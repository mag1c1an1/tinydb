mod kv {
    mod memory {
        use std::cmp::Ordering;
        use std::ops::{Deref, DerefMut};

        type Key = Vec<u8>;
        type Value = Vec<u8>;
        type KVPair = (Key, Value);
        type KVPairs = Vec<KVPair>;

        #[derive(Debug, PartialEq)]
        struct Values(KVPairs);

        impl Deref for Values {
            type Target = KVPairs;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for Values {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Values {
            fn new(order: usize) -> Self {
                Self(Vec::with_capacity(order))
            }
            fn delete(&mut self, key: &[u8]) {
                // not binary search
                for (i, (k, _)) in self.iter().enumerate() {
                    match (**k).cmp(key) {
                        Ordering::Greater => break,
                        Ordering::Equal => {
                            self.remove(i);
                            break;
                        }
                        Ordering::Less => {}
                    }
                }
            }
            fn get(&self, key: &[u8]) -> Option<Value> {
                self.iter()
                    .find_map(|(k, v)| match (**k).cmp(key) {
                        // earlty stop
                        Ordering::Greater => Some(None),
                        Ordering::Equal => Some(Some(v.to_vec())),
                        Ordering::Less => None,
                    })
                    .flatten()
            }
            fn get_first(&self) -> Option<KVPair> {
                self.0.first().cloned()
            }
            fn get_last(&self) -> Option<KVPair> {
                self.0.last().cloned()
            }
            fn get_next(&self, key: &[u8]) -> Option<KVPair> {
                // first greater
                self.iter().find_map(|(k, v)| match (**k).cmp(key) {
                    Ordering::Greater => Some((k.to_vec(), v.to_vec())),
                    _ => None,
                })
            }
            fn get_prev(&self, key: &[u8]) -> Option<KVPair> {
                self.iter().rev().find_map(|(k, v)| match (**k).cmp(key) {
                    Ordering::Less => Some((k.to_vec(), v.to_vec())),
                    _ => None,
                })
            }
            fn set(&mut self, key: &[u8], value: Vec<u8>) -> Option<(Key, Values)> {
                let mut insert_at = self.len();
                for (i, (k, v)) in self.iter_mut().enumerate() {
                    match (**k).cmp(key) {
                        Ordering::Greater => {
                            insert_at = i;
                            break;
                        }
                        Ordering::Equal => {
                            *v = value;
                            return None;
                        }
                        Ordering::Less => {}
                    }
                }
                if self.len() < self.capacity() {
                    self.insert(insert_at, (key.to_vec(), value));
                    return None;
                }
                let mut split_at = self.len() / 2;
                // ????
                if insert_at >= split_at {
                    split_at += 1;
                }
                let mut rvalues = Values::new(self.capacity());
                rvalues.extend(self.drain(split_at..));
                if insert_at >= self.len() {
                    rvalues.insert(insert_at - self.len(), (key.to_vec(), value));
                } else {
                    self.insert(insert_at, (key.to_vec(), value));
                }
                Some((rvalues[0].0.clone(), rvalues))
            }
        }
        mod tests {
            #[test]
            fn set_split() {
                todo!()
            }
        }
    }
}
