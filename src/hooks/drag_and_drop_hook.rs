use std::rc::Rc;

use web_sys::Element;
use yew::prelude::*;

#[derive(Debug, Clone)]
enum DragAndDropAction {
    OnDragStart(usize, DragEvent),
    OnDragOver(usize, DragEvent),
    OnDragEnter(usize, DragEvent),
    OnDragLeave(usize, DragEvent),
    OnDragEnd(usize, DragEvent),
    OnDrop(usize, DragEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropArguments {
    pub dragging_index: usize,
    pub target_index: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct DragAndDropReducer {
    parent_ref: Option<NodeRef>,
    dragging_index: Option<usize>,
    target_index: Option<usize>,
    mouse_position: Option<(f64, f64)>,
    on_drop_callback: Callback<DropArguments>
}

impl Reducible for DragAndDropReducer {
    type Action = DragAndDropAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            DragAndDropAction::OnDragStart(idx, _) => self.with_dragging_index(Some(idx)),
            DragAndDropAction::OnDragOver(_, e) => {
                e.prevent_default();

                if let Some(parent_ref) = &self.parent_ref {
                    if let Some(element) = parent_ref.cast::<Element>() {
                        let rect = element.get_bounding_client_rect();
                        let x = (e.client_x() as f64) - rect.left();
                        let y = (e.client_y() as f64) - rect.top();

                        return self.with_mouse_position(Some((x, y)));
                    }
                }

                self
            },
            DragAndDropAction::OnDragEnter(idx, _) => self.with_target_index(Some(idx)),
            DragAndDropAction::OnDragLeave(_, _) => self.with_mouse_position(None),
            DragAndDropAction::OnDragEnd(_, _) => self.cleared(),
            DragAndDropAction::OnDrop(_, _) => {
                let dragging_index = match self.dragging_index {
                    Some(idx) => idx,
                    None => return self
                };

                let target_index = match self.target_index {
                    Some(idx) => idx,
                    None => return self
                };

                self.on_drop_callback.emit(DropArguments { dragging_index, target_index });

                self.cleared()
            }
        }
    }
}

impl DragAndDropReducer {
    fn new(on_drop_callback: Callback<DropArguments>) -> Self {
        Self {
            parent_ref: None,
            dragging_index: None,
            target_index: None,
            mouse_position: None,
            on_drop_callback
        }
    }

    fn new_with_mouse_position(node_ref: NodeRef, on_drop_callback: Callback<DropArguments>) -> Self {
        Self {
            parent_ref: Some(node_ref),
            dragging_index: None,
            target_index: None,
            mouse_position: None,
            on_drop_callback
        }
    }

    fn cleared(self: Rc<Self>) -> Rc<Self> {
        Rc::new(Self {
            parent_ref: self.parent_ref.clone(),
            dragging_index: None,
            target_index: None,
            mouse_position: None,
            on_drop_callback: self.on_drop_callback.clone()
        })
    }

    fn with_dragging_index(self: Rc<Self>, index: Option<usize>) -> Rc<Self> {
        Rc::new(Self {
            parent_ref: self.parent_ref.clone(),
            dragging_index: index,
            target_index: self.target_index,
            mouse_position: self.mouse_position,
            on_drop_callback: self.on_drop_callback.clone()
        })
    }

    fn with_target_index(self: Rc<Self>, index: Option<usize>) -> Rc<Self> {
        Rc::new(Self {
            parent_ref: self.parent_ref.clone(),
            dragging_index: self.dragging_index,
            target_index: index,
            mouse_position: self.mouse_position,
            on_drop_callback: self.on_drop_callback.clone()
        })
    }

    fn with_mouse_position(self: Rc<Self>, pos: Option<(f64, f64)>) -> Rc<Self> {
        Rc::new(Self {
            parent_ref: self.parent_ref.clone(),
            dragging_index: self.dragging_index,
            target_index: self.target_index,
            mouse_position: pos,
            on_drop_callback: self.on_drop_callback.clone()
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseDragAndDropHandle {
    reducer: UseReducerHandle<DragAndDropReducer>
}

impl UseDragAndDropHandle {
    pub fn dragging_index(&self) -> Option<usize> {
        self.reducer.dragging_index
    }

    pub fn is_dragging_index(&self, idx: usize) -> bool {
        match self.reducer.dragging_index {
            Some(dragging_index) => dragging_index == idx,
            None => false
        }
    }

    pub fn target_index(&self) -> Option<usize> {
        self.reducer.target_index
    }

    pub fn is_target_index(&self, idx: usize) -> bool {
        match self.reducer.dragging_index {
            Some(dragging_index) => dragging_index == idx,
            None => false
        }
    }

    pub fn mouse_position(&self) -> Option<(f64, f64)> {
        self.reducer.mouse_position
    }

    pub fn on_drag_start(&self, idx: usize) -> Callback<DragEvent> {
        let reducer = self.reducer.clone();
        Callback::from(move |e| {
            reducer.dispatch(DragAndDropAction::OnDragStart(idx, e));
        })
    }

    pub fn on_drag_over(&self, idx: usize) -> Callback<DragEvent> {
        let reducer = self.reducer.clone();
        Callback::from(move |e| {
            reducer.dispatch(DragAndDropAction::OnDragOver(idx, e));
        })
    }

    pub fn on_drag_enter(&self, idx: usize) -> Callback<DragEvent> {
        let reducer = self.reducer.clone();
        Callback::from(move |e| {
            reducer.dispatch(DragAndDropAction::OnDragEnter(idx, e));
        })
    }

    pub fn on_drag_leave(&self, idx: usize) -> Callback<DragEvent> {
        let reducer = self.reducer.clone();
        Callback::from(move |e| {
            reducer.dispatch(DragAndDropAction::OnDragLeave(idx, e));
        })
    }

    pub fn on_drag_end(&self, idx: usize) -> Callback<DragEvent> {
        let reducer = self.reducer.clone();
        Callback::from(move |e| {
            reducer.dispatch(DragAndDropAction::OnDragEnd(idx, e));
        })
    }

    pub fn on_drop(&self, idx: usize) -> Callback<DragEvent> {
        let reducer = self.reducer.clone();
        Callback::from(move |e| {
            reducer.dispatch(DragAndDropAction::OnDrop(idx, e));
        })
    }
}

#[hook]
pub fn use_drag_and_drop(callback: impl Into<Callback<DropArguments>>) -> UseDragAndDropHandle {
    let callback = callback.into();
    let reducer = use_reducer(move || DragAndDropReducer::new(callback));
    UseDragAndDropHandle { reducer }
}

#[hook]
pub fn use_drag_and_drop_with_mouse(node: NodeRef, callback: impl Into<Callback<DropArguments>>) -> UseDragAndDropHandle {
    let callback = callback.into();
    let reducer = use_reducer(move || DragAndDropReducer::new_with_mouse_position(node, callback));
    UseDragAndDropHandle { reducer }
}