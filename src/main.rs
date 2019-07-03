extern crate i3ipc;
use i3ipc::reply::{Node, NodeType, WindowProperty};
use i3ipc::I3Connection;

fn print_node(node: &Node, output: &str, workspace: &str) {
    if node.nodetype == NodeType::Con {
        match node.name {
            Some(ref name) => {
                let empty = String::from("");
                let class = match &node.window_properties {
                    Some(map) => map.get(&WindowProperty::Class).unwrap_or(&empty),
                    _ => "",
                };
                println!(
                    "{}◌{}◌{}◌{}◌{}",
                    node.id, output, workspace, class, name
                )
            }
            None => (),
        }
    }
    for child in &node.nodes {
        let output = if node.nodetype == NodeType::Output {
            match node.name {
                Some(ref name) => name,
                _ => output,
            }
        } else {
            output
        };

        let workspace = if node.nodetype == NodeType::Workspace {
            match node.name {
                Some(ref name) => name,
                _ => workspace,
            }
        } else {
            workspace
        };

        print_node(&child, &output, &workspace);
    }
}

fn print_window_infos(connection: &mut I3Connection) {
    let tree = connection
        .get_tree()
        .expect("Unable to fetch tree from sway");

    print_node(&tree, "", "");
}

fn main() {
    // establish a connection to i3 over a unix socket
    let mut connection = I3Connection::connect().unwrap();
    print_window_infos(&mut connection);
}
