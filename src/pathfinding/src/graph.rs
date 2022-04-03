use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    hash::Hash,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vertex<V: Copy + Clone + Debug + Ord + Hash> {
    pub name: V,
}
impl<V: Copy + Clone + Debug + Ord + Hash> Vertex<V> {
    pub fn new(name: V) -> Self {
        Self { name }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AdjacencyList<V: Copy + Clone + Debug + Display + Ord + Hash, E: Clone> {
    pub hash_map: BTreeMap<Vertex<V>, BTreeMap<Vertex<V>, E>>,
}
impl<V: Copy + Clone + Debug + Display + Ord + Hash, E: Clone> AdjacencyList<V, E> {
    pub fn new(hash_map: BTreeMap<Vertex<V>, BTreeMap<Vertex<V>, E>>) -> Self {
        Self { hash_map }
    }
    pub fn add_vertex(&mut self, vertex: Vertex<V>) {
        self.hash_map.insert(vertex, BTreeMap::new());
    }
    pub fn add_vertex_with_undirected_edges(
        &mut self,
        vertex: Vertex<V>,
        edges: BTreeMap<Vertex<V>, E>,
    ) {
        for (neighbor, weight) in edges {
            self.add_edge_undirected(vertex, neighbor, weight);
        }
    }
    pub fn add_vertex_with_directed_edges(
        &mut self,
        vertex: Vertex<V>,
        edges: BTreeMap<Vertex<V>, E>,
    ) {
        self.hash_map.insert(vertex, edges);
    }
    /// Adds a new directed edge from `a` to `b` with a given `weight`.\
    /// If `a` doesn't exist, it is created along with the edge to `b`.\
    /// If `b` doesn't exist, it is created.
    pub fn add_edge_directed(&mut self, a: Vertex<V>, b: Vertex<V>, weight: E) {
        if let Some(a) = self.get_neighbors_mut(&a) {
            a.insert(b, weight);
        } else {
            self.add_vertex_with_directed_edges(a, BTreeMap::from([(b, weight)]));
        }
        if self.get_neighbors(&b).is_none() {
            self.add_vertex_with_directed_edges(b, BTreeMap::new());
        }
    }
    /// Adds a new undirected edge between `a` and `b` with a given `weight`.\
    /// If `a` doesn't exist, it is created along with the edge to `b`.\
    /// If `b` doesn't exist, it is created along with the edge to `a`.
    pub fn add_edge_undirected(&mut self, a: Vertex<V>, b: Vertex<V>, weight: E) {
        if let Some(a) = self.get_neighbors_mut(&a) {
            a.insert(b, weight.clone());
        } else {
            self.add_vertex_with_directed_edges(a, BTreeMap::from([(b, weight.clone())]));
        }
        if let Some(b) = self.get_neighbors_mut(&b) {
            b.insert(a, weight);
        } else {
            self.add_vertex_with_directed_edges(b, BTreeMap::from([(a, weight)]));
        }
    }
    pub fn remove_vertex(&mut self, vertex: &Vertex<V>) {
        if let Some(edges) = self.get_neighbors(vertex) {
            let edges = edges.iter().map(|n| *n.0).collect::<Vec<Vertex<V>>>();

            for neighbor in edges {
                if let Some(edges) = self.get_neighbors_mut(&neighbor) {
                    edges.remove(vertex);
                }
            }
        }

        self.hash_map.remove(vertex);
    }
    pub fn toggle_vertex(&mut self, vertex: &Vertex<V>, edges: &BTreeMap<Vertex<V>, E>) {
        if self.hash_map.contains_key(vertex) {
            self.remove_vertex(vertex);
        } else {
            self.add_vertex_with_directed_edges(*vertex, edges.to_owned());
        }
    }
    pub fn get_neighbors(&self, vertex: &Vertex<V>) -> Option<&BTreeMap<Vertex<V>, E>> {
        self.hash_map.get(vertex)
    }
    pub fn get_neighbors_mut(&mut self, vertex: &Vertex<V>) -> Option<&mut BTreeMap<Vertex<V>, E>> {
        self.hash_map.get_mut(vertex)
    }
    pub fn into_mermaid(&self) -> String {
        let mut diagram = String::from("flowchart LR");

        for (vertex, edges) in self.hash_map.iter() {
            for neighbor in edges.keys() {
                let edge_type = if self.get_neighbors(neighbor).unwrap().contains_key(vertex) {
                    "---"
                } else {
                    "-->"
                };
                diagram +=
                    format!("\n    {} {} {}", vertex.name, edge_type, neighbor.name).as_str();
            }
        }

        diagram
    }
}
