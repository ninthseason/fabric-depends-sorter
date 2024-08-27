use petgraph::Graph;

use crate::mod_t::{FabricMod, FabricModLite};

pub fn build_graph(mods: &Vec<FabricMod>) -> Graph<FabricModLite, i8> {
    let mut graph = Graph::<FabricModLite, i8>::new();
    let mut indexes = vec![];
    for item in mods {
        let idx = graph.add_node(FabricModLite {
            filename: item.filename.clone(),
            id: item.id.clone(),
        });
        indexes.push(idx);
    }
    for (idx, item) in mods.iter().enumerate() {
        for (k, _) in item.depends.iter() {
            for (dep_idx, dep_item) in mods.iter().enumerate() {
                if &dep_item.id == k {
                    graph.add_edge(indexes[dep_idx], indexes[idx], 1);
                }
            }
        }
    }
    graph
}
