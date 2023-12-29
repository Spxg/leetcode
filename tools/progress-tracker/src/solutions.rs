use git2::{Tree, TreeWalkMode, TreeWalkResult};

pub struct Solution {
    pub problem_id: String,
    pub id: String,
    pub root: String,
    pub file: String,
}

pub fn list(tree: &Tree, mut f: impl FnMut(Solution)) {
    tree.walk(TreeWalkMode::PreOrder, |root, entry| {
        let name = entry.name().unwrap();

        if let Some(problem_id) = root
            .strip_prefix("src/problem_")
            .and_then(|s| s.strip_suffix('/'))
        {
            if let Some(solution_id) = name.strip_suffix(".rs") {
                if name != "mod.rs" {
                    f(Solution {
                        problem_id: problem_id.replace('_', "-"),
                        id: solution_id.to_string(),
                        root: root.to_string(),
                        file: name.to_string(),
                    });
                }
            }
        }

        TreeWalkResult::Ok
    })
    .unwrap();
}
