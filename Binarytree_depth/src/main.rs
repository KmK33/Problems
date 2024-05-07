
pub struct TreeNode{
    pub val:i32,
    pub left:Option<Box<TreeNode>>,
    pub right:Option<Box<TreeNode>>,
}

impl  TreeNode {
    pub fn new(val:i32) -> Self{
        TreeNode {
            val,
            left:None,
            right:None
        }
    }

    pub fn max_depth(root:Option<Box<TreeNode>>) -> i32{
        match root {
            None => 0,
            Some(node) => {
                let left_depth = TreeNode::max_depth(node.left);
                let right_depth = TreeNode::max_depth(node.right);

                1+std::cmp::max(left_depth, right_depth)
            }
        }
    }
}








fn main() {

    let root = Some(Box::new(TreeNode{
        val:1,
        left:Some(Box::new(TreeNode{
            val:2,
            left:Some(Box::new(TreeNode{
                val:4,
                left:Some(Box::new(TreeNode{
                    val:5,
                    left:None,
                    right:None

                })),
                right:None
            })),
            right:None

        })),
        right:Some(Box::new(TreeNode{
            val:3,
            left:None,
            right:None
        }))
    }));
    

    let depth = TreeNode::max_depth(root);

    println!("The max depth {depth}");
}
