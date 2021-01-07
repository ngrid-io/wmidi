use crate::{
    prelude::*,
    MIDIClient,
};


pub struct MIDIAccess {
    client: MIDIClient,
    inputs: MIDIPortMap<MIDIInput>,
    outputs: MIDIPortMap<MIDIOutput>,
}

impl MIDIAccess {
    pub fn inputs(&self) -> &MIDIPortMap<MIDIInput> {
        &self.inputs
    }

    pub fn outputs(&self) -> &MIDIPortMap<MIDIOutput> {
        &self.outputs
    }
}