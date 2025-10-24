struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        Container { value }
    }
}

enum NextNode<T> {
    Next(Box<ListNode<T>>),
    End,
}

struct ListNode<T> {
    data: Box<T>,
    next: NextNode<T>,
}

fn main() {
    let str_container = Container {
        value: "Thought is free.",
    };

    println!("{}", str_container.value);

    let ambiguous_container = Container {
        value: Option::<String>::None,
    };

    let short_alt_ambiguous_container = Container::<Option<String>>::new(Option::<String>::None);
}
