use taffy::prelude::*;

pub fn create_taffy_layout() -> (TaffyTree, NodeId, NodeId) {
    let mut taffy = TaffyTree::new();

    // Create a root node that fills the entire window
    let root_style = Style {
        size: Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        justify_content: Some(JustifyContent::Center), // Center children horizontally
        align_items: Some(AlignItems::Center),     // Center children vertically
        ..Default::default()
    };

    // Create a child node with a fixed size
    let child_style = Style {
        size: Size {
            width: Dimension::percent(1.0),
            height: Dimension::percent(1.0),
        },
        ..Default::default()
    };

    let child = taffy.new_leaf(child_style).unwrap();
    let child2 = child.clone();
    let child3 = child.clone();

    let root = taffy.new_with_children(root_style, &[child, child2, child3]).unwrap();

    // Compute the layout
    taffy
        .compute_layout(
            root,
            Size { width: AvailableSpace::Definite(900.0), height: AvailableSpace::Definite(600.0) },
        )
        .unwrap();

    (taffy, root, child)
}
