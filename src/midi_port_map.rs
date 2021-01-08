use crate::{
    prelude::*,
    MIDIClient,
    MIDIEndpoint,
};

fn get_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MIDIPortID {
    inner: u32,
}

impl MIDIPortID {
    pub fn new(inner: u32) -> Self {
        Self { inner }
    }
}

pub struct MIDIPortMapIterator<'a, T: MIDIPort> {
    inner: std::collections::hash_map::Iter<'a, MIDIPortID, T>,
}

impl<'a, T: MIDIPort> Iterator for MIDIPortMapIterator<'a, T> {
    type Item = (&'a MIDIPortID, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct MIDIPortMap<T: MIDIPort> {
    inner: std::collections::HashMap<MIDIPortID, T>,
}

impl<T: MIDIPort> MIDIPortMap<T> {
    pub fn iter(&self) -> MIDIPortMapIterator<T> {
        MIDIPortMapIterator {
            inner: self.inner.iter(),
        }
    }

    fn port_for_endpoint(&self, endpoint: &MIDIEndpoint) -> Option<&T> {
        let id = endpoint.id();
        self.iter().find(|x| x.1.id() == id).map(|x| x.1)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn prompt(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        println!("Select {:?} ", std::any::type_name::<T>());

        let ports: Vec<_> = self.iter().collect();
        for (i, (k, v)) in ports.iter().enumerate() {
            println!("#{:?} {:?}", i, v);
        }

        let input = get_line();
        let index = input.parse::<usize>();
        match index {
            Ok(index) => {
                if (0..ports.len()).contains(&index) {
                    Some(ports[index].1)
                } else {
                    println!("invalid index {:?}, max value {:?}", index, ports.len());
                    None
                }
            }
            _ => {
                println!("failed to parse input {:?}", input);
                None
            }
        }
    }
}

impl MIDIPortMap<MIDIInput> {
    pub(crate) fn new(client: &MIDIClient) -> Self {
        let mut inner = std::collections::HashMap::new();
        for e in coremidi::Sources {
            let input = MIDIInput::new(e, client.clone());
            inner.insert(input.id(), input);
        }
        Self { inner }
    }
}

impl MIDIPortMap<MIDIOutput> {
    pub(crate) fn new(client: &MIDIClient) -> Self {
        let mut inner = std::collections::HashMap::new();
        for e in coremidi::Destinations {
            let output = MIDIOutput::new(e, client.clone());
            inner.insert(output.id(), output);
        }
        Self { inner }
    }
}

impl<T: MIDIPort> std::ops::Index<&MIDIPortID> for MIDIPortMap<T> {
    type Output = T;
    fn index(&self, index: &MIDIPortID) -> &Self::Output {
        &self.inner[index]
    }
}
