use loopybayesnet::BayesNet;
use ndarray::{ArrayD, IxDyn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Node {
    pub parents: Vec<usize>,
    pub children: Vec<usize>,
    pub label: String,
    pub description: String,
    pub values: Vec<String>,
    pub credencies: Option<ArrayD<f32>>,
    pub cred_description: Vec<String>,
    pub observation: Option<usize>,
}

#[derive(Copy, Clone, Debug)]
pub enum EdgeError {
    BadNode,
    WouldCycle,
    AlreadyExisting,
}

#[derive(Debug)]
pub struct Dag {
    nodes: Vec<Option<Node>>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(into = "f32", from = "OptFloat")]
struct Float(f32);

#[derive(Deserialize, Clone, Copy)]
#[serde(transparent)]
struct OptFloat(Option<f32>);

impl From<OptFloat> for Float {
    fn from(o: OptFloat) -> Float {
        Float(o.0.unwrap_or(std::f32::NEG_INFINITY))
    }
}

impl From<Float> for f32 {
    fn from(f: Float) -> f32 {
        f.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonNode {
    label: String,
    #[serde(default)]
    description: String,
    values: Vec<String>,
    parents: Vec<usize>,
    observation: Option<usize>,
    credencies: Option<Vec<Float>>,
    #[serde(default)]
    cred_description: Vec<String>,
}

#[derive(Debug)]
pub enum DeserError {
    Json(serde_json::Error),
    Graph(EdgeError),
}

impl Dag {
    pub fn new() -> Dag {
        Dag { nodes: Vec::new() }
    }

    pub fn estimate_iteration_number(&self) -> usize {
        2 * self.nodes.len()
    }

    pub fn insert_node(&mut self) -> usize {
        let new_node = Node {
            parents: Vec::new(),
            children: Vec::new(),
            description: String::new(),
            label: String::new(),
            values: Vec::new(),
            credencies: None,
            cred_description: Vec::new(),
            observation: None,
        };
        if let Some(id) = self.nodes.iter().position(|n| n.is_none()) {
            self.nodes[id] = Some(new_node);
            id
        } else {
            self.nodes.push(Some(new_node));
            self.nodes.len() - 1
        }
    }

    pub fn duplicate_node(&mut self, node: usize) -> Option<usize> {
        self.get(node)?;
        let new_node = self.insert_node();
        // duplicate the node
        self.nodes[new_node] = self.nodes[node].clone();
        // properly update the parents & children though
        let (new_parents, new_children, label, credencies) = {
            let new_node = self.nodes[new_node].as_mut().unwrap();
            let new_parents = std::mem::take(&mut new_node.parents);
            let new_children = std::mem::take(&mut new_node.children);
            let new_label = std::mem::take(&mut new_node.label);
            let credencies = new_node.credencies.take();
            (new_parents, new_children, new_label, credencies)
        };
        for p in new_parents {
            self.add_edge(new_node, p).unwrap();
        }
        for c in new_children {
            self.add_edge(c, new_node).unwrap();
        }
        // update the label to differentiate
        self.set_label(new_node, format!("{} (bis)", label));
        // set back the credency matrix that was lost in the process
        self.nodes[new_node].as_mut().unwrap().credencies = credencies;
        Some(new_node)
    }

    pub fn remove_node(&mut self, node: usize) {
        let (old_parents, old_children) = if let Some(old_node) = self.get(node) {
            (old_node.parents.clone(), old_node.children.clone())
        } else {
            return;
        };
        for p in old_parents {
            self.remove_edge(node, p);
        }
        for c in old_children {
            self.remove_edge(c, node);
        }
        self.nodes[node] = None;
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
            while let Some(id) = ancestors.pop() {
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

    fn count_parent_values(&self, node: usize) -> usize {
        if let Some(&Some(ref node)) = self.nodes.get(node) {
            let mut values = 1;
            for &p in &node.parents {
                values *= self.nodes[p].as_ref().unwrap().values.len();
            }
            values
        } else {
            0
        }
    }

    pub fn add_edge(&mut self, child: usize, parent: usize) -> Result<(), EdgeError> {
        // check if a cycle would be created...
        self.check_edge_addition(child, parent)?;

        // no cycle, all is good, insert
        if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(child) {
            node.parents.push(parent);
            // reset the credencies when changing the parents
            node.credencies = None;
            node.cred_description = Vec::new();
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
            node.cred_description = Vec::new();
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
            node.cred_description = Vec::new();
            node.observation = None;
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
            node.cred_description = Vec::new();
            node.observation = None;
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

    pub fn set_observation(&mut self, node: usize, observation: Option<usize>) {
        if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(node) {
            node.observation = observation;
        }
    }

    pub fn set_description(&mut self, node: usize, description: String) {
        if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(node) {
            node.description = description;
        }
    }

    pub fn set_cred_descriptions(
        &mut self,
        node: usize,
        descriptions: Vec<String>,
    ) -> Result<(), ()> {
        let parent_values = self.count_parent_values(node);
        if descriptions.len() != parent_values {
            return Err(());
        }
        if let Some(&mut Some(ref mut node)) = self.nodes.get_mut(node) {
            node.cred_description = descriptions;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get(&self, id: usize) -> Option<&Node> {
        self.nodes.get(id).and_then(|o| o.as_ref())
    }

    fn compact_ids(&self) -> (Vec<usize>, Vec<Option<usize>>) {
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

        // a map for reverse indexing the nodes from our indices indices to compacted ones
        let mut map: Vec<Option<usize>> = vec![None; self.nodes.len()];

        for (i, &n) in order.iter().enumerate() {
            map[n] = Some(i);
        }

        (order, map)
    }

    pub fn make_bayesnet(&self) -> Result<(BayesNet, Vec<usize>), ()> {
        let (order, map) = self.compact_ids();
        // order now contains a topological ordering of the nodes of the graph,
        // which we will now feed into loopybayesnet
        let mut net = BayesNet::new();
        let mut observation = Vec::new();
        // insert the nodes in the bayesnet
        for &n in &order {
            let node = self.nodes[n].as_ref().unwrap();
            // early return if any node has no values
            if node.values.is_empty() {
                return Err(());
            }

            let mut parent_ids = Vec::new();
            let mut values_count = vec![node.values.len()];
            for &p in &node.parents {
                parent_ids.push(map[p].unwrap());
                values_count.push(self.nodes[p].as_ref().unwrap().values.len());
            }
            let credencies_data = node.credencies.clone().unwrap_or_else(|| {
                let count = values_count.iter().product();
                ArrayD::from_shape_vec(IxDyn(&values_count), vec![0.0; count]).unwrap()
            });
            let log_probas = credencies_data * 10f32.ln();
            let loopy_id = net.add_node_from_log_probabilities(&parent_ids, log_probas);

            // collect the observation
            if let Some(ev) = node.observation {
                observation.push((loopy_id, ev));
            }
        }

        net.set_evidence(&observation);

        Ok((net, order))
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = (usize, &Node)> {
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(i, n)| n.as_ref().map(|n| (i, n)))
    }

    pub fn to_json(&self) -> String {
        let (order, map) = self.compact_ids();
        let mut nodelist: Vec<JsonNode> = Vec::with_capacity(order.len());

        for &n in &order {
            let node = self.nodes[n].as_ref().unwrap();
            nodelist.push(JsonNode {
                label: node.label.clone(),
                values: node.values.clone(),
                description: node.description.clone(),
                parents: node
                    .parents
                    .iter()
                    .map(|&i| map[i].unwrap())
                    .collect::<Vec<_>>(),
                observation: node.observation,
                credencies: node
                    .credencies
                    .as_ref()
                    .map(|a| a.iter().map(|&f| Float(f)).collect()),
                cred_description: node.cred_description.clone(),
            });
        }

        serde_json::to_string_pretty(&nodelist).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Dag, DeserError> {
        let contents: Vec<JsonNode> = serde_json::from_str(json).map_err(DeserError::Json)?;

        let mut dag = Dag::new();

        for node in &contents {
            let id = dag.insert_node();
            dag.set_label(id, node.label.clone());
            for &p in &node.parents {
                dag.add_edge(id, p).map_err(DeserError::Graph)?;
            }
            for v in &node.values {
                dag.add_value(id, v.into());
            }
            dag.set_observation(id, node.observation);
            dag.set_description(id, node.description.clone());
            // ingore bad descriptions
            let _ = dag.set_cred_descriptions(id, node.cred_description.clone());
            // and the credencies
            if let Some(ref array) = node.credencies {
                let mut shape = vec![node.values.len()];
                for &p in &node.parents {
                    shape.push(contents[p].values.len());
                }
                let array = match ArrayD::from_shape_vec(
                    IxDyn(&shape),
                    array.iter().map(|&f| f.into()).collect::<Vec<f32>>(),
                )
                .ok()
                {
                    Some(a) => a,
                    None => continue, // ignore bad arrays
                };
                // ignore bad arrays
                let _ = dag.set_credencies(id, array);
            }
        }

        Ok(dag)
    }
}
