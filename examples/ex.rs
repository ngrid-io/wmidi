use wmidi::prelude::*;

fn main() {
    let access = MIDIAccess::new("wmidi");

    for (_, p) in access.inputs().iter() {
        println!("input: {:?}", p);
    }
}
