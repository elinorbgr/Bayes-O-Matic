use loopybayesnet::BayesNet;
use ndarray::{ArrayD, IxDyn};

#[derive(Debug)]
pub struct Node {
    pub parents: Vec<usize>,
    pub children: Vec<usize>,
    pub label: String,
    pub values: Vec<String>,
    pub credencies: Option<ArrayD<f32>>,
    pub evidence: Option<usize>,
}

#[derive(Copy, Clone, Debug)]
pub enum EdgeError {
    BadNode,
    WouldCycle,
    AlreadyExisting,
}

#[derive(Debug)]
pub struct DAG {
    nodes: Vec<Option<Node>>,
}

impl DAG {
    pub fn new() -> DAG {
        DAG { nodes: Vec::new() }
    }

    pub fn insert_node(&mut self) -> usize {
        let new_node = Node {
            parents: Vec::new(),
            children: Vec::new(),
            label: String::new(),
            values: Vec::new(),
            credencies: None,
            evidence: None,
        };
        if let Some(id) = self.nodes.iter().position(|n| n.is_none()) {
            self.nodes[id] = Some(new_node);
            id
        } else {
            self.nodes.push(Some(new_node));
            self.nodes.len() - 1
        }
    }

    pub fn check_edge_addition(&self, child: usize, parent: usize) -> Result<(), EdgeError> {
        if let Some(&Some(ref node)) = self.nodes.get(parent) {
            if parent == child {
                return Err(EdgeError::WouldCycle);
            }
            if node.children.contains(&child) {
                return Err(EdgeError::AlreadyExisting);
            }
            let mut ancestors = node.parents.clone();
            let mut visited = vec![parent];
            // iteratively check all ancestors for equality with the child, if we find
            // any adding this edge would create a cycle
            loop {
                let id = match ancestors.pop() {
                    Some(v) => v,
                    None => break,
                };
                if id == child {
                    return Err(EdgeError::WouldCycle);
                }
                if visited.contains(&id) {
                    continue;
                }
                visited.push(id);
                ancestors.extend(&self.nodes[id].as_ref().unwrap().parents);
            }
        } else {
            return Err(EdgeError::BadNode);
        }
        Ok(())
    }

    pub fn add_edge(&mut self, child: usize, parent: usize) -> Result<(), EdgeError> {
        // check if a cycle would be created...
        self.check_edge_addition(child, parent)?;

        // no cycle, all is good, insert
        if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(child) {
            node.parents.push(parent);
            // reset the credencies when changing the parents
            node.credencies = None;
        } else {
            return Err(EdgeError::BadNode);
        }
        // also insert as a child to the parent
        let node = self.nodes[parent].as_mut().unwrap();
        if !node.children.contains(&child) {
            node.children.push(child);
        }
        Ok(())
    }

    pub fn remove_edge(&mut self, child: usize, parent: usize) {
        if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(child) {
            node.parents.retain(|&v| v != parent);
            // reset the credencies when changing the parents
            node.credencies = None;
        }
        if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(parent) {
            node.children.retain(|&v| v != child);
        }
    }

    pub fn add_value(&mut self, node: usize, value: String) {
        let children = if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(node) {
            node.values.push(value);
            // reset the credencies when changing the values
            node.credencies = None;
            node.evidence = None;
            node.children.clone()
        } else {
            Vec::new()
        };
        // also reset the credencies of the children
        for child in children {
            if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(child) {
                node.credencies = None;
            }
        }
    }

    pub fn remove_value(&mut self, node: usize, value_id: usize) {
        if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(node) {
            node.values.remove(value_id);
            // reset the credencies when changing the values
            node.credencies = None;
            node.evidence = None;
        }
    }

    pub fn set_label(&mut self, node: usize, label: String) {
        if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(node) {
            node.label = label;
        }
    }

    pub fn set_credencies(&mut self, node: usize, credencies: ArrayD<f32>) -> Result<(), ()> {
        // sanity check, the dimensions of the array must match
        if let Some(&Some(ref node)) = self.nodes.get(node) {
            let mut shape = vec![node.values.len()];
            for &p in &node.parents {
                shape.push(self.nodes[p].as_ref().unwrap().values.len());
            }
            if credencies.shape() != &shape[..] {
                return Err(());
            }
        }

        if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(node) {
            node.credencies = Some(credencies);
        }

        Ok(())
    }

    pub fn set_evidence(&mut self, node: usize, evidence: Option<usize>) {
        if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(node) {
            node.evidence = evidence;
        }
    }

    pub fn get(&self, id: usize) -> Option<&Node> {
        self.nodes.get(id).and_then(|o| o.as_ref())
    }

    pub fn make_bayesnet(&self) -> Result<(BayesNet, Vec<usize>), ()> {
        // Order the nodes of the graph into a topological order for insertion into
        // loopybayesnet
        let mut order = Vec::new();
        fn visit(nodes: &[Option<Node>], order: &mut Vec<usize>, n: usize) {
            if order.contains(&n) {
                return;
            }
            for &c in &nodes.get(n).unwrap().as_ref().unwrap().children {
                visit(nodes, order, c);
            }
            order.push(n);
        }
        for (i, node) in self.nodes.iter().enumerate() {
            if node.is_some() {
                visit(&self.nodes, &mut order, i);
            }
        }
        order.reverse();

        // order now contains a topological ordering of the nodes of the graph, which we will now
        // feed into loopybayesnet
        let mut net = BayesNet::new();

        // a map for reverse indexing the nodes from our indices indices to loopybayesnet ones
        let mut map: Vec<Option<usize>> = vec![None; self.nodes.len()];
        let mut evidence = Vec::new();
        // insert the nodes in the bayesnet
        for &n in &order {
            let node = self.nodes[n].as_ref().unwrap();
            // early return if any node has no values
            if node.values.len() == 0 {
                return Err(());
            }

            let mut parent_ids = Vec::new();
            let mut values_count = vec![node.values.len()];
            for &p in &node.parents {
                parent_ids.push(map[p].unwrap());
                values_count.push(self.nodes[p].as_ref().unwrap().values.len());
            }
            let credencies_data = node.credencies.clone().unwrap_or_else(|| {
                let count = values_count.iter().fold(1, |a, b| a * b);
                ArrayD::from_shape_vec(IxDyn(&values_count), vec![0.0; count]).unwrap()
            });
            let log_probas = credencies_data * 10f32.ln();
            let loopy_id = net.add_node_from_log_probabilities(&parent_ids, log_probas);
            map[n] = Some(loopy_id);

            // collect the evidence
            if let Some(ev) = node.evidence {
                evidence.push((loopy_id, ev));
            }
        }

        net.set_evidence(&evidence);

        Ok((net, order))
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = (usize, &Node)> {
        self.nodes.iter().enumerate().filter_map(|(i, n)| match n {
            Some(ref n) => Some((i, n)),
            None => None,
        })
    }

    pub fn reset(&mut self) {
        self.nodes.clear();
    }

    pub fn to_json(&self) -> String {
        unimplemented!();
    }

    pub fn from_json(json: &str) -> Result<DAG, serde_json::error::Error> {
        unimplemented!();
    }
}
