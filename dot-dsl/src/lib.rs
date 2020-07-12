pub mod graph {
    use self::graph_items::{node::Node, edge::Edge};
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec!(),
                edges: vec!(),
                attrs: HashMap::new()
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            let mut hash_map: HashMap<String, String> = HashMap::new();
            for &value in attrs.into_iter() {
                hash_map.insert(value.0.to_string(), value.1.to_string());
            }
            self.attrs = hash_map;
            self
        }

        pub fn get_node(&self, node_id: &str) -> Option<&Node> {
            println!("{:?}", self.nodes);
            self.nodes.iter()
                .find(|&node| node.value == node_id.to_string())
        }
    }

    pub mod graph_items {
        pub mod edge {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge(String, String);

            impl Edge {
                pub fn new(first: &str, second: &str) -> Self {
                    Edge(first.to_string(), second.to_string())
                }

                pub fn with_attrs(self, _attrs: &[(&str, &str)]) -> Self {
                    self
                }
            }
        }

        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub value: String,
                pub attrs: Vec<(String, String)>
            }

            impl Node {
                pub fn new(value: &str) -> Self {
                    Node {
                        value: value.to_string(),
                        attrs: vec!()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .to_vec()
                        .iter()
                        .map(|tuple| (tuple.0.to_string(), tuple.1.to_string()))
                        .collect::<Vec<(String, String)>>();
                    self
                }

                pub fn get_attr(&self, id: &str) -> Option<&str>{
                    self.attrs
                        .iter()
                        .find(|&tuple| tuple.0 == id.to_string())
                        .map(|value| value.1.as_str())
                }
            }
        }
    }
}