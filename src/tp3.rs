use std::cell::RefCell;
use std::rc::Rc;

type MazeContainer = Rc<RefCell<Maze>>;

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
        Rc::new(RefCell::new(Maze::Branch(Branch {
            label,
            left: Rc::clone(left),
            right: Rc::clone(right),
            status: Exploration::UnExplored,
        })))
    }
}

struct Leaf(String);

impl Leaf {
    fn from(label: String) -> MazeContainer {
        Rc::new(RefCell::new(Maze::Leaf(label)))
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
        work: &mut Vec<MazeContainer>,
    ) {
        match self {
            Maze::Branch(b) => {
                if b.status == Exploration::UnExplored {
                    b.status = Exploration::PartialExplored;
                    work.push(node);
                    list.push(b.label.clone());
                    b.left.borrow_mut().explore(Rc::clone(&b.left), list, work);
                } else if b.status == Exploration::PartialExplored {
                    b.status = Exploration::Explored;
                    b.right
                        .borrow_mut()
                        .explore(Rc::clone(&b.right), list, work);
                }
            }
            Maze::Leaf(l) => {
                list.push(l.clone());
            }
        }
    }
}

pub fn maze_solver_tp3() {
    let leaf2 = Leaf::from(String::from("2"));
    let leaf4 = Leaf::from(String::from("4"));
    let leaf5 = Leaf::from(String::from("5"));
    let leaf8 = Leaf::from(String::from("8"));
    let branch3 = Branch::from(String::from("3"), &leaf4, &leaf5);
    let branch1 = Branch::from(String::from("1"), &leaf2, &branch3);
    let branch7 = Branch::from(String::from("7"), &leaf5, &leaf8);
    let branch6 = Branch::from(String::from("6"), &branch3, &branch7);
    let branch0 = Branch::from(String::from("0"), &branch1, &branch6);

    let mut work = vec![Rc::clone(&branch0)];
    let mut trace = vec![];
    while let Some(node) = work.pop() {
        node.borrow_mut()
            .explore(Rc::clone(&node), &mut trace, &mut work);
        println!("{:?}", trace);
    }
}
