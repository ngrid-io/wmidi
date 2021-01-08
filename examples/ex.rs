// based on webmidikit demo
use wmidi::prelude::*;

struct MIDIReceiver {}

impl MIDIInputObserver for MIDIReceiver {
    fn receive(&mut self, packet_list: &coremidi::PacketList) {
        println!("received packet");
        // todo!()
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

    // let noteOn: [UInt8] = [0x90, 0x40, 0x7f]
    // let noteOff: [UInt8] = [0x80, 0x40, 0]
    let input = access.inputs().prompt().unwrap();
    // println!("selected {:?}", z);
    let output = access.output_for(input);

    // input.set_input_observer(Box::new(MIDIReceiver {}));
}
