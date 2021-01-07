use crate::{
    prelude::*,
    MIDIEndpoint,
};

pub struct MIDIPortMapIterator<'a, T: MIDIPort> {
    inner: std::collections::hash_map::Iter<'a, u32, T>,
}

impl<'a, T: MIDIPort> Iterator for MIDIPortMapIterator<'a, T> {
    type Item = (&'a u32, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct MIDIPortMap<T: MIDIPort> {
    inner: std::collections::HashMap<u32, T>,
}

impl<T: MIDIPort> MIDIPortMap<T> {
    pub fn iter(&self) -> MIDIPortMapIterator<T> {
        MIDIPortMapIterator {
            inner: self.inner.iter(),
        }
    }

    fn port_for(&self, endpoint: &MIDIEndpoint) -> T {
        todo!()
    }
}

impl MIDIPortMap<MIDIInput> {
    pub fn new() -> Self {
        // use std::collections::hash_map::Iter;
        todo!()
    }
}

impl MIDIPortMap<MIDIOutput> {
    pub fn new() -> Self {
        todo!()
    }
}

impl<T: MIDIPort> std::ops::Index<&u32> for MIDIPortMap<T> {
    type Output = T;
    fn index(&self, index: &u32) -> &Self::Output {
        &self.inner[index]
    }
}
