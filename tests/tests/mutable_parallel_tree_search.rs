// use std::sync::{Arc, Mutex, MutexGuard};
// use std::thread;
// use std::thread::JoinHandle;
//
// type SafeNode = Arc<Mutex<TreeNode>>;
//
// #[derive(Debug)]
// struct TreeNode {
//     value: u16,
//     children: Vec<TreeNode>,
// }
//
// fn main() {
//     let mut root = TreeNode {
//         value: 1,
//         children: vec![TreeNode {
//             value: 2,
//             children: vec![],
//         }],
//     };
//     for _i in 1..5 {
//         // for 5 turns, deepen search tree and pick best move
//         root = find_best_child(root);
//         println!("{:#?}", root);
//     }
// }
//
// fn find_best_child(root: TreeNode) -> TreeNode {
//     let new_children = spawn_mutate_threads(root.children);
//     new_children
//         .into_iter()
//         .map(|c| {
//             Arc::try_unwrap(c)
//                 .expect("unwraping arc")
//                 .into_inner()
//                 .expect("unwrapping mutex")
//         })
//         .max_by_key(|c| {
//             println!("c {:?}", c);
//             c.value
//         })
//         .expect("max by") // now that we've deepend the tree and propogated the node values up, pick the best child node (move) and return it as the new position
// }
//
// fn spawn_mutate_threads(children: Vec<TreeNode>) -> Vec<SafeNode> {
//     children
//         .into_iter()
//         .map(|child| thread(Arc::new(Mutex::new(child)))) // for each possible move, spawn a thread that deepens the search tree
//         .map(|th| th.join().expect("panicked joining threads"))
//         .collect()
// }
//
// fn thread(child: SafeNode) -> JoinHandle<SafeNode> {
//     thread::spawn(move || {
//         let child_guard = child.lock().unwrap();
//         mutate(child_guard);
//         child.clone()
//     })
// }
//
// fn mutate(mut child: MutexGuard<TreeNode>) {
//     for _i in 1..5 {
//         child.children.push(TreeNode {
//             //simulate deeping search tree by adding new node
//             value: rand::random(),
//             children: vec![TreeNode {
//                 value: rand::random(),
//                 children: vec![],
//             }],
//         });
//     }
//     child.value = rand::random(); // value updated to reflect what we learned about the value of this move by evaluating new children
// }
