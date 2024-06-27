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

    println!("***********************************************");
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

    for item in &bst {
        println!("{:?}", item);
    }
    println!("***********************************************");
    for item in bst.iter() {
        println!("{:?}", item);
    }
    println!("***********************************************");
    println!("BST: {:?}", bst);

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

    for item in &bst {
        println!("{:?}", item);
    }

    bst.insert(1313);
    println!("BST: {:?}", bst);

    for item in &bst {
        println!("{:?}", item);
    }

    for item in bst {
        println!("{:?}", item);
    }

    // println!("BST: {:?}", bst);  // compiler would correctly scream

    let mut bst: BST<&str> = BST::new();
    bst.insert("bbb");
    bst.insert("aaa");
    bst.insert("ccc");

    println!("BST: {:?}", bst);
    println!("***********************************************");
    for item in &bst {
        println!("{:?}", item);
    }
    println!("***********************************************");
    for item in bst.iter() {
        println!("{:?}", item);
    }
    println!("***********************************************");
    println!("BST: {:?}", bst);
    println!("***********************************************");
    for item in bst {
        println!("{:?}", item);
    }

    let mut bst: BST<i32> = BST::new();
    bst.insert(25);
    bst.insert(50);
    bst.insert(75);
    bst.insert(100);
    bst.insert(120);
    bst.insert(150);
    bst.insert(170);
    bst.insert(200);
    bst.insert(210);
    bst.insert(220);
    bst.insert(230);
    bst.insert(250);
    bst.insert(260);
    bst.insert(270);
    bst.insert(280);
    println!("***********************************************");
    println!("BST: {:?}", bst);
    for item in &bst {
        println!("{:?}", item);
    }
    println!("***********************************************");
}
