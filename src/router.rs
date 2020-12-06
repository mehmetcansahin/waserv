use crate::Params;
use path_tree::PathTree;
use web_sys::{Request, Response};

pub struct Router {
    pub(crate) tree: PathTree<Box<dyn Fn(Request, Params) -> Response>>,
}

impl Router {
    pub fn new() -> Router {
        let tree = PathTree::<Box<dyn Fn(Request, Params) -> Response>>::new();
        Router { tree: tree }
    }

    pub fn add(&mut self, path: String, data: Box<dyn Fn(Request, Params) -> Response>) {
        self.tree.insert(&path, data);
    }
}
