
// AveragedCollection : OOP

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64
}

impl AveragedCollection {
    pub fn new(list: Vec<i32>) -> AveragedCollection {
        let mut average_collection = AveragedCollection {
            list,
            average: 0.0
        };
        average_collection.update_average();
        average_collection
    }



    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}



// Post : State Pattern

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}


struct Draft {}
struct PendingReview {}
struct Published {}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}


impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}


impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}



// AltPost : More Rust-style

#[derive(Debug)]
pub struct AltPost {
    content: String,
}

#[derive(Debug)]
pub struct AltDraftPost {
    content: String,
}

#[derive(Debug)]
pub struct AltPending1ReviewPost {
    content: String,
}

#[derive(Debug)]
pub struct AltPending2ReviewPost {
    content: String,
}

impl AltPost {
    pub fn new() -> AltDraftPost {
        AltDraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl AltDraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> AltPending1ReviewPost {
        AltPending1ReviewPost {
            content: self.content,
        }
    }
}

impl AltPending1ReviewPost {
    pub fn approve(self) -> AltPending2ReviewPost {
        AltPending2ReviewPost {
            content: self.content,
        }
    }
    pub fn reject(self) -> AltDraftPost {
        AltDraftPost {
            content: self.content,
        }
    }
}

impl AltPending2ReviewPost {
    pub fn approve(self) -> AltPost {
        AltPost {
            content: self.content,
        }
    }
    pub fn reject(self) -> AltDraftPost {
        AltDraftPost {
            content: self.content,
        }
    }
}
