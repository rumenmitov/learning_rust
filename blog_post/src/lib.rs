pub struct Post {
    state :Option<Box<dyn State>>,
    content :String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {
                changes: String::new()
            })),
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn add_text(&mut self, text :&str) {
        self.state.as_mut().unwrap().add_text(text);
    }

    pub fn request_review(&mut self) {
        let changes = self.state.take().unwrap().get_changes();
        if changes != "" {
            self.content = changes;
        }
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn add_text(&mut self, _ :&str) {
        ()
    }

    fn get_changes(self :Box<Self>) -> String {
        String::new()
    }

    fn request_review(self :Box<Self>) -> Box<dyn State>;
    fn approve(self :Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _ :&'a Post) -> &'a str {
        ""
    }
    fn reject(self :Box<Self>) -> Box<dyn State>;
}

struct Draft {
    changes :String,
}

impl State for Draft {
    fn add_text(&mut self, changes :&str) {
        self.changes.push_str(changes);
    }

    fn get_changes(self :Box<Self>) -> String {
        String::from(self.changes)
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {
            approved_count: 0,
        })
    }

    fn approve(self :Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self :Box<Self>) -> Box<dyn State> {
        self
    }

}

struct PendingReview {
    approved_count :u8,
}

impl State for PendingReview {
    fn request_review(self :Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self :Box<Self>) -> Box<dyn State> {
        if self.as_ref().approved_count == 1 {
            Box::new(Published {})

        } else {
            self.as_mut().approved_count += 1;
            self
        }
    }

    fn reject(self :Box<Self>) -> Box<dyn State> {
        Box::new(Draft {
            changes: String::new()
        })
    }
}

struct Published {}

impl State for Published {
    fn request_review(self :Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self :Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self :Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post :&'a Post) -> &'a str {
        &post.content
    }
}
