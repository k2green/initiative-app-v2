use yew::prelude::*;

#[hook]
pub fn use_multi_node_refs(count: usize) -> Vec<NodeRef> {
    let refs = (0..count).map(|_| NodeRef::default()).collect::<Vec<_>>();
    let state = use_state(|| refs);

    (*state).clone()
}