#![cfg_attr(test, feature(proc_macro_hygiene))]

#[cfg(test)]
use mocktopus::macros::mockable;

trait Watch {
    fn watch(&self) -> String;
}

#[cfg(test)]
#[cfg_attr(test, mockable)]
mod test {
    use crate::*;
    use mocktopus::mocking::*;

    struct Watcher {}

    impl Watch for Watcher {
        fn watch(&self) -> String {
            unreachable!()
        }
    }

    #[test]
    fn mock_test() {
        Watcher::watch.mock_safe(|_| MockResult::Return("mocking".into()));

        assert_eq!("Hello mocking!", hello_world(Watcher {}));
    }
}

fn hello_world(w: impl Watch) -> String {
    format!("Hello {}!", w.watch())
}

fn main() {
    println!("Hello, world!");
}
