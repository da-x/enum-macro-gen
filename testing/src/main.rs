#![allow(unused)] // TODO
                  //
use enum_macro_gen::EnumMacroGen;

struct Item {
    x: u32,
}

#[derive(EnumMacroGen)]
#[enum_macro[handle_test={match: $self.handle_$variant($fields);}]]
enum Test {
    Foo(Item),
    Double(Item, Box<Test>),
    Bar,
}

struct Main {}

impl Main {
    fn test(&self, test: &Test) {
        handle_test! {self, test}
    }

    fn handle_foo(&self, item: &Item) {}

    fn handle_double(&self, item: &Item, test: &Box<Test>) {}

    fn handle_bar(&self) {}
}

fn main() {}
