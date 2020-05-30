use native::pool_tree::PoolTree;

fn main() {
    let mut pool = PoolTree::new(0);
    let root = pool.get_root_mut();
}