#[allow(unused)]
pub struct Thing {
    call_count: usize,
    value: i32,
    name: String,
}

impl Thing {
    pub fn new(value: i32, name: String) -> Self {
        Self {
            call_count: 0,
            value,
            name
        }
    }
    pub fn some_operation(&mut self) {
        self.call_count += 1;
    }
    pub fn odd(&self) -> bool {
        self.value % 2 == 1
    }
}

pub struct Logic {
    things: Vec<Thing>,
}

impl Logic {
    pub fn new() -> Self {
        Self { things: Vec::new() }
    }
    pub fn add_thing(&mut self, thing: Thing) {
        self.things.push(thing);
    }

    pub fn do_something(&mut self) {
        for thing in &mut self.things {
            thing.some_operation();
        }
    }

    pub fn do_something_odd_filter(&mut self) {
        for thing in &mut self.things {
            if thing.value % 2 == 1 {
                thing.some_operation();
            }
        }
    }
    pub fn do_something_external_odd_filter(&mut self) {
        let odd_things = self.things.iter_mut().filter(|thing| thing.value % 2 == 1);
        for thing in odd_things {
            thing.some_operation();
        }
    }
    pub fn count(&self) -> usize {
        self.things.iter().find(|t| t.odd()).expect("thing").call_count
    }
}
