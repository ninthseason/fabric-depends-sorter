use crate::graph::*;
use crate::mod_t::*;
use crate::Opt;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn real_main(opt: Opt) -> i32 {
    let mods = match fetch_mods(&opt) {
        Ok(value) => value,
        Err(value) => return value,
    };
    let graph = build_graph(&mods);
    let v = petgraph::algo::toposort(&graph, None).unwrap();
    let sorted_mods = v
        .iter()
        .map(|i| graph[*i].filename.to_string())
        .collect::<Vec<String>>();
    for (index, filename) in sorted_mods.iter().enumerate() {
        fs::rename(
            opt.directory.join(&filename),
            opt.directory.join(format!("{:05}_{}", index + 1, filename)),
        )
        .unwrap();
    }
    0
}

fn fetch_mods(opt: &Opt) -> Result<Vec<FabricMod>, i32> {
    let all_files = std::fs::read_dir(&opt.directory).unwrap();
    let mod_files: Vec<_> = all_files
        .map(|x| x.unwrap())
        .map(|x| x.path())
        .filter(|x| {
            x.is_file()
                && x.file_name().unwrap().to_str().unwrap().ends_with(".jar")
                && !opt
                    .ignores
                    .contains(&x.file_name().unwrap().to_str().unwrap().to_string())
        })
        .collect();
    if mod_files.is_empty() {
        println!("No mods found in {}", opt.directory.display());
        return Err(1);
    }
    println!("Found {} mods", mod_files.len());
    let mods = if opt.threads > 1 {
        println!("Multithread mode. Using {} threads.", opt.threads,);
        let mod_files_split = mod_files.chunks(mod_files.len() / opt.threads);
        let mods: Arc<Mutex<Vec<FabricMod>>> = Arc::new(Mutex::new(Vec::new()));
        let mut handles = vec![];
        for jars in mod_files_split {
            let jars = jars.to_owned();
            let mods = Arc::clone(&mods);
            handles.push(thread::spawn(move || {
                let mut part_mods = FabricMod::from_jars(jars);
                mods.lock().unwrap().append(&mut part_mods);
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        Arc::try_unwrap(mods).unwrap().into_inner().unwrap()
    } else {
        FabricMod::from_jars(mod_files)
    };
    Ok(mods)
}
