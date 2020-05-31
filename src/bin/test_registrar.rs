use ui_system::state::state;
use ui_system::state::registrar::REGISTRAR;

fn main() {
    let mut s1 = state(0);
    let mut s2 = state("hi");
    let used = REGISTRAR.with(|reg|reg.used_states(||{
        s1.load();
        s2.load();
    }));

    println!("{:?}", used);
}