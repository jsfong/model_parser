use std::collections::HashMap;

use crate::model::cubs_model::{Element, Relationship};

// Graph hold all the connection

#[derive(Clone, Debug)]
pub struct ElementConnectorGraph {
    // Each element contain one connector
    connectors: HashMap<String, ElementConnector>,
    connected_relationship: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ElementConnector {
    element_id: String,
    in_path: Vec<String>,
    out_path: Vec<String>,
}


//Implementation
impl ElementConnectorGraph {
    pub fn new() -> Self {
        Self {
            connectors: HashMap::new(),
            connected_relationship: Vec::new(),
        }
    }

    // Add connector without connection
    pub fn add_connector(&mut self, id: &str) {
        if !self.connectors.contains_key(id) {
            self.connectors.insert(
                id.to_owned(),
                ElementConnector {
                    element_id: id.to_owned(),
                    in_path: Vec::new(),
                    out_path: Vec::new(),
                },
            );
        }
    }

    // Connect element
    pub fn connect(&mut self, relationship_id: &str, from_id: &str, to_id: &str) {
        let mut connected_in = false;
        let mut connected_out = false;

        // From Obj --> add output
        if let Some(from_obj) = self.connectors.get_mut(from_id) {
            from_obj
                .out_path
                .push(format!("{}:{}", relationship_id, to_id));
            connected_in = true;
        }

        // To Obj --> add input
        if let Some(to_obj) = self.connectors.get_mut(to_id) {
            to_obj
                .in_path
                .push(format!("{}:{}", relationship_id, from_id));
            connected_out = true;
        }

        if connected_in && connected_out {
            self.connected_relationship.push(relationship_id.to_owned());
        }
    }

    pub fn get_connection(&self, id: &str) -> Option<&ElementConnector> {
        self.connectors.get(id)
    }

    pub fn get_connection_count(&self) -> usize {
        self.connectors.len()
    }

    pub fn get_connected_relationship_count(&self) -> usize {
        self.connected_relationship.len()
    }
}

// Trait
impl<'a> std::fmt::Display for ElementConnector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = &self.element_id;
        let default: &str = "";

        //Parent
        for path in &self.in_path {
            let parts: Vec<&str> = path.split(":").collect();
            let id = parts.get(1).unwrap_or(&default);
            let rel_id = parts.get(0).unwrap_or(&default);
            writeln!(f, "<{}> -- ({}) --> ", id, rel_id)?;
        }

        //Current
        writeln!(f, "              [{}] ", id)?;

        // Child
        for path in &self.out_path {
            let parts: Vec<&str> = path.split(":").collect();
            let id = parts.get(1).unwrap_or(&default);
            let rel_id = parts.get(0).unwrap_or(&default);
            writeln!(f, "                 -- ({}) --> <{}>", rel_id, id)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::element_connector::{ElementConnector, ElementConnectorGraph};

    #[test]
    fn test_new() {
        // Build
        let mut graph = ElementConnectorGraph::new();
        graph.add_connector("c1");
        graph.add_connector("c2");
        graph.add_connector("c3");
        graph.add_connector("c4");
        graph.add_connector("c5");
        graph.connect("r1", "c1", "c3");
        graph.connect("r2", "c2", "c3");
        graph.connect("r3", "c3", "c4");
        graph.connect("r4", "c3", "c5");

        // Print
        let c1 = graph.get_connection("c1");
        let c2 = graph.get_connection("c2");
        let c3 = graph.get_connection("c3");
        let c4 = graph.get_connection("c4");
        let c5 = graph.get_connection("c5");

        if let Some(c) = c1 {
            println!("--- Print C1 ----");
            println!("{}", c);
        }
        println!();

        if let Some(c) = c2 {
            println!("--- Print C2 ----");
            println!("{}", c);
        }

        println!();

        if let Some(c) = c3 {
            println!("--- Print C3 ----");
            println!("{}", c);
        }

        println!();

        if let Some(c) = c4 {
            println!("--- Print C4 ----");
            println!("{}", c);
        }

        println!();

        if let Some(c) = c5 {
            println!("--- Print C5 ----");
            println!("{}", c);
        }

        assert!(true);
    }
}
