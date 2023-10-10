pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut result = String::from("/");
        let mut subpaths = vec![];
        for subpath in path.split('/').filter(|x| !x.is_empty()) {
            if subpath.eq("..") {
                subpaths.pop();
            } else if subpath.ne(".") {
                subpaths.push(subpath);
            }
        }
        result.push_str(&subpaths.join("/"));
        result
    }
}

impl super::Solution for Solution {
    fn simplify_path(path: String) -> String {
        Self::simplify_path(path)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
