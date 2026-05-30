use crate::*;

#[test]
fn test_virtual_node_default() {
    let node: VirtualNode = VirtualNode::default();
    match node {
        VirtualNode::Empty => {}
        _ => panic!("expected empty node"),
    }
}
