use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

type MazeContainer = Arc<Mutex<Maze>>;

#[derive(PartialEq)]
enum Exploration {
    Explored,
    PartialExplored,
    UnExplored,
}

struct Branch {
    label: String,
    left: MazeContainer,
    right: MazeContainer,
    status: Exploration,
}

impl Branch {
    fn from(label: String, left: &MazeContainer, right: &MazeContainer) -> MazeContainer {
        Arc::new(Mutex::new(Maze::Branch(Branch {
            label,
            left: Arc::clone(left),
            right: Arc::clone(right),
            status: Exploration::UnExplored,
        })))
    }
}

struct Leaf(String);

impl Leaf {
    fn from(label: String) -> MazeContainer {
        Arc::new(Mutex::new(Maze::Leaf(label)))
    }
}

enum Maze {
    Branch(Branch),
    Leaf(String),
}

impl Maze {
    fn explore(
        &mut self,
        node: MazeContainer,
        list: &mut Vec<String>,
        work: &mut Arc<Mutex<Vec<MazeContainer>>>,
    ) {
        match self {
            Maze::Branch(b) => {
                if b.status == Exploration::UnExplored {
                    b.status = Exploration::PartialExplored;
                    work.lock().unwrap().push(node);
                    list.push(b.label.clone());
                    b.left
                        .lock()
                        .unwrap()
                        .explore(Arc::clone(&b.left), list, work);
                } else if b.status == Exploration::PartialExplored {
                    b.status = Exploration::Explored;
                    b.right
                        .lock()
                        .unwrap()
                        .explore(Arc::clone(&b.right), list, work);
                }
            }
            Maze::Leaf(l) => {
                list.push(l.clone());
            }
        }
    }
}

pub fn maze_solver_tp4() {
    let leaf2 = Leaf::from(String::from("2"));
    let leaf4 = Leaf::from(String::from("4"));
    let leaf5 = Leaf::from(String::from("5"));
    let leaf8 = Leaf::from(String::from("8"));
    let branch3 = Branch::from(String::from("3"), &leaf4, &leaf5);
    let branch1 = Branch::from(String::from("1"), &leaf2, &branch3);
    let branch7 = Branch::from(String::from("7"), &leaf5, &leaf8);
    let branch6 = Branch::from(String::from("6"), &branch3, &branch7);
    let branch0 = Branch::from(String::from("0"), &branch1, &branch6);

    let work = Arc::new(Mutex::new(vec![Arc::clone(&branch0)]));
    let mut handles = vec![];
    while let Some(node) = work.lock().unwrap().pop() {
        let a_work = Arc::clone(&work);
        let handle = thread::spawn(move || {
            let mut trace: Vec<String> = vec![];
            node.lock()
                .unwrap()
                .explore(Arc::clone(&node), &mut trace, &mut Arc::clone(&a_work));

            println!("{:?}", trace);
        });
        thread::sleep(Duration::new(1, 0));
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
