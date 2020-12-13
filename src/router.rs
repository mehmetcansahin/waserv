use crate::Handler;
use path_tree::PathTree;

pub struct Router {
    pub(crate) tree: PathTree<Handler>,
}

impl Router {
    pub fn new() -> Router {
        let tree = PathTree::<Handler>::new();
        Router { tree: tree }
    }

    pub fn add(&mut self, method: &str, path: &str, data: Handler) {
        let path_method = format!("/{}{}", method, path);
        self.tree.insert(&path_method, data);
    }

    pub fn get(&mut self, path: &str, data: Handler) {
        self.add("GET", path, data);
    }

    pub fn post(&mut self, path: &str, data: Handler) {
        self.add("POST", path, data);
    }

    pub fn put(&mut self, path: &str, data: Handler) {
        self.add("PUT", path, data);
    }

    pub fn patch(&mut self, path: &str, data: Handler) {
        self.add("PATCH", path, data);
    }

    pub fn delete(&mut self, path: &str, data: Handler) {
        self.add("DELETE", path, data);
    }

    pub fn connect(&mut self, path: &str, data: Handler) {
        self.add("CONNECT", path, data);
    }

    pub fn head(&mut self, path: &str, data: Handler) {
        self.add("HEAD", path, data);
    }

    pub fn options(&mut self, path: &str, data: Handler) {
        self.add("OPTIONS", path, data);
    }

    pub fn trace(&mut self, path: &str, data: Handler) {
        self.add("TRACE", path, data);
    }
}
