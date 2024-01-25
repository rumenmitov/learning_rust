use blog_post_v2;

fn main() {
    let mut post = blog_post_v2::Post::new();

    post.add_text("I ate a salad today.");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad today.", post.content());
}
