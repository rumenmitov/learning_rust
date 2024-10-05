use linked_list::linked_list;

fn main() {

    let mut node = linked_list::Node::new();
    node.append(1);
    node.append(2);
    node.append(4);

    node.insert(3, 3);

    let nit = node.into_iter();
    nit.for_each(|node| {
        print!("{} ", node.borrow());
    });

    print!("\n");
}
