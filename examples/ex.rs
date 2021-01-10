fn main() {
    let i = wmidi_coremidi::MIDISourceIterator::new();

    for e in i {
        println!("endpoint {:?}", e.manufacturer());
    }

    // let a = "one";
    // let b = "two";

    // println!("{:?} {:?}", a, b);

}