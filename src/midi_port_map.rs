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
        // guard let type = first?.1.type else { print("No ports found"); return nil }
        // print("Select \(type) by typing the associated number")
        // let ports = map { $0.1 }

        // for (i, port) in ports.enumerated() {
        //     print("  #\(i) = \(port)")
        // }

        // print("Select: ", terminator: "")
        // guard let choice = (readLine().flatMap { Int($0) }) else { return nil }
        // return ports[safe: choice]
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
            },
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

impl<T: MIDIPort> std::ops::Index<&u32> for MIDIPortMap<T> {
    type Output = T;
    fn index(&self, index: &u32) -> &Self::Output {
        &self.inner[index]
    }
}
