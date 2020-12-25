use crate::Handler;
use path_tree::PathTree;

pub struct Router {
    pub tree: PathTree<Handler>,
}

impl Router {
    pub fn new() -> Router {
        let tree = PathTree::<Handler>::new();
        Router { tree: tree }
    }

    /// Add a new route with method, path and the handler.
    pub fn add(&mut self, method: &str, path: &str, data: Handler) {
        let path_method = format!("/{}{}", method, path);
        self.tree.insert(&path_method, data);
    }

    /// Add a new route with `GET` method.
    pub fn get(&mut self, path: &str, data: Handler) {
        self.add("GET", path, data);
    }

    /// Add a new route with `POST` method.
    pub fn post(&mut self, path: &str, data: Handler) {
        self.add("POST", path, data);
    }

    /// Add a new route with `PUT` method.
    pub fn put(&mut self, path: &str, data: Handler) {
        self.add("PUT", path, data);
    }

    /// Add a new route with `PATCH` method.
    pub fn patch(&mut self, path: &str, data: Handler) {
        self.add("PATCH", path, data);
    }

    /// Add a new route with `DELETE` method.
    pub fn delete(&mut self, path: &str, data: Handler) {
        self.add("DELETE", path, data);
    }

    /// Add a new route with `CONNECT` method.
    pub fn connect(&mut self, path: &str, data: Handler) {
        self.add("CONNECT", path, data);
    }

    /// Add a new route with `HEAD` method.
    pub fn head(&mut self, path: &str, data: Handler) {
        self.add("HEAD", path, data);
    }

    /// Add a new route with `OPTIONS` method.
    pub fn options(&mut self, path: &str, data: Handler) {
        self.add("OPTIONS", path, data);
    }

    /// Add a new route with `TRACE` method.
    pub fn trace(&mut self, path: &str, data: Handler) {
        self.add("TRACE", path, data);
    }
}
