use pdg::PDG;

mod pdg;
mod path;
mod cspdg;

fn main() {
    let mut pdg = PDG::test_data();
    pdg.print();
    let paths = pdg.get_paths();
    for p in paths{
        pdg.print_path(&p);
    }

    let cspdg = pdg.cspdg();
    cspdg.print();
}
