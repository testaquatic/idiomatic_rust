use std::borrow::Cow;

#[derive(Debug, Clone)]
struct CowList<'a> {
    cows: Cow<'a, [String]>,
}

impl<'a> CowList<'a> {
    fn add_cow(&self, cow: &str) -> Self {
        let mut new_cows = self.clone();
        new_cows.cows.to_mut().push(cow.to_string());

        new_cows
    }
}

impl Default for CowList<'_> {
    fn default() -> Self {
        CowList {
            cows: Cow::from(vec![]),
        }
    }
}

fn main() {
    let list_of_cows = CowList::default()
        .add_cow("Bessie")
        .add_cow("Daisy")
        .add_cow("Moo");
    dbg!(&list_of_cows);
    let list_of_cow_plus_one = list_of_cows.add_cow("Penelope");
    dbg!(&list_of_cows);
    dbg!(&list_of_cow_plus_one);
}
