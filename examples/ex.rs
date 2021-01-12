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
    // let v = vec![1,2,3];
    // access.outputs().iter().first
    let mut sender = access
        .outputs()
        .iter()
        .next()
        .map(|x| x.1.sender())
        .unwrap();
    let mut recv = access
        .inputs()
        .iter()
        .next()
        .map(|x| x.1.receiver())
        .unwrap();

    // sender.send()
    // for (k, v) in access.outputs().iter() {
    //     sender = Some(v.sender());
    //     break;
    // }

    // let a = access.inputs().iter().first();
    // let mut recv = None;
    // for (k, v) in access.inputs().iter() {
    //     recv = Some(v.receiver());
    //     break;
    // }
}
