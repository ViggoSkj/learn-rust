#[derive(Debug)]
struct BranchPoint {
    nodes: Vec<QuickSortNode>,
}

#[derive(Debug)]
struct EndPoint {
    values: Vec<u32>,
}

#[derive(Debug)]
enum QuickSortNode {
    EndPoint(EndPoint),
    BranchPoint(BranchPoint),
}

fn apply_quick_sort(node: &EndPoint) -> BranchPoint {
    let pivot = node.values[0];
    let mut left_end = EndPoint { values: vec![] };
    let mut right_end = EndPoint { values: vec![] };
    let mut same = 0;
    for v in &node.values {
        if *v < pivot {
            left_end.values.push(*v);
        } else if *v > pivot {
            right_end.values.push(*v);
        } else {
            same += 1;
        }
    }

    for i in 0..same {
        if i % 2 == 0 {
            left_end.values.push(pivot)
        } else {
            right_end.values.push(pivot)
        }
    }

    return BranchPoint {
        nodes: vec![
            QuickSortNode::EndPoint(left_end),
            QuickSortNode::EndPoint(right_end),
        ],
    };
}

fn quick_sort_cycle(node: &mut QuickSortNode) -> bool {
    match node {
        QuickSortNode::BranchPoint(_) => return false,
        QuickSortNode::EndPoint(vector) => {
            let mut same = true;
            for value in &vector.values {
                if *value != vector.values[0] {
                    same = false;
                    break;
                }
            }
            if same {
                return false;
            }

            *node = QuickSortNode::BranchPoint(apply_quick_sort(&vector));
            return true;
        }
    }
}

pub fn sort(vector: &mut Vec<u32>) {
    let mut tree = QuickSortNode::EndPoint(EndPoint {
        values: vector.clone(),
    });
    vector.clear();

    let mut path: Vec<usize> = Vec::new();
    'whole: loop {
        // find endpoint node
        let mut node = &mut tree;

        for p in &mut path {
            if let QuickSortNode::BranchPoint(branch) = node {
                if *p >= branch.nodes.len() {
                    path.pop();
                    continue 'whole;
                }

                node = &mut branch.nodes[*p];
            } 
        }

        if quick_sort_cycle(node) {
            path.push(0);
        } else {
            if path.last_mut().is_some() {
                *path.last_mut().unwrap() += 1;
            } else {
                break;
            }
            if let QuickSortNode::EndPoint(x) = node {
                vector.append(&mut x.values);
            }
        }
    }
}
