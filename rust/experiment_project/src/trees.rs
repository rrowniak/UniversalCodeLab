use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type BtNodeRef = Option<Rc<RefCell<BtNode>>>;

#[derive(Debug, Clone)]
struct BtNode {
    val: i32,
    l: BtNodeRef,
    r: BtNodeRef,
}

impl BtNode {
    fn from(val: i32) -> Self {
        Self {
            val,
            l: None,
            r: None,
        }
    }

    fn insert(&mut self, val: i32) {
        let n = if val > self.val {
            &mut self.r
        } else {
            &mut self.l
        };
        match n {
            Some(n) => n.borrow_mut().insert(val),
            None => *n = Some(Rc::new(RefCell::new(BtNode::from(val)))),
        }
    }

    #[allow(dead_code)]
    fn depth_first_search(&self) -> Vec<i32> {
        let mut v = vec![self.val];
        if let Some(n) = &self.l {
            v.append(n.borrow().depth_first_search().as_mut());
        }
        if let Some(n) = &self.r {
            v.append(n.borrow().depth_first_search().as_mut());
        }
        v
    }

    #[allow(dead_code)]
    fn breadth_first_search(&self) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        let mut q: VecDeque<BtNode> = VecDeque::new();
        q.push_back(self.clone());
        while let Some(n) = q.pop_front() {
            if let Some(c) = n.l {
                q.push_back(c.borrow().clone());
            }
            if let Some(c) = n.r {
                q.push_back(c.borrow().clone());
            }
            v.push(n.val);
        }
        v
    }
}

// create the following tree
//           5
//       4       7
//     2       6   8
//   1
fn example_tree() -> BtNode {
    let mut root = BtNode {
        val: 5,
        l: Default::default(),
        r: Default::default(),
    };
    root.insert(4);
    root.insert(2);
    root.insert(1);

    root.insert(7);
    root.insert(6);
    root.insert(8);
    root
}

pub fn main() {
    let root = example_tree();
    println!("root node: {:?}", root);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_depth_first_search() {
        let root = example_tree();
        let res = root.depth_first_search();
        assert_eq!(res, vec![5, 4, 2, 1, 7, 6, 8]);
    }

    #[test]
    fn check_breadth_depth_search() {
        let root = example_tree();
        let res = root.breadth_first_search();
        assert_eq!(res, vec![5, 4, 7, 2, 6, 8, 1]);
    }
}
