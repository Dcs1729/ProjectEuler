// DO NOT RUN THIS PIECE OF SHIT CODE IT HAS MEMORY LEAKS

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use std::iter;

pub fn run() {
    let mut x = WrappedNode::new(10);
    println!("{:?} ", x);

    let mut y = WrappedNode::new(9);

    let z = WrappedNode::new(8);

    println!("{}------", Rc::strong_count(&x.node));

    y.push_child(z);

    println!("successfully pushed z");
    println!("{:?} ", y);

    x.push_child(y);

    println!("successfully pushed y");
    println!("{:?} ", x);
    println!("{}------", x.ref_count());
    {
        let y_copy = x.node.borrow().children[0].get_copy();
        println!("new y: {:?} {}", y_copy, Rc::strong_count(&y_copy.node));
        println!("{}------", x.ref_count());

        let z_copy = y_copy.node.borrow().children[0].get_copy();

        let x_copy = y_copy.get_parent().unwrap();
        println!("new x: {:?} ", x_copy);
        println!("{}, {}, {}", x_copy.ref_count(), y_copy.ref_count(), z_copy.ref_count());

        let r = z_copy.root();
        println!("root: {:?} ", r);
        println!("{}, {}, {}", x_copy.ref_count(), y_copy.ref_count(), z_copy.ref_count());

        let root_iter = x_copy.root_path();
        println!("ROOT PATH:::");
        for node in root_iter {
            println!("node: {:?}", node);
        }

        println!("{}, {}, {}", x_copy.ref_count(), y_copy.ref_count(), z_copy.ref_count());
    }

    println!("{}------", Rc::strong_count(&x.node));


    

    let ch = x.pop_child();

    println!("popped 1st child successfully: {:?} ", ch);
    println!("x: {:?} ", x);

    println!("{}------", Rc::strong_count(&x.node));

    let ch2 = x.pop_child();
    println!("popped 2nd child successfully: {:?} ", ch2);
    println!("x: {:?} ", x);

    println!("{}------", Rc::strong_count(&x.node));



    println!("\n\n\nNew Stuff:\n"); // NEW CHAPTER OF RANDOM PRINTS WOOOO

    let mut new_tree = WrappedNode::new(727);
    for i in 1..=5 {
        new_tree.push_child(WrappedNode::new(i));
    }

    println!("{}", new_tree.ref_count());

    println!("{:?}", new_tree);
    let mut kids = new_tree.get_children();
    println!("{:#?}", kids);
    
    for kid in &kids {
        if kid.get_val() == 4 {
            let mut x = kid.get_copy();
            x.push_child(WrappedNode::new(100));
        }
    }

    println!("{:#?}", kids);
    println!("{}", kids[3].ref_count());

    println!("{:?} {}", new_tree.node.borrow().children[3].node.borrow().get_children_vals(), new_tree.node.borrow().children[3].ref_count());
    kids[3].push_child(WrappedNode::new(200));
    println!("{}", kids[3].ref_count());
}

type WrappedNodeType = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    value: usize,
    parent: Option<WrappedNode>,    
    children: Vec<WrappedNode>,
    
}

// #[derive(Debug)]
struct WrappedNode {
    node: WrappedNodeType,
}

impl TreeNode {
    fn new(value: usize) -> Self {    
        Self {
            value,
            parent: None,
            children: Vec::new(),
        }
    }

    fn set_parent(&mut self, parent: WrappedNodeType) {
        self.parent = Some(
            WrappedNode {
                node: parent,
            }
        );
    }

    fn rem_parent(&mut self) { // tempted to call this invoke_cps()
        self.parent = None;
    }

    fn get_parent_val(&self) -> Option<usize> {
        match &self.parent {
            None => None,
            Some(parent) => Some(parent.node.borrow().value),
        }
    }

    fn get_children_vals(&self) -> Vec<usize> {
        self.children
        .iter()
        .map(|child| child.node.borrow().value)
        .collect()
    }
}

impl WrappedNode { 
    fn new(value: usize) -> Self {
        Self {
            node: Rc::new(RefCell::new(TreeNode::new(value))),
        }
    }

    fn get_val(&self) -> usize {
        self.node.borrow().value
    }

    fn ref_count(&self) -> usize {      // ok so im kinda a genius for my memory efficiency here. this took me way too long to reason with myself, but ref_count will be equal to:
        Rc::strong_count(&self.node)    // 1 for parent (unless node is the root) + 1 for each child + 1 for each variable which owns a direct WrappedNode type of the node
    }                                   // Note: Copying the parent/children will NOT increase the ref_count for the middle node itself, only direct copies will - this is awesome!

    fn get_copy(&self) -> WrappedNode {
        WrappedNode {
            node: Rc::clone(&self.node),
        }
    }

    fn has_child(&self) -> bool {
        self.node.borrow().children.len() >= 1
    }

    fn push_child(&mut self, child: WrappedNode) { // need i even make the joke
        (*child.node.borrow_mut()).set_parent(Rc::clone(&self.node));
        (*self.node.borrow_mut()).children.push(child);
    }

    fn pop_child(&mut self) -> Option<WrappedNode> { // tempted to call this create_orphan()
        let orphan_option = (*self.node.borrow_mut()).children.pop();
        match orphan_option {
            None => None,
            Some(orphan) => {
                orphan.node.borrow_mut().rem_parent();
                Some(orphan)
            }
        }
    }

    fn get_children(&self) -> Vec<WrappedNode> {
        self.node.borrow().children
            .iter()
            .map(|child| child.get_copy())
            .collect()
    }

    fn get_parent(&self) -> Option<WrappedNode> {
        match &self.node.borrow().parent {
            None => None,
            Some(p) => {
                Some(WrappedNode {
                    node: Rc::clone(&p.node),
                })
            },
        }
    }

    fn root(&self) -> WrappedNode {
        let mut r: Self = self.get_copy();
        loop {
            match r.get_parent() {
                None => break,
                Some(p) => r = p,
            }
        }
        r
    }

    fn root_path(&self) -> impl iter::Iterator<Item=WrappedNode> {  // return an iterator of nodes from the inputted node to the root of the tree inclusive.
        iter::successors(
            Some(self.get_copy()),
            |current| current.get_parent()
        )
            
        
    }

//     fn display(&self) { // note this is recursive so might result in a stack overflow error at runtime (this also only displays the node downwards, so really displays a subtree)
//         let node_ref = self.node.borrow();
//         let node_children: Vec<usize> = node_ref.get_children_vals()
//             .iter()
//             .map(|val| (*child).get_val())
//             .collect();
//             .join(", ")

//         format!("WrappedNode: {{ value: {}, parent: {:?}, children ({}): {:?} }}", node_ref.value, node_ref.get_parent_val(), node_children.len(), node_children)
//     }
}

// this is gonna be complicated to implement... lets do it
// ok its doable... also lets make it a void function which prints ACTUALLY NO cos i cant do that recursively with 1 print statement... hmm...

impl fmt::Debug for WrappedNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node_ref = self.node.borrow();
        let node_children: Vec<usize> = node_ref.children
            .iter()
            .map(|child| (*child).get_val())
            .collect();

        write!(f, "WrappedNode: {{ value: {}, parent: {:?}, children ({}): {:?} }}", node_ref.value, node_ref.get_parent_val(), node_children.len(), node_children)
    }
}

// fn t_tree(n: usize) -> WrappedNode { // could implement this as t_next iteratively, or with a trait? idk
    
// }