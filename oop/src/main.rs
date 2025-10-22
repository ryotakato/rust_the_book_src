use oop::AveragedCollection;
use oop::Post;
use oop::AltPost;

fn main() {

    alt_blog();

    blog();

    average();
}


fn alt_blog() {
    println!("-- alt_blog ---------------------------------");
    let mut post = AltPost::new();

    post.add_text("I ate a salad for lunch today. ");
    println!("{:?}", post);

    post.add_text("Then, I dropped my fork.");
    println!("{:?}", post);

    let post = post.request_review();
    println!("{:?}", post);
    let post = post.approve();
    println!("{:?}", post);
    let post = post.approve();
    println!("{:?}", post);

    assert_eq!("I ate a salad for lunch today. Then, I dropped my fork.", post.content());
    println!("{}", post.content());



}

fn blog() {
    println!("-- blog ---------------------------------");
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    println!("{}", post.content());

    post.request_review();
    assert_eq!("", post.content());
    println!("{}", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("{}", post.content());


}

fn average() {
    println!("-- average ---------------------------------");

    //let mut average_collection = AveragedCollection::new(vec![2,5,16,62,82,35,21,79]);
    let mut average_collection = AveragedCollection::new(vec![2,3,4,5]);


    println!("avg = {}", average_collection.average());

    average_collection.add(6);

    println!("avg = {}", average_collection.average());

}
