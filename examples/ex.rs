use wmidi_coremidi::prelude::*;

fn main() {
    // let i = wmidi_coremidi::MIDISourceIterator::new();

    // for e in i {
    //     println!("endpoint {:?}", e.manufacturer());
    // }

    // let a = "one";
    // let b = "two";

    // println!("{:?} {:?}", a, b);

    let access = MIDIAccess::new("example");

    for (k, v) in access.outputs().iter() {

        // let sender = v.sender();
        // println!("")
        // break;
        // println!("{} {}", k, v);
    }
}
