use crate::{entity::EntityId, geometry::mbr::Mbr};

pub type NodeId = usize;

const MAX_ENTRIES: usize = 4;

#[derive(Clone)]
pub struct RTree {
    nodes: Vec<Node>,
    root: Option<NodeId>,
}

#[derive(Clone)]
pub struct Node {
    pub entries: Vec<Entry>,
    pub parent: Option<NodeId>,
}

#[derive(Clone)]
pub enum Entry {
    Leaf { mbr: Mbr, entity_id: EntityId },
    Internal { mbr: Mbr, child: NodeId },
}

impl RTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
        }
    }

    pub fn insert(&mut self, entity_id: EntityId, mbr: Mbr) {
        let Some(root_id) = self.root else {
            let node_id = self.nodes.len();

            self.nodes.push(Node {
                entries: vec![Entry::Leaf { mbr, entity_id }],
                parent: None,
            });

            self.root = Some(node_id);

            return;
        };

        // root_id 하위의 요소들 중 mbr을 가지고 들어가기 좋은 곳을 탐색
        let leaf_id = self.choose_leaf(root_id, &mbr);
        self.nodes[leaf_id]
            .entries
            .push(Entry::Leaf { mbr, entity_id });

        if self.nodes[leaf_id].entries.len() <= MAX_ENTRIES {
            return;
        }

        let (left_id, right_id) = self.split_leaf(leaf_id);
        let parent = self.nodes[leaf_id].parent;

        match parent {
            None => self.create_new_root(left_id, right_id),
            Some(parent_id) => {
                self.replace_child(parent_id, leaf_id, left_id, right_id);
            }
        }
    }

    fn create_new_root(&mut self, left_id: NodeId, right_id: NodeId) {
        let new_root_id = self.nodes.len();
        let left_mbr = self.compute_node_mbr(left_id);
        let right_mbr = self.compute_node_mbr(right_id);

        self.nodes.push(Node {
            entries: vec![
                Entry::Internal {
                    mbr: left_mbr,
                    child: left_id,
                },
                Entry::Internal {
                    mbr: right_mbr,
                    child: right_id,
                },
            ],
            parent: None,
        });
        self.nodes[left_id].parent = Some(new_root_id);
        self.nodes[right_id].parent = Some(new_root_id);
        self.root = Some(new_root_id);
    }

    fn compute_node_mbr(&self, node_id: NodeId) -> Mbr {
        let node = &self.nodes[node_id];

        let first = match &node.entries[0] {
            Entry::Leaf { mbr, .. } => *mbr,
            Entry::Internal { mbr, .. } => *mbr,
        };

        node.entries.iter().skip(1).fold(first, |current, entry| {
            let entry_mbr = match entry {
                Entry::Leaf { mbr, .. } => *mbr,
                Entry::Internal { mbr, .. } => *mbr,
            };

            current.expand(&entry_mbr)
        })
    }

    pub fn search(&self, query: &Mbr) -> Vec<EntityId> {
        let Some(root_id) = self.root else {
            return Vec::new();
        };

        self.search_node(root_id, query)
    }

    fn search_node(&self, node_id: NodeId, query: &Mbr) -> Vec<EntityId> {
        let mut result = Vec::new();
        let node = &self.nodes[node_id];

        if node.is_leaf() {
            for entry in &node.entries {
                if let Entry::Leaf { mbr, entity_id } = entry {
                    if mbr.intersects(query) {
                        result.push(*entity_id);
                    }
                }
            }

            return result;
        }

        for entry in &node.entries {
            let Entry::Internal { mbr, child } = entry else {
                continue;
            };

            if !mbr.intersects(query) {
                continue;
            }

            let child_result = self.search_node(*child, query);
            result.extend(child_result);
        }

        return result;
    }

    fn split_leaf(&mut self, node_id: NodeId) -> (NodeId, NodeId) {
        let entries = std::mem::take(&mut self.nodes[node_id].entries);
        let mid = entries.len() / 2;

        let mut left_entries = entries;
        let right_entries = left_entries.split_off(mid);

        let left_id = left_entries.len();
        self.nodes.push(Node {
            entries: left_entries,
            parent: None,
        });

        let right_id = right_entries.len();
        self.nodes.push(Node {
            entries: right_entries,
            parent: None,
        });

        (left_id, right_id)
    }

    fn choose_leaf(&self, node_id: NodeId, mbr: &Mbr) -> NodeId {
        let node = &self.nodes[node_id];

        if node.is_leaf() {
            return node_id;
        }

        let mut best_child = None;
        let mut best_enlargement = f64::INFINITY;

        for entry in &node.entries {
            let Entry::Internal {
                mbr: child_mbr,
                child,
            } = entry
            else {
                continue;
            };

            let enlargetment = child_mbr.enlargement(mbr);
            if enlargetment < best_enlargement {
                best_enlargement = enlargetment;
                best_child = Some(*child);
            }
        }

        self.choose_leaf(best_child.expect("Internal node has no children"), mbr)
    }

    fn replace_child(
        &mut self,
        parent_id: NodeId,
        old_child_id: NodeId,
        left_id: NodeId,
        right_id: NodeId,
    ) {
        let left_mbr = self.compute_node_mbr(left_id);
        let right_mbr = self.compute_node_mbr(right_id);

        let parent = &mut self.nodes[parent_id];
        let index = parent
            .entries
            .iter()
            .position(|entry| match entry {
                Entry::Internal { child, .. } => *child == old_child_id,
                Entry::Leaf { .. } => false,
            })
            .expect("child not found");

        parent.entries.remove(index);
        parent.entries.insert(
            index,
            Entry::Internal {
                mbr: left_mbr,
                child: left_id,
            },
        );

        parent.entries.insert(
            index + 1,
            Entry::Internal {
                mbr: right_mbr,
                child: right_id,
            },
        );

        self.nodes[left_id].parent = Some(parent_id);
        self.nodes[right_id].parent = Some(parent_id);

        if self.nodes[parent_id].entries.len() > MAX_ENTRIES {
            let grandparent = self.nodes[parent_id].parent;
            let (left_id, right_id) = self.split_internal(parent_id);

            match grandparent {
                Some(grandparent_id) => {
                    self.insert_split(grandparent_id, parent_id, left_id, right_id);
                }

                None => {
                    self.create_new_root(left_id, right_id);
                }
            }
        }
    }

    fn split_internal(&mut self, node_id: NodeId) -> (NodeId, NodeId) {
        let entries = std::mem::take(&mut self.nodes[node_id].entries);

        let mid = entries.len() / 2;

        let mut left_entries = entries;
        let right_entries = left_entries.split_off(mid);

        let left_id = self.nodes.len();
        self.nodes.push(Node {
            entries: left_entries,
            parent: None,
        });

        let right_id = self.nodes.len();
        self.nodes.push(Node {
            entries: right_entries,
            parent: None,
        });

        let left_children: Vec<NodeId> = self.nodes[left_id]
            .entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Internal { child, .. } => Some(*child),
                Entry::Leaf { .. } => None,
            })
            .collect();

        for child_id in left_children {
            self.nodes[child_id].parent = Some(left_id);
        }

        let right_children: Vec<NodeId> = self.nodes[right_id]
            .entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Internal { child, .. } => Some(*child),
                Entry::Leaf { .. } => None,
            })
            .collect();

        for child_id in right_children {
            self.nodes[child_id].parent = Some(right_id);
        }

        (left_id, right_id)
    }

    fn insert_split(
        &mut self,
        parent_id: NodeId,
        old_child_id: NodeId,
        left_id: NodeId,
        right_id: NodeId,
    ) {
        let grandparent = self.nodes[parent_id].parent;

        let index = {
            let parent = &self.nodes[parent_id];

            parent
                .entries
                .iter()
                .position(|entry| match entry {
                    Entry::Internal { child, .. } => *child == old_child_id,
                    Entry::Leaf { .. } => false,
                })
                .expect("child not found")
        };

        let left_mbr = self.compute_node_mbr(left_id);
        let right_mbr = self.compute_node_mbr(right_id);

        {
            let parent = &mut self.nodes[parent_id];
            parent.entries.remove(index);

            parent.entries.insert(
                index,
                Entry::Internal {
                    mbr: left_mbr,
                    child: left_id,
                },
            );

            parent.entries.insert(
                index + 1,
                Entry::Internal {
                    mbr: right_mbr,
                    child: right_id,
                },
            );
        }
        self.nodes[left_id].parent = Some(parent_id);
        self.nodes[right_id].parent = Some(parent_id);

        if self.nodes[parent_id].entries.len() <= MAX_ENTRIES {
            return;
        }

        let (new_left, new_right) = self.split_internal(parent_id);
        match grandparent {
            Some(grandparent_id) => {
                self.insert_split(grandparent_id, parent_id, new_left, new_right);
            }

            None => {
                let new_root_id = self.nodes.len();
                let left_mbr = self.compute_node_mbr(new_left);
                let right_mbr = self.compute_node_mbr(new_right);

                self.nodes.push(Node {
                    parent: None,
                    entries: vec![
                        Entry::Internal {
                            mbr: left_mbr,
                            child: new_left,
                        },
                        Entry::Internal {
                            mbr: right_mbr,
                            child: new_right,
                        },
                    ],
                });
                self.nodes[new_left].parent = Some(new_root_id);
                self.nodes[new_right].parent = Some(new_root_id);
                self.root = Some(new_root_id);
            }
        }
    }

    pub fn remove(&mut self, entity_id: EntityId) -> bool {
        let Some(root_id) = self.root else {
            return false;
        };

        self.remove_from_node(root_id, entity_id)
    }

    fn remove_from_node(&mut self, node_id: NodeId, entity_id: EntityId) -> bool {
        let is_leaf = self.nodes[node_id].is_leaf();

        if is_leaf {
            let node = &mut self.nodes[node_id];
            let index = node.entries.iter().position(
                |entry| matches!(entry, Entry::Leaf { entity_id: id, .. } if *id == entity_id),
            );

            if let Some(index) = index {
                node.entries.remove(index);
                return true;
            }
            return false;
        }

        let children: Vec<NodeId> = self.nodes[node_id]
            .entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Internal { child, .. } => Some(*child),
                Entry::Leaf { .. } => None,
            })
            .collect();

        for child_id in children {
            if self.remove_from_node(child_id, entity_id) {
                return true;
            }
        }

        false
    }
}

impl Node {
    pub fn new_leaf(parent: Option<NodeId>) -> Self {
        Self {
            entries: Vec::new(),
            parent,
        }
    }

    pub fn new_internal(parent: Option<NodeId>) -> Self {
        Self {
            entries: Vec::new(),
            parent,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self.entries.first() {
            Some(Entry::Leaf { .. }) => true,
            Some(Entry::Internal { .. }) => false,
            None => false,
        }
    }
}
