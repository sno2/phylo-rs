pub mod simple_rtree;

use std::collections::{HashMap, HashSet};

use crate::{node::*, taxa};
use crate::tree::simple_rtree::*;
use crate::iter::{node_iter::*, edge_iter::*};

pub struct UnrootedPhyloTree{
    nodes: HashMap<NodeID, NodeType>,
    neighbours: HashMap<NodeID, HashSet<(Option<EdgeWeight>, NodeID)>>,
    leaves: HashMap<NodeID, String>,
}

#[derive(Debug)]
pub struct RootedPhyloTree{
    root: NodeID,
    nodes: HashMap<NodeID, NodeType>,
    children: HashMap<NodeID, HashSet<NodeID>>,
    distance_matrix: HashMap<(NodeID,NodeID),EdgeWeight>,
    parents: HashMap<NodeID, Option<NodeID>>,
    leaves: HashMap<NodeID, String>,
}


impl RootedPhyloTree{
    pub fn new()->Self{
        RootedPhyloTree { 
            root: 0,
            nodes: HashMap::from([(0, false)]),
            children: HashMap::new(),
            distance_matrix: HashMap::new(),
            parents: HashMap::from([(0, None)]),
            leaves: HashMap::new()
        }
    }

    pub fn from_newick(newick_string: String)->Self{
        todo!()
    }

    fn leaves_of_node(&self, node_id:&NodeID, leaves:&mut Vec<NodeID>){
        if self.get_children(node_id).expect("Invalid NodeID!").is_empty(){
            leaves.push(*node_id);
        }

        for child_node_id in self.get_children(node_id).expect("Invalid NodeID").iter(){
            self.leaves_of_node(child_node_id, leaves);
        }
    }
}

impl SimpleRTree for RootedPhyloTree{
    fn add_node(&mut self,is_leaf:bool)->NodeID {
        let n : NodeID = self.nodes.len();
        self.nodes.insert(n, is_leaf);
        return n
    }

    fn assign_taxa(&mut self,node:&NodeID, taxa:&str) {
        self.leaves.insert(*node, String::from(taxa));   
    }

    fn add_child(&mut self,parent:&NodeID, child:NodeID,distance:EdgeWeight) {
        let hs = self.children.get_mut(parent);
        match hs{
            None => {
                let mut h:HashSet<NodeID> = HashSet::new();
                h.insert(child);
                self.children.insert(*parent, h);
            },
            Some(x) =>{
                x.insert(child);
            }
        }
    }
    fn get_root(&self)->&NodeID{
        &self.root
    }

    fn get_nodes(&self)->&HashMap<NodeID, NodeType>{
        &self.nodes
    }

    fn get_children(&self, node_id: &NodeID)->Option<&HashSet<NodeID>>{
        self.children.get(node_id)
    }

    fn get_leaves(&self, node_id: &NodeID)->HashSet<NodeID>{
        let mut leaf_vec: Vec<NodeID> = Vec::new();
        self.leaves_of_node(node_id, &mut leaf_vec);
        leaf_vec.into_iter().collect::<HashSet<NodeID>>()
    }

    fn get_subtree(&self, node_id: &NodeID)->Box<dyn SimpleRTree>{
        let root= node_id.clone();
        let mut nodes: HashMap<NodeID, NodeType>= HashMap::new();
        let mut children: HashMap<NodeID, HashSet<NodeID>> = HashMap::new();
        let mut parents: HashMap<NodeID, Option<NodeID>> = HashMap::new();
        let mut leaves: HashMap<NodeID, String> = HashMap::new();
        for descendant_node_id in self.iter_node_pre(node_id){
            nodes.insert(descendant_node_id.clone(), self.nodes.get(&descendant_node_id).expect("Invalid NodeID!").clone());
            children.insert(descendant_node_id.clone(), self.children.get(&descendant_node_id).expect("Invalid NodeID!").clone());
            parents.insert(descendant_node_id.clone(), self.parents.get(&descendant_node_id).expect("Invalid NodeID!").clone());
            if self.is_leaf(&descendant_node_id){
                leaves.insert(descendant_node_id.clone(), self.leaves.get(&descendant_node_id).cloned().unwrap());
            }
        }
        Box::new(
            RootedPhyloTree{
                root: root,
                nodes: nodes,
                children: children,
                distance_matrix:self.distance_matrix.clone(),
                parents: parents,
                leaves: leaves,
            }
        )
    }

    fn get_mrca(&self, node_id_list: Vec<&NodeID>)->&NodeID{
        todo!()
    }

    fn is_leaf(&self, node_id: &NodeID)->bool{
        self.nodes.get(node_id).expect("Invalid NodeID").clone()
    }

    fn graft_subtree(&mut self, tree: Box<dyn SimpleRTree>, edge: (&NodeID, &NodeID)){
        todo!()
    }

    fn extract_subtree(&mut self, node_id: &NodeID)-> Box<dyn SimpleRTree>{
        let root= node_id.clone();
        let mut nodes: HashMap<NodeID, NodeType>= HashMap::new();
        let mut children: HashMap<NodeID, HashSet<NodeID>> = HashMap::new();
        let mut parents: HashMap<NodeID, Option<NodeID>> = HashMap::new();
        let mut leaves: HashMap<NodeID, String> = HashMap::new();
        for decsendant_node_id in self.iter_node_pre(node_id){
            nodes.insert(decsendant_node_id.clone(), self.nodes.remove(&decsendant_node_id).expect("Invalid NodeID!").clone());
            children.insert(decsendant_node_id.clone(), self.children.remove(&decsendant_node_id).expect("Invalid NodeID!").clone());
            parents.insert(decsendant_node_id.clone(), self.parents.remove(&decsendant_node_id).expect("Invalid NodeID!").clone());
            match self.leaves.remove(&decsendant_node_id){
                Some(taxa_id) => {leaves.insert(decsendant_node_id.clone(), taxa_id);},
                None => {},
            }
        }
        Box::new(
            RootedPhyloTree{
                root: root,
                nodes: nodes,
                children: children,
                distance_matrix: self.distance_matrix.clone(),
                parents: parents,
                leaves: leaves,
            }
        )
    }

    fn iter_node_pre(&self, start_node_id: &NodeID)->PreOrdNodes{
        PreOrdNodes::new(start_node_id, &self.children)
    }

    fn iter_node_post(&self, start_node_id: &NodeID)->PostOrdNodes{
        PostOrdNodes::new(start_node_id, &self.children)
    }

    fn iter_edges_pre(&self, start_node_id: &NodeID)->PreOrdEdges{
        todo!()
    }

    fn iter_edges_post(&self, start_node_id: &NodeID)->PostOrdEdges{
        todo!()
    }

    fn get_ancestors(&self, node_id: &NodeID)->Vec<&NodeID>{
        todo!()
    }

    fn leaf_distance_matrix(&self, weighted: bool)->Vec<Vec<EdgeWeight>>{
        todo!()
    }

    fn reroot_at_node(&mut self, node_id: &NodeID){
        self.root = node_id.clone();
    }

    fn insert_internal_node(&mut self, edge: (NodeID, NodeID), edge_weights:(Option<EdgeWeight>, Option<EdgeWeight>)){
        todo!()
    }

    fn distance_from_root(&self, weighted: bool)->f64{
        todo!()
    }

    fn get_bipartition(&self, edge: (&NodeID, &NodeID))->(HashSet<NodeID>, HashSet<NodeID>){
        let c2 = self.get_cluster(edge.1);
        (self.leaves.keys().map(|x| x.clone()).collect::<HashSet<NodeID>>().difference(&c2).map(|x| x.clone()).collect(), c2)
    }

    fn get_cluster(&self, node_id: &NodeID)-> HashSet<NodeID>{
        let mut leaves: Vec<NodeID> = Vec::new();
        self.leaves_of_node(node_id, &mut leaves);
        HashSet::from_iter(leaves)
    }

}

