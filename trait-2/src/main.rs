
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Drawing logic here
        println!("Drawing a button: {}", self.label);
    }
}

fn main() {

    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Cancel"),
            }),
        ],
    };

    screen.run();

    println!("Hello, world!");
}
