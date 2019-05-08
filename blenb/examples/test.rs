use blenb::Blend;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::{env, path};

pub fn main() -> Result<(), io::Error> {
    let base_path = path::PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("could not find cargo manifest dir"),
    );

    let blend_path = base_path.join("../assets/characters/male/source/male2.blend"); //examples/print_blend/simple.blend
    let output_path = base_path.join("examples/print_blend/output.txt");

    println!("{:?}", blend_path);

    let mut file = File::open(blend_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let blend = Blend::new(&data[..]);

    let inst = blend
        .get_by_code([b'M', b'A'])
        .filter_map(|inst| {
            if inst.is_valid("nodetree") {
                Some(inst)
            } else {
                None
            }
        })
        .next()
        .unwrap()
        .get_vec("nodetree");


    for inst in inst {
        let nodes = inst.get("nodes");
        for (n, f) in nodes.fields {
            println!("{} {}", f.type_name, n);
        }
    }

    Ok(())
}