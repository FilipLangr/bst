use bst::BST;


fn main() {
    let mut bst: BST<i32> = BST::new();
    println!("BST: {:?}", bst);
    bst.insert(10);
    println!("BST: {:?}", bst);
    bst.insert(5);
    println!("BST: {:?}", bst);
    bst.insert(15);
    println!("BST: {:?}", bst);
    bst.insert(12);
    println!("BST: {:?}", bst);
    bst.insert(18);
    println!("BST: {:?}", bst);
    bst.insert(11);
    println!("BST: {:?}", bst);
    bst.insert(13);
    println!("BST: {:?}", bst);
    bst.insert(17);
    println!("BST: {:?}", bst);
    bst.insert(19);
    println!("BST: {:?}", bst);

    println!("BST contains 15: {}", bst.contains(15));
    println!("BST contains 1968: {}", bst.contains(1968));

    println!("***********************************************");
    println!("BST: {:?}", bst);
    println!("***********************************************");
    bst.delete(15);
    println!("BST: {:?}", bst);
    println!("***********************************************");
    bst.delete(17);
    println!("BST: {:?}", bst);
    println!("***********************************************");
    bst.delete(1968);
    println!("BST: {:?}", bst);
    println!("***********************************************");
    bst.delete(5);
    println!("BST: {:?}", bst);
    println!("***********************************************");
    bst.delete(11);
    println!("BST: {:?}", bst);
    println!("***********************************************");
    bst.delete(19);
    println!("BST: {:?}", bst);
    println!("***********************************************");
    bst.delete(18);
    println!("BST: {:?}", bst);
    println!("***********************************************");
    bst.delete(12);
    println!("BST: {:?}", bst);
    println!("***********************************************");
    bst.delete(10);
    println!("BST: {:?}", bst);
    println!("***********************************************");
    bst.delete(13);
    println!("BST: {:?}", bst);
    println!("***********************************************");

    let mut bst: BST<f32> = BST::new();
    println!("BST: {:?}", bst);
    bst.insert(13.2);
    println!("BST: {:?}", bst);
    bst.insert(5.1);
    println!("BST: {:?}", bst);
    bst.insert(3.9);
    println!("BST: {:?}", bst);
    bst.insert(9.6);
    println!("BST: {:?}", bst);
}
