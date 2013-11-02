//
// btree.rs
// Nif Ward
// 10/24/13
//
// starting implementation of a btree for rust
// inspired by github user davidhalperin's gist


//What's in a BTree?
pub struct BTree<K, V>{
    root: Node<K, V>,
    len: uint,
    lower_bound: uint,
    upper_bound: uint
}

impl<K: TotalOrd, V> BTree<K, V>{
    
    //Returns new BTree with root node (leaf) and user-supplied lower bound
    fn new(k: K, v: V, lb: uint) -> BTree<K, V>{
        BTree{
	    root: Node::new_leaf(~[LeafElt::new(k, v)]),
	    len: 1,
	    lower_bound: lb,
	    upper_bound: 2 * lb
        }
    }

    fn get(self, k: K) -> Option<V>{
        return self.root.get(k);
    }
    
}

impl<K: ToStr + TotalOrd, V: ToStr> ToStr for BTree<K, V>{
    //Returns a string representation of the BTree
    fn to_str(&self) -> ~str{
        let ret=self.root.to_str();
	return ret;
    }


}

//Node types
enum Node<K, V>{
    LeafNode(Leaf<K, V>),
    BranchNode(Branch<K, V>)
}


//Node functions/methods
impl<K: TotalOrd, V> Node<K, V>{
    //differentiates between leaf and branch nodes
    fn is_leaf(self) -> bool{
        match self{
	    LeafNode(*) => true,
	    BranchNode(*) => false
        }
    }
    
    //Creates a new leaf or branch node
    fn new_leaf(vec: ~[LeafElt<K, V>]) -> Node<K,V>{
         LeafNode(Leaf::new(vec))
    }
    fn new_branch(vec: ~[BranchElt<K, V>], right: ~Node<K, V>) -> Node<K, V>{
        BranchNode(Branch::new(vec, right))
    }

    fn get(self, k: K) -> Option<V>{
        match self{
	    LeafNode(leaf) => return leaf.get(k),
	    BranchNode(branch) => return branch.get(k)
        }
    }
}


impl<K: ToStr + TotalOrd, V: ToStr> ToStr for Node<K, V>{
    fn to_str(&self) -> ~str{
       match *self{
           LeafNode(ref leaf) => leaf.to_str(),
	   BranchNode(*) => ~""
       }
    }
}


//Array with no children
struct Leaf<K, V>{
    elts: ~[LeafElt<K, V>]
}

//Array of values with children, plus a rightmost child (greater than all)
struct Branch<K, V>{
    elts: ~[BranchElt<K,V>],
    rightmost_child: ~Node<K, V>
}


impl<K: TotalOrd, V> Leaf<K, V>{
    //Constructor takes in a vector of leaves
    fn new(vec: ~[LeafElt<K, V>]) -> Leaf<K, V>{
        Leaf{
            elts: vec
        }
    }


    fn get(self, k: K) -> Option<V>{
        for s in self.elts.iter(){
	    let order=s.key.cmp(&k);
	    match order{
	        Equal => return Some(s.value),
		_ => {}
	    }
	}
	return None;
    }

}

impl<K: ToStr + TotalOrd, V: ToStr> ToStr for Leaf<K, V>{
    fn to_str(&self) -> ~str{
       let mut ret=~"";
       for s in self.elts.iter(){
           ret = ret+" // "+ s.to_str();
       }
       return ret;
    }

}


impl<K: TotalOrd, V> Branch<K, V>{
    //constructor takes a branch vector and a rightmost child
    fn new(vec: ~[BranchElt<K, V>], right: ~Node<K, V>) -> Branch<K, V>{
        Branch{
	    elts: vec,
	    rightmost_child: right
        }
    }

    fn get(self, k: K) -> Option<V>{
        for s in self.elts.iter(){
	    let order = s.key.cmp(&k);
	    match order{
	        Less => return s.left.get(k),
		Equal => return Some(s.value),
		_ => {}
	    }
	}
	return self.rightmost_child.get(k);
    }
}

//No left child
struct LeafElt<K, V>{
    key: K,
    value: V
}

//Has a left child
struct BranchElt<K, V>{
    left: Node<K, V>,
    key: K,
    value: V
}

impl<K: TotalOrd, V> LeafElt<K, V>{
    fn new(k: K, v: V) -> LeafElt<K, V>{
        LeafElt{
            key: k,
	    value: v
	}
    }

    fn less_than(self, other: LeafElt<K, V>) -> bool{
        let order = self.key.cmp(&other.key);
	match order{
	    Less => true,
	    _ => false
	}
    }

    fn greater_than(self, other: LeafElt<K, V>) -> bool{
        let order = self.key.cmp(&other.key);
	match order{
	    Greater => true,
	    _ => false
	}
    }


    fn has_key(self, other: K) -> bool{
        let order = self.key.cmp(&other);
	match order{
	    Equal => true,
	    _ => false
	}
    }

}

impl<K: ToStr + TotalOrd, V: ToStr> ToStr for LeafElt<K, V>{
    fn to_str(&self) -> ~str{
        return "Key: "+self.key.to_str()+", value: "+self.value.to_str()+"; ";
    }

}

impl<K: TotalOrd, V> BranchElt<K, V>{
    fn new(k: K, v: V, n: Node<K, V>) -> BranchElt<K, V>{
        BranchElt{
            left: n,
            key: k,
            value: v
        }
    }
}



fn main(){
    let b: BTree<int, ~str>;
    b = BTree::new(1, ~"taco", 2);
    println(b.to_str());
}