/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }
}
        

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge<T: PartialOrd>(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
    let mut merged = LinkedList::new();

    // 我们需要可变地遍历两个链表，所以取出它们的 start 指针
    let mut curr_a = list_a.start;
    let mut curr_b = list_b.start;

    // 清空原链表的 start/end，防止 drop 时重复释放节点（所有权转移到 merged）
    list_a.start = None;
    list_a.end = None;
    list_b.start = None;
    list_b.end = None;

    // 经典的归并过程
    while let (Some(ptr_a), Some(ptr_b)) = (curr_a, curr_b) {
        // 安全地取出两个节点的值进行比较
        let node_a = unsafe { ptr_a.as_ref() };
        let node_b = unsafe { ptr_b.as_ref() };

        if node_a.val <= node_b.val {
            // 把 list_a 的当前节点移动到 merged
            let next_a = node_a.next;
            // 取出当前节点的所有权
            let mut node_box = unsafe { Box::from_raw(ptr_a.as_ptr()) };
            node_box.next = None; // 断开原来的 next
            let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node_box)) });

            // 加到 merged
            merged.append_node(node_ptr);

            curr_a = next_a;
        } else {
            // 同理处理 list_b 的节点
            let next_b = node_b.next;
            let mut node_box = unsafe { Box::from_raw(ptr_b.as_ptr()) };
            node_box.next = None;
            let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node_box)) });

            merged.append_node(node_ptr);

            curr_b = next_b;
        }
    }

    // 把剩余的节点直接接到 merged 后面
    if let Some(ptr) = curr_a {
        merged.append_remaining(ptr);
    }
    if let Some(ptr) = curr_b {
        merged.append_remaining(ptr);
    }

    merged
}

// 下面是两个辅助方法（放在 LinkedList<T> impl 块里）

// 把单个已经断开 next 的节点指针追加到链表末尾
fn append_node(&mut self, node_ptr: Option<NonNull<Node<T>>>) {
    if let Some(ptr) = node_ptr {
        match self.end {
            None => self.start = Some(ptr),
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = Some(ptr) },
        }
        self.end = Some(ptr);
        self.length += 1;
    }
}

// 把剩余的一整段链表直接接到末尾（不需要逐个拆）
fn append_remaining(&mut self, mut remaining: Option<NonNull<Node<T>>>) {
    if let Some(first) = remaining {
        // 找到剩余段的最后一个节点
        let mut last = first;
        loop {
            let node = unsafe { last.as_ref() };
            self.length += 1;
            if let Some(next) = node.next {
                last = next;
            } else {
                break;
            }
        }

        // 连接
        match self.end {
            None => self.start = remaining,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = remaining },
        }
        self.end = Some(last);
    }
}