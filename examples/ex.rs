use wmidi::prelude::*;

struct MIDIReceiver {}

impl MIDIInputObserver for MIDIReceiver {
    fn receive(&mut self, packet_list: &MIDIPacketList) {
        todo!()
    }
}
fn main() {
    let access = MIDIAccess::new("wmidi");

    // for (_, p) in access.inputs().iter() {
    //     println!("input: {:?}", p);
    // }

    // for (_, p) in access.outputs().iter() {
    //     println!("outputs: {:?}", p);
    // }

    let z = access.inputs().prompt();
    println!("selected {:?}", z);
}
