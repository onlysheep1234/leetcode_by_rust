mod common;
use common::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
use std::vec::Vec;

pub struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec!();
        }

        let mut ret = Vec::new();
        Self::visit(root.as_ref(), sum, &vec!(), &mut ret);
        return ret;
    }

    fn visit(root:Option<&Rc<RefCell<TreeNode>>>, left: i32, path: &Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if root.is_none(){
            return;
        }
        
        let r = root.unwrap().borrow();
        let is_leaf = r.left.is_none() && r.right.is_none();
        let left = left - r.val;
        if is_leaf && left == 0 {
            let mut p = path.clone();
            p.push(r.val);
            result.push(p);
        } else if !is_leaf { //可能是负数，故此处不可加上left>0条件
            let mut p = path.clone();
            p.push(r.val);
            if !r.left.is_none() {
               Self::visit(r.left.as_ref(), left, &p, result); 
            } 
            
            if !r.right.is_none() {
               Self::visit(r.right.as_ref(), left, &p, result);
            }
        }
    }
}