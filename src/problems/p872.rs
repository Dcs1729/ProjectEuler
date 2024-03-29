use std::cell::RefCell;
use std::f32::consts::E;
use std::rc::{Rc, Weak};
use std::fmt;
use std::iter;
use std::ops::Range;
// use either::Either::{self, Left, Right};
// use Path::NodeList as NodeList;
// use Path::Grid as Grid;

pub fn run() {
    // let x = TreeNode::new(1);
    // let y = TreeNode::new(2);

    // x.push_child(y.copy());
    // println!("{}", x.ref_count());
    // dbg!(x.copy());

    // println!("{:?}", y.has_children(1));

    // println!("adding more children");

    // let a1 = TreeNode::new(101);
    // let a2 = TreeNode::new(102);
    // let a3 = TreeNode::new(103);
    // let a4 = TreeNode::new(104);

    // y.push_child(a4.copy());
    // y.push_child(a3.copy());
    // y.push_child(a2.copy());
    // y.push_child(a1.copy());

    // let root_iter = a4.root_path();
    
    // for i in root_iter {
    //     println!("Next Root:\n{:?}\n", i);
    // }

    // println!("{:?}", a3.get_children_vals());

    // println!("{}", y.ref_count());

    // println!("\n┬───┬\n│   ├──\n    └─┬───"); // PERFECT
    
    // testing dfs
    
    // let a1 = TreeNode::new(1);
    // let a2 = TreeNode::new(2);
    // let a3 = TreeNode::new(3);
    // let a4 = TreeNode::new(4);
    // let a5 = TreeNode::new(5);
    // let a6 = TreeNode::new(6);
    // let a7 = TreeNode::new(7);
    // let a8 = TreeNode::new(8);
    // let a9 = TreeNode::new(9);
    // let a10 = TreeNode::new(10);
    // let a11 = TreeNode::new(11);
    // let a12 = TreeNode::new(12);
    // let a13 = TreeNode::new(13);

    // a8.push_child(a9);
    // a8.push_child(a10);
    // a7.push_child(a8);
    // a7.push_child(a11);
    // a3.push_child(a5);
    // a3.push_child(a6);
    // a3.push_child(a7);
    // a4.push_child(a12);
    // a4.push_child(a13);
    // a1.push_child(a2);
    // a1.push_child(a3);
    // a1.push_child(a4);


    // a1.display();

    // let b1 = TreeNode::new("one");
    // let b2 = TreeNode::new("two");
    // let b3 = TreeNode::new("three");
    // let b4 = TreeNode::new("four");
    // let b5 = TreeNode::new("five");
    // let b6 = TreeNode::new("six");
    // let b7 = TreeNode::new("seven");
    // let b8 = TreeNode::new("eight");
    // let b9 = TreeNode::new("nine");
    // let b10 = TreeNode::new("ten");
    // let b11 = TreeNode::new("eleven");
    // let b12 = TreeNode::new("twelve");
    // let b13 = TreeNode::new("thirteen");

    // b8.push_child(b9);
    // b8.push_child(b10);
    // b7.push_child(b8);
    // b7.push_child(b11);
    // b3.push_child(b5);
    // b3.push_child(b6);
    // b3.push_child(b7);
    // b1.push_child(b2);
    // b1.push_child(b3);
    // b4.push_child(b12);
    // b4.push_child(b13);
    // b1.push_child(b4);

    // b1.display();

    // for i in 1..75 {
    //     let t = t_tree(i, |x| x); // format!("{:b}", x));
    //     println!("\nn={}:", i);
    //     t.display();    
    // }

    // t.display();

    // let x: Vec<u128> = binary_digits(52).collect();
    // println!("{:?}", x);

    let y = f(u128::pow(10, 17), u128::pow(9, 17));
    println!("{:?}", y);
}


#[derive(Debug)]
struct Node<T> where
    T: fmt::Display + fmt::Debug + Eq + Clone
{
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
    
}

#[derive(PartialEq, Eq)]
struct TreeNode<T> where
    T: fmt::Display + fmt::Debug + Eq + Clone
{
    node: Rc<Node<T>>,
}

// #[derive(Debug)]
// enum Path
// {
//     NodeList,
//     Grid,
// }

impl<T> Node<T> where
    T: fmt::Display + fmt::Debug + Eq + Clone
{
    fn new(value: T) -> Self {    
        Self {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        }
    }

    fn wrap(self) -> TreeNode<T> { // i think this is useless but might as well leave it in idk
        TreeNode {node: Rc::new(self)}
    }

    fn set_parent(&self, parent: &TreeNode<T>) {
        *self.parent.borrow_mut() = Rc::downgrade(&parent.node);
    }

    fn rem_parent(&self) { // tempted to call this invoke_cps()
        *self.parent.borrow_mut() = Weak::new();
    }
}

impl<T> TreeNode<T> where
    T: fmt::Display + fmt::Debug + Eq + Clone
{
    fn new(value: T) -> Self {
        Self {
            node: Rc::new(Node::new(value)),
        }
    }

    fn copy(&self) -> Self {
        Self {
            node: Rc::clone(&self.node)
        }
    }

    fn ref_count(&self) -> usize {      // use for testing + make accompanying weak counter function?
        Rc::strong_count(&self.node)   
    } 

    fn get_val(&self) -> T {
        self.node.value.clone()
    }

    fn get_parent_val(&self) -> Option<T> {
        self.node.parent.borrow()
            .upgrade()
            .map(|parent| parent.value.clone())
    }

    fn get_parent(&self) -> Option<TreeNode<T>> {
        self.node.parent.borrow()
            .upgrade()
            .map(|parent| TreeNode{node: Rc::clone(&parent)}) // mapping an option
    }

    fn get_children_vals(&self) -> Vec<T> {
        self.node.children.borrow()
        .iter()
        .map(|child| child.value.clone())
        .collect()
    }

    fn get_children(&self) -> Vec<TreeNode<T>> {
        self.node.children.borrow()
            .iter()
            .map(|child| TreeNode{node: Rc::clone(child)}) // do i need to clone? I think so since I would get a lifetime error otherwise
            .collect()
    }                                  

    fn has_children(&self, no_of_children: usize) -> bool { // at least n children
        self.node.children.borrow().len() >= no_of_children
    }

    fn has_siblings(&self, no_of_siblings: usize) -> bool { // at least n children
        match self.get_parent() {
            None => false,
            Some(parent) => parent.has_children(no_of_siblings + 1),
        }
    }
    

    fn push_child(&self, child: TreeNode<T>) { // need i even make the joke
        match child.get_parent() {             // btw this function replaces the old parent if the child had a pre-existing parent,
            None => {},                        // and removes the child from the old parent to keep things consistent
            Some(p) => {
                p.rem_child(child.copy());
            },
        }

        child.node.set_parent(self);
        self.node.children.borrow_mut().push(child.node); // further note slash fun fact, you can push x.copy() to x to create an infinite loop
    }

    fn pop_child(&self) -> Option<TreeNode<T>> { // tempted to call this create_orphan()
        let orphan_option = self.node.children.borrow_mut().pop();
        match orphan_option {
            None => None,
            Some(orphan) => {
                orphan.rem_parent();
                Some(TreeNode{node: orphan}) // dont need to clone since we're moving the value out i believe?
            }                                // also not doing this one via map cos i will confuse myself with (de)referencing
        }
    }

    fn rem_child(&self, child: TreeNode<T>) -> Option<TreeNode<T>> { // removes a specific child and returns it if the operation was successful
        // println!("self: {:?}", self);
        let mut children = self.node.children.borrow_mut();
        
        // println!("CHILD: {:?}", child.node);
        for i in 0..children.len() {
            // println!("Child: {:?}", children[i]);
            if children[i] == child.node {
                // println!("should remove child");s
                children.remove(i);
                return Some(child)
            }
        }
        None
    }

    fn root(&self) -> TreeNode<T> {
        let mut r: Self = self.copy();
        loop {
            match r.get_parent() {
                None => break,
                Some(p) => r = p,
            }
        }
        r
    }

    fn root_path(&self) -> impl iter::Iterator<Item=TreeNode<T>> {  // return an iterator of nodes from the inputted node to the root of the tree inclusive.
        iter::successors(
            Some(self.copy()),
            |current| current.get_parent()
        )
            
        
    }

    fn _max_spacing(&self) -> usize { // impl iter::Iterator<Item=String> { // a sub-function for display - actually no LOL
        self.get_children_vals()
            .iter()
            .map(|val| val.to_string())
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap_or(String::from(""))
            .len()
    }

    fn dfs_path(&self) -> Vec<(TreeNode<T>, Vec<usize>)> { // impl iter::Iterator<Item=TreeNode<T>> { // lets do vec firs then this next
        let mut children_pos_stack: Vec<usize> = vec![];
        let mut current_node = self.copy();
        let mut dfs_vec: Vec<(TreeNode<T>, Vec<usize>)> = vec![(self.copy(), vec![])];

        'outer: loop {
            if current_node.has_children(1) {
               current_node = current_node.get_children()[0].copy();
               children_pos_stack.push(0);
               
               
            }
            else {
                'inner: loop { // goes backwards if possible, then forwards once to a sibling
                    match children_pos_stack.pop() {
                        None => break 'outer,
                        Some(index) => {
                            let parent = current_node.get_parent().unwrap();
                            if parent.get_children().len() >= index + 2 { // this should be safe to unwrap since the stack is unempty and the stack length represents depth
                                current_node = parent.get_children()[index + 1].copy();
                                children_pos_stack.push(index + 1);
                                break 'inner;
                            }
                            else {
                                current_node = parent.copy();
                            }
                        }
                    }
                }
            }

            dfs_vec.push((current_node.copy(), children_pos_stack.clone()));

            // see if we can go 1 to the left (node found)

            // if stack is empty and we cannot go to the left, break
            
            // else go back until we can turn around

            // "increment" the stack by 1 so we choose the next branch

            // go 1 to the left (node found)
        }
        dfs_vec
    }

    fn display(&self) {
        let dfs_vec = self.dfs_path();
        let mut display_cols: Vec<Vec<char>> = vec![];
        let mut col_ranges: Vec<Range<usize>> = vec![];
        

        let mut i = 0;
        while i < dfs_vec.len() {
            
            let start_i = i;
            let start_depth = dfs_vec[i].1.len();
            
            'inner: while dfs_vec[i].1.len() + start_i == start_depth + i {
                i += 1;
                if i == dfs_vec.len() {break 'inner;}
            }

            col_ranges.push(start_i..i);
        }

        let mut depth_pos_stack: Vec<usize> = vec![]; // this whole thing to get the correct space and horizontal line characters could have been done with bools instead of usizes oh well
        
        for range in col_ranges {
            // let col_chars: Vec<char> = vec![];
            
            let col_char_iter = &dfs_vec[range.clone()]  // remember to remove this clone when i have finished debugging
                .iter()
                .map (|(node, pos)| (node, pos, node.node.value.to_string()));
                

            let max_spacing = col_char_iter // so add this many horizonal lines to nodes with more siblings
                .clone()
                .map(|(_, _, chars)| chars.chars().count())
                .max()
                .unwrap_or(1);

             let new_dps: Vec<usize> = col_char_iter  // char indexes of the depth layers (depth of each node which isnt a last sibling, also excluding the root)
                .clone()
                .filter(|(node, pos, _)| pos.len() != 0 && node.has_siblings(pos.last().unwrap_or(&0) + 1)) // not root && not last child
                .map(|(_, pos, _)| pos.len())
                .collect();
            
            let start_depth = col_char_iter.clone().next().unwrap().1.len();
            // let end_depth = col_char_iter.clone()
            //     .filter(|(node, pos, _| ) // children which are not the last nodes
            //     .last().unwrap().1.len();

            depth_pos_stack.retain(|i|  *i < start_depth);
            depth_pos_stack.extend(new_dps.clone()); // remove clone after debugging
            depth_pos_stack.dedup();

            let mut spaces_vec: Vec<char> = match start_depth {
                0 => vec![],
                _ => {
                    (0..(2*start_depth-1))
                        .map(|i| if i % 2 == 1 && depth_pos_stack.contains(&((i+1)/2)) {'─'} else {' '})
                        .collect()
                },
            };

            let intermediate_vecs: Vec<Vec<char>> = (1..max_spacing + 3)
                .map(|i| {
                    let mut v = spaces_vec.clone();

                    v.extend(
                        col_char_iter
                        .clone()
                        .flat_map(|(node, pos, val)|
                            match (         // check for root and last child
                                pos.len(),
                                node.has_siblings(pos.last().unwrap_or(&0) + 1),
                                val.chars().nth(i)
                            ) {
                                (0, _, Some(c)) => vec![c],
                                (0, _, None) => vec![' '],
                                (_, true, Some(c)) => vec!['─', c],
                                (_, true, None) => vec!['─', ' '],
                                (_, false, Some(c)) => vec![' ', c],
                                (_, false, None) => vec![' ', ' '],
                            }
                        )
                    );
                    
                    v
                })
                .collect();
            

            
            // println!("{:?}", new_dps);

            let pipes_vec: Vec<char> = col_char_iter
                .clone()
                .map(|(node, pos, val)|
                    vec![
                        match (         // find correct pipe, we have up if node is first child, left if node is not first child, right if node is not last child,
                            pos.len(),                                      // we also check for the special case of the root, where we dont have a pipe char at all
                            pos.last() == Some(&0),                         // so we match for (exists, up, right) there will be exactly one of left/up
                            node.has_siblings(pos.last().unwrap_or(&0) + 1) // of course, all pipes will point downwards as they are pointing at the node itself
                        ) {
                            (0, _, _) => vec![],
                            (_, true, true) => vec!['├'],
                            (_, true, false) => vec!['│'],
                            (_, false, true) => vec!['┬'],
                            (_, false, false) => vec!['┐'],
                        },
                        match val.chars().next() {         // matching to print the actual node value display char. 
                            None => vec![' '],
                            Some(c) => vec![c],
                        }                            
                    ]
                )
                .flat_map(|nested_vec| nested_vec.into_iter().flatten())
                .collect();
            
            spaces_vec.extend(pipes_vec);
            display_cols.push(spaces_vec);
            display_cols.extend(intermediate_vecs);

            
            // println!("{}, {:?},\t{:?},\t{:?}", start_depth, depth_pos_stack, new_dps, display_cols.last().unwrap());
        }
        
        // fill in display_cols with spaces to make it a rectangular array

        let max_height = display_cols
                            .iter()
                            .max_by(|a, b| Vec::len(a).cmp(&Vec::len(b)))
                            .unwrap_or(&vec![])
                            .len();
        
        display_cols = display_cols
            .into_iter()
            .map(|mut col| {
                col.resize(max_height, ' ');
                col
            })
            .collect()
            ;
        
        // PRINTING THE TREE
        
        for i in 0..max_height {
            let row: String = (0..display_cols.len()).map(|j| display_cols[j][i]).collect(); // transpose 2d vector and print by row
            println!("{}", row);
        }

        // for col in display_cols {
        //     println!("{:?}", col);
        // }

        
        // ONLY THING LEFT TO DO IS TO TRANSPOSE THE DISPLAY VEC, AND ALSO ADD IN EXTRA SPACES, WILL NEED TO RECALC THE SPACES_VEC TO ACCOUNT FOR A POSSIBLE EXTRA HORIZONTAL LINE

        

        // for i in col_ranges {
        //     println!("{:?}", i);
        // }


        // loop {
        //     // if layer.len() == 0 {break;}
        //     // // else:

        //     // display_rows.push([
        //     //     vec!['├'],
        //     //     iter::repeat('┬').take(layer.len() - 1).collect()
        //     //     ].concat()
        //     // );





        // }
        
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

//  playground: ┬───┬  
//              │ ┬ ├─ 
//                  └─┬─── 2 ────

// chars:   ┬   U+252C
//          ─   U+2500
//          │   U+2502
//          ├   U+251C 	
//          ┐   U+2510

// this is gonna be complicated to implement... lets do it
// ok its doable... also lets make it a void function which prints ACTUALLY NO cos i cant do that recursively with 1 print statement... hmm... helper function? DFS grid!

impl<T> fmt::Debug for TreeNode<T> where
    T: fmt::Display + fmt::Debug + Eq + Clone
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let node_children = self.get_children_vals();
        // .iter()
        // .map(|val| val.to_string())
        // .collect::<Vec<String>>()
        // .join(", ");

        write!(f, "TreeNode: {{\n\tvalue: {},\n\tparent: {:?},\n\tchildren ({}): {:?}\n}}", self.get_val(), self.get_parent_val(), node_children.len(), node_children)
    }
}

impl<T> PartialEq for Node<T>
where
    T: fmt::Display + fmt::Debug + Eq + Clone
{
    fn eq(&self, other: &Self) -> bool {
        // Check if the values are equal
        self.value == other.value &&
        // dont compare parent

        // Compare children recursively
        self.children.borrow().len() == other.children.borrow().len() &&

        self.children.borrow().iter().zip(other.children.borrow().iter()).all(|(a, b)| a == b)
    }
}

impl<T> Eq for Node<T>
where
    T: fmt::Display + fmt::Debug + Eq + Clone
{
}

fn t_tree<T>(n: usize, f: fn(usize) -> T) -> TreeNode<T> where
    T: fmt::Display + fmt::Debug + Eq + Clone
{ // could implement this as t_next iteratively, or with a trait? idk
    match n {
        0 => panic!("0 is invalid input!"),
        1 => TreeNode::new(f(n)),
        _ => {
            let new_root = TreeNode::new(f(n));
            let mut node = t_tree(n-1, f);
            let mut path: Vec<TreeNode<T>> = vec![];
            loop {
                path.push(node.copy());

                // let next = node
                //     .get_children()
                //     .into_iter()
                //     .max_by(|a, b| a.get_val().cmp(&b.get_val()));

                // match next {
                //     None => break,
                //     Some(child) => {node = child.copy()},
                // }
                
                let children = node.get_children();
                match children.len() {
                    0 => break,
                    _ => {node = children[0].copy()}
                }
            }

            // let path = node
            //     .root_path()
            //     .collect::<Vec<TreeNode<usize>>>();

            // println!("{:?}", path);
            
            path
                .into_iter()
                .for_each(|n| new_root.push_child(n));


            new_root
        }
    }
}

fn f(n: u128, k: u128) -> u128 {
    // algorithm:

    // to find the path from n to k, we first take their difference, and then express that number in binary.
    // then from right to left, we take the ith child (counting from 0) where i is the number of intermediate 0s between the next 1
    // e.g. for f(71, 19), n-k = 52, which is 110100 in binary. So we firstly take the 2nd child (100), then the 1st child (10) then the 0th child (1) and we go from 71 to 19

    // furthermore, we can see that if the previous difference between the parent and current node was 2^k (it is always a power of 2),
    // then each child of the current node have a difference of 2^(k+1), 2^(k+2) etc. while the child value remains positive

    // from this information, we can calculate the values of all the nodes in our path from n to k

    let mut pow = 1;
    let mut total = n;
    let mut current_val = n;
    for digit in binary_digits(n-k) {
        match digit {
            0 => {},
            _ => // 1 (hopefully)
                {
                    current_val -= pow;
                    total += current_val;
                },
        }
        pow *= 2;
    }

    total
}

fn binary_digits(mut n: u128) -> impl Iterator<Item = u128> { // "reverse" order (as wanted)
    iter::successors(Some(n % 2), move |&_x| {
        n /= 2;
        if n == 0 {
            None
        } else {
            Some(n % 2)
        }
    })
}
