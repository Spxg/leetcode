use std::collections::{HashMap, HashSet};

struct ThroneInheritance {
    king: String,
    properties: HashMap<String, Vec<String>>,
    death: HashSet<String>,
}

impl ThroneInheritance {
    #[allow(non_snake_case)]
    fn new(kingName: String) -> Self {
        let mut properties = HashMap::new();
        properties.insert(kingName.clone(), vec![]);
        Self {
            king: kingName,
            properties,
            death: HashSet::new(),
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        self.properties
            .entry(parent_name.clone())
            .and_modify(|x| x.push(child_name.clone()));

        self.properties.insert(child_name, vec![]);
    }

    fn death(&mut self, name: String) {
        self.death.insert(name);
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        let mut result = Vec::with_capacity(self.properties.len() - self.death.len());
        if !self.death.contains(&self.king) {
            result.push(self.king.clone());
        }
        self.helper(&self.king, &mut result);
        result
    }

    fn helper(&self, name: &str, result: &mut Vec<String>) {
        for child in &self.properties[name] {
            if !self.death.contains(child) {
                result.push(child.clone());
            }
            self.helper(child, result);
        }
    }
}

impl super::ThroneInheritance for ThroneInheritance {
    fn new(king_name: String) -> Self {
        Self::new(king_name)
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        self.birth(parent_name, child_name);
    }

    fn death(&mut self, name: String) {
        self.death(name);
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        self.get_inheritance_order()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::ThroneInheritance>();
    }
}
