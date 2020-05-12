use std::collections::{HashSet, VecDeque};
use std::ops::{Index, IndexMut};

use super::{BoundingVolume, Transformable, Triangle, TriangleStorage, AABB};
use math::Transform;
use math::{Point3, Vector3};
use ray::Ray;

#[derive(Debug, Clone, Copy)]
struct NodeId(usize);

impl NodeId {
    const fn none() -> Self {
        NodeId(usize::MAX)
    }
}

impl From<usize> for NodeId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
struct Node<M, T> {
    metadata: M,
    data: T,
}

impl<M: Default, T> Node<M, T> {
    fn new(data: T) -> Self {
        Self {
            data,
            metadata: M::default(),
        }
    }
}

#[derive(Debug)]
struct Arena<M, T> {
    nodes: Vec<Node<M, T>>,
}

impl<M: Default, T> Arena<M, T> {
    fn new() -> Self {
        Self {
            nodes: Vec::default(),
        }
    }

    fn new_node(&mut self, data: T) -> NodeId {
        let next_index = self.nodes.len();

        let node = Node::new(data);

        self.nodes.push(node);

        NodeId::from(next_index)
    }

    fn clear(&mut self) {
        self.nodes.clear();
    }

    fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
}

impl<M, T> Index<NodeId> for Arena<M, T> {
    type Output = Node<M, T>;

    fn index(&self, id: NodeId) -> &Self::Output {
        &self.nodes[id.0]
    }
}

impl<M, T> IndexMut<NodeId> for Arena<M, T> {
    fn index_mut(&mut self, id: NodeId) -> &mut Self::Output {
        &mut self.nodes[id.0]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TriangleId(usize);

impl TriangleId {
    fn value(&self) -> usize {
        self.0
    }
}

impl From<usize> for TriangleId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

pub struct IntersectionIterator<'a, I> {
    ids: I,
    triangles: &'a [Triangle],
}

impl<'a, I> IntersectionIterator<'a, I> {
    fn new(ids: I, triangles: &'a [Triangle]) -> Self {
        Self { ids, triangles }
    }
}

impl<'a, I> Iterator for IntersectionIterator<'a, I>
where
    I: Iterator<Item = TriangleId>,
{
    type Item = &'a Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self
            .ids
            .next()
            .and_then(|triangle_id| self.triangles.get(triangle_id.value()));

        item
    }
}

#[derive(Debug)]
struct Metadata {
    children: [NodeId; 8],
    bounding_box: AABB,
    is_leaf: bool,
    is_empty: bool,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            children: [NodeId::none(); 8],
            bounding_box: AABB::empty(),
            is_leaf: false,
            is_empty: false,
        }
    }
}

const MAX_DEPTH: usize = 6;
const MIN_SIZE: f32 = 0.0005;
const MIN_TRIANGLES_PER_NODE: usize = 1;

#[derive(Debug)]
pub struct Octree {
    triangles: Vec<Triangle>,
    arena: Arena<Metadata, HashSet<TriangleId>>,
    root: NodeId,
}

impl Octree {
    fn rebuild(&mut self) {
        self.arena.clear();

        let root_id = self
            .arena
            .new_node((0..self.triangles.len()).map(TriangleId::from).collect());

        let mut iterator = self.triangles.iter();
        self.root = root_id;
        self.arena[root_id].metadata.bounding_box = AABB::from_triangles(&mut iterator);
        self.build(root_id, 1);

        dbg!(self.arena.num_nodes());
        dbg!(&self.arena[self.root].metadata);
        dbg!(&self.arena[root_id].metadata.bounding_box);

        let all_triangle_ids: HashSet<TriangleId> =
            (0..self.triangles.len()).map(TriangleId::from).collect();

        let mut triangle_ids_in_nodes: HashSet<TriangleId> = HashSet::default();
        let mut to_visit: VecDeque<NodeId> = VecDeque::new();
        let mut num_nodes = 1;
        let mut num_triangles = self.arena[self.root].data.len();
        to_visit.push_back(self.root);

        while let Some(id) = to_visit.pop_back() {
            let child_node = &self.arena[id];
            num_triangles += child_node.data.len();

            if !child_node.metadata.is_leaf {
                for octant_id in child_node.metadata.children.iter().cloned() {
                    to_visit.push_back(octant_id);
                }
            }

            for triangle_id in child_node.data.iter().cloned() {
                triangle_ids_in_nodes.insert(triangle_id);
            }

            num_nodes += 1;
        }

        dbg!(num_triangles / num_nodes);

        assert!(triangle_ids_in_nodes == all_triangle_ids, "Found orphans");
    }

    fn build(&mut self, node_id: NodeId, depth: usize) {
        let (mut child_nodes, mut child_bounding_volumes) = {
            let node = &mut self.arena[node_id];
            let bounding_box = &node.metadata.bounding_box;

            // Exit condition triangles per node
            if node.data.len() <= MIN_TRIANGLES_PER_NODE {
                node.metadata.is_leaf = true;
                node.metadata.is_empty = node.data.is_empty();
                return;
            }

            let dimensions = bounding_box.dimensions();

            // Exit condition smallest node
            if dimensions.x <= MIN_SIZE && dimensions.y <= MIN_SIZE && dimensions.z <= MIN_SIZE {
                node.metadata.is_leaf = true;
                return;
            }

            // Exit condition max depth
            if depth >= MAX_DEPTH {
                node.metadata.is_leaf = true;
                return;
            }

            let child_bounding_volumes = Self::build_octants_for(bounding_box, dimensions);
            let mut to_delete: HashSet<TriangleId> = HashSet::default();
            let mut child_nodes: VecDeque<HashSet<TriangleId>> = VecDeque::with_capacity(8);
            for _ in 0..8 {
                child_nodes.push_back(HashSet::default());
            }

            for triangle_id in &node.data {
                for i in 0..8 {
                    let triangle = &self.triangles[triangle_id.value()];

                    if child_bounding_volumes[i].intersects_triangle(&triangle) {
                        to_delete.insert(*triangle_id);
                        child_nodes[i].insert(*triangle_id);
                        // TODO: Think about this break, is it enough if a triangle is in a single
                        // node or should it be in all that it intersects with?
                        // break;
                    }
                }
            }

            node.data = node.data.difference(&to_delete).cloned().collect();
            assert!(node.data.len() == 0);
            // println!(
            //     "Triangles left in parent({}): {}",
            //     node_id.0,
            //     node.data.len()
            // );
            (child_nodes, child_bounding_volumes)
        };

        for i in 0..8 {
            let id = self.arena.new_node(child_nodes.pop_front().unwrap());
            self.arena[id].metadata.bounding_box = child_bounding_volumes.pop_front().unwrap();

            self.arena[node_id].metadata.children[i] = id;
        }

        for i in 0..8 {
            self.build(self.arena[node_id].metadata.children[i], depth + 1);
        }
    }

    fn build_octants_for(bounding_box: &AABB, dimensions: Vector3) -> VecDeque<AABB> {
        let half = (dimensions * 0.5).as_point();
        let center = (bounding_box.min() + half.as_vector()).as_point();

        let (min, max) = (bounding_box.min(), bounding_box.max());
        let mut child_bounding_volumes = VecDeque::with_capacity(8);
        // Left
        child_bounding_volumes.push_back(AABB::new(min, center));
        child_bounding_volumes.push_back(AABB::new(
            Point3::new(min.x, min.y, center.z),
            Point3::new(center.x, center.y, max.z),
        ));
        child_bounding_volumes.push_back(AABB::new(
            Point3::new(min.x, center.y, min.z),
            Point3::new(center.x, max.y, center.z),
        ));
        child_bounding_volumes.push_back(AABB::new(
            Point3::new(min.x, center.y, center.z),
            Point3::new(center.x, max.y, max.z),
        ));
        // Right
        child_bounding_volumes.push_back(AABB::new(center, max));
        child_bounding_volumes.push_back(AABB::new(
            Point3::new(center.x, center.y, min.z),
            Point3::new(max.x, max.y, center.z),
        ));
        child_bounding_volumes.push_back(AABB::new(
            Point3::new(center.x, min.y, center.z),
            Point3::new(max.x, center.y, max.z),
        ));
        child_bounding_volumes.push_back(AABB::new(
            Point3::new(center.x, min.y, min.z),
            Point3::new(max.x, center.y, center.z),
        ));

        child_bounding_volumes
    }
}

impl Transformable for Octree {
    fn transform(&mut self, transform: &Transform) {
        for triangle in self.all_mut() {
            triangle.transform(&transform);
        }

        self.rebuild();
    }
}

impl<'a> TriangleStorage<'a> for Octree {
    type Iterator = std::slice::Iter<'a, Triangle>;
    type IteratorMut = std::slice::IterMut<'a, Triangle>;
    type IntersectionIterator =
        IntersectionIterator<'a, std::collections::hash_set::IntoIter<TriangleId>>;

    fn new(triangles: Vec<Triangle>) -> Self {
        let mut arena = Arena::new();
        let root_id = arena.new_node((0..triangles.len()).map(TriangleId::from).collect());

        let mut tree = Self {
            triangles,
            arena,
            root: root_id,
        };

        tree.rebuild();

        tree
    }

    fn intersect(&'a self, ray: Ray, _cull: bool) -> Self::IntersectionIterator {
        let mut node_ids: VecDeque<NodeId> = VecDeque::new();
        node_ids.push_front(self.root);
        let mut triangles_to_test = HashSet::new();

        while let Some(id) = node_ids.pop_back() {
            let child_node = &self.arena[id];

            if child_node.metadata.bounding_box.intersect(ray) {
                if !child_node.metadata.is_leaf {
                    for octant_id in child_node.metadata.children.iter().cloned() {
                        node_ids.push_back(octant_id);
                    }
                }

                for triangle_id in child_node.data.iter().cloned() {
                    triangles_to_test.insert(triangle_id);
                }
            }
        }

        IntersectionIterator::new(triangles_to_test.into_iter(), &self.triangles)
    }

    fn all(&'a self) -> Self::Iterator {
        self.triangles.iter()
    }

    fn all_mut(&'a mut self) -> Self::IteratorMut {
        self.triangles.iter_mut()
    }
}
