use crate::pool_tree::PoolTree;
use crate::component::NewComponent;

struct Widget{
    constructor: Box<dyn FnMut()>,
    component_id: u64,
}

struct Components{
    components: PoolTree<NewComponent>,
    widgets: PoolTree<Widget>
}