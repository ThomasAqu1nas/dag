use std::{collections::{BTreeMap, VecDeque}, error::Error};

#[derive(Debug, Clone, PartialEq)]
pub struct DagNode<T: Clone + PartialEq> {
    sources: Vec<u32>,
    value: T
}

impl<T: Clone + PartialEq> DagNode<T> {
    pub fn sources(&self) -> &Vec<u32> {
        &self.sources
    }

    pub fn sources_len(&self) -> usize {
        self.sources.len()
    }

    pub fn new(sources: Vec<u32>, value: T) -> Self {
        Self { sources, value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DAG<T: Clone + PartialEq> {
    inner: BTreeMap<u32, DagNode<T>>,
}

impl<T: Clone + PartialEq> DAG<T> {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    } 

    pub fn add_node(&mut self, node_id: u32, node: DagNode<T>) -> Result<Option<DagNode<T>>, Box<dyn Error>> {
        let mut cloned = self.clone();
        let prev = cloned.inner.insert(node_id, node);
    
        if cloned.sort().is_none() {
            return Err(Box::<dyn Error>::from("Cycling"));
        }
    
        self.inner.insert(node_id, cloned.inner.get(&node_id).unwrap().clone());
        Ok(prev)
    }

    pub fn in_degree(&self, node: u32) -> Option<usize> {
        self.inner.get(&node)
            .map(|node_sources| node_sources.sources().len())
    }
}


pub trait TopologicalSort {
    // None if Graph is cycling
    fn sort(&self) -> Option<Vec<u32>>;
}

impl<T: Clone + PartialEq> TopologicalSort for DAG<T> {
    fn sort(&self) -> Option<Vec<u32>> {
        let mut inner = self.inner.clone();

        let mut queue = VecDeque::<u32>::with_capacity(inner.len());
        for (&node_id, node) in inner.iter() {
            if node.sources_len() == 0 {
                queue.push_back(node_id);
            }
        }

        let mut sorted = Vec::with_capacity(inner.len());

        while let Some(node_id) = queue.pop_front() {
            sorted.push(node_id);

            for (&other_id, node) in inner.iter_mut() {
                if let Some(pos) = node.sources.iter().position(|&src| src == node_id) {
                    node.sources.remove(pos);
                    if node.sources_len() == 0 {

                        if !sorted.contains(&other_id) && !queue.contains(&other_id) {
                            queue.push_back(other_id);
                        }
                    }
                }
            }
        }

        if sorted.len() != inner.len() {
            None
        } else {
            Some(sorted)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort_acyclic() {

        let mut dag_acyclic = DAG::new();
        dag_acyclic.inner.insert(1, DagNode { sources: vec![], value: 0u32 });
        dag_acyclic.inner.insert(2, DagNode { sources: vec![1], value: 0u32 });
        dag_acyclic.inner.insert(3, DagNode { sources: vec![1], value: 0u32 });
        dag_acyclic.inner.insert(4, DagNode { sources: vec![2, 3], value: 0u32 });

        // Must return Some(sorted_order)
        let sorted_option = dag_acyclic.sort();
        assert!(sorted_option.is_some(), "Ациклический граф должен быть отсортирован");
        let sorted = sorted_option.unwrap();

        let pos = |node: u32| sorted.iter().position(|&n| n == node).unwrap();
        assert!(pos(1) < pos(2), "Вершина 1 должна идти раньше 2");
        assert!(pos(1) < pos(3), "Вершина 1 должна идти раньше 3");
        assert!(pos(2) < pos(4), "Вершина 2 должна идти раньше 4");
        assert!(pos(3) < pos(4), "Вершина 3 должна идти раньше 4");
    }

    #[test]
    fn test_topological_sort_cyclic() {

        let mut dag_cyclic = DAG::new();
        dag_cyclic.inner.insert(1, DagNode { sources: vec![3], value: 0u32 });
        dag_cyclic.inner.insert(2, DagNode { sources: vec![1], value: 0u32 });
        dag_cyclic.inner.insert(3, DagNode { sources: vec![2], value: 0u32 });


        let sorted_option = dag_cyclic.sort();
        assert!(sorted_option.is_none(), "Граф с циклом должен вернуть None");
    }
}
