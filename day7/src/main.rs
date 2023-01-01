use std::{env,fs,process};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug)]
enum FSNode<'a> {
    // size, parent id
    File(u64, u32),
    // Children, size, parent
    Directory(Vec<(u32,&'a str)>,RefCell<Option<u64>>,Option<u32>),
}

#[derive(Debug)]
struct FSTree<'a> {
    tree: HashMap<u32, FSNode<'a>>,
    max_id: u32,
}

impl<'a> FSTree<'a> {
    fn new() -> Self {
        let root = FSNode::Directory(vec![], RefCell::new(None), None);
        let mut tree = HashMap::new();
        tree.insert(0, root);
        Self { tree, max_id: 0 }
    }

    // Creates a new directory in the hierarchy with a new id, and return such id
    fn new_dir(&mut self, dirid: &u32, dirname: &'a str) -> u32 {
        if let Some(node) = self.tree.get_mut(dirid) {
            match node {
                FSNode::Directory(v,_,_) => {
                    let dir = FSNode::Directory(vec![], RefCell::new(None), Some(*dirid));
                    self.max_id += 1;
                    v.push((self.max_id,dirname));
                    self.tree.insert(self.max_id, dir);
                    self.max_id
                },
                _ => {
                    panic!("Node {dirid} is not a directory");
                },
            }
        } else {
            panic!("Directory {dirid} does not exist");
        }
    }

    fn new_file(&mut self, dirid: &u32, filename: &'a str, size: u64) {
        if let Some(node) = self.tree.get_mut(dirid) {
            match node {
                FSNode::Directory(v,_,_) => {
                    let file = FSNode::File(size,*dirid);
                    self.max_id += 1;
                    v.push((self.max_id,filename));
                    self.tree.insert(self.max_id, file);
                },
                _ => {
                    panic!("Node {dirid} is not a directory");
                },
            }
        } else {
            panic!("Directory {dirid} does not exist");
        }
    }

    fn size(&self, id: &u32) -> u64 {
        if let Some(node) = self.tree.get(id) {
            match node {
                FSNode::Directory(v,cell,_) => {
                    let mut opsize = cell.borrow_mut();
                    match *opsize {
                        Some(size) => size,
                        None => {
                            let mut sum = 0;
                            for (n,_) in v {
                                sum += self.size(n);
                            }
                            *opsize.insert(sum)
                        },
                    }
                },
                FSNode::File(size,_) => *size,
            }
        } else {
            panic!("Node {id} does not exist");
        }
    }
}

fn read_log(input: &str) -> FSTree {
    let mut tree = FSTree::new();
    let mut lines = input.lines();
    lines.next();
    let mut line = lines.next().unwrap();
    let mut current_dir = 0;
    loop {
        if let Some(dirname) = line.strip_prefix("$ cd") {
            let dir = tree.tree.get(&current_dir).unwrap();
            current_dir = match dirname.trim() {
                ".." => {
                    match *dir {
                        FSNode::Directory(_,_,Some(parent)) => parent,
                        _ => {
                            panic!("Directory {current_dir} has no parent");
                        },
                    }
                },
                s => {
                    match dir {
                        FSNode::Directory(v,_,_) => {
                            let mut id = 0;
                            let mut found = false;
                            for (n,name) in v {
                                if *name == s {
                                    id = *n;
                                    found = true;
                                    break;
                                }
                            }
                            if found {
                                id
                            } else {
                                panic!("Directory {s} is unreachable from {current_dir}");
                            }
                        },
                        _ => panic!("Node {current_dir} is not a directory"),
                    }
                },
            };
            if let Some(s) = lines.next() {
                line = s;
            } else {
                break;
            }
        } else if line.trim() == "$ ls" {
            let mut op = lines.next();
            while let Some(l) = op {
                if l.starts_with('$') {
                    break;
                }
                if let Some(dirname) = l.strip_prefix("dir ") {
                    let dirname = dirname.trim();
                    tree.new_dir(&current_dir,dirname);
                } else {
                    let vector: Vec<&str> = l.split(' ').collect();
                    let size = vector[0].parse::<u64>().unwrap();
                    tree.new_file(&current_dir, vector[1],size);
                }
                op = lines.next();
            }
            if let Some(s) = op {
                line = s;
            } else {
                break;
            }
        }
    }
    tree
}

fn run1(input: &str) -> u64 {
    let fstree = read_log(input);
    let max_size = 100000;
    let mut sum = 0;
    for (id,node) in &fstree.tree {
        match node {
            FSNode::Directory(_,_,_) => {
                let s = fstree.size(id);
                if s < max_size {
                    sum += s;
                }
            },
            _ => {},
        }
    }
    sum
}

fn run2(input: &str) -> u64 {
    let fstree = read_log(input);
    let total = 70_000_000;
    let required = 30_000_000;
    let used = fstree.size(&0);
    //println!("{used}");
    if total >= required + used {
        return 0;
    } else {
        let bound = required + used - total;
        //println!("{bound}");
        let mut min = 0;
        for (id,node) in &fstree.tree {
            match node {
                FSNode::Directory(_,_,_) => {
                    let s = fstree.size(id);
                    //println!("{id}: {s}");
                    if s >= bound && (min == 0 || s < min) {
                        min = s;
                    }
                },
                _ => {},
            }
        }
        return min;
    }
}

fn main() {
    let mut args = env::args();
    let filepath;
    args.next();
    if let Some(s) = args.next() {
        filepath = s;
    }
    else {
        eprintln!("Give me a file name! I must feeds on files! Aaargh!");
        process::exit(1);
    }

    let input = fs::read_to_string(filepath).unwrap();

    let res = run2(&input);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,95437);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res,1297159);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,24933642);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,3866390);
}
