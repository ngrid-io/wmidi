use wmidi::prelude::*;

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
