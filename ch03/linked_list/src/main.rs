use linked_list::LinkedList;

fn main() {
    let mut dinosaurs = LinkedList::new("Tyrannosaurus Rex");
    dinosaurs.append("Triceratops");
    dinosaurs.append("Velociraptor");
    dinosaurs.append("Stegosaurus");
    dinosaurs.append("Spinosaurus");
    dinosaurs.iter().for_each(|data| println!("data={}", data));
    dinosaurs
        .iter_mut()
        .for_each(|data| println!("data={}", data));
    for data in &dinosaurs {
        println!("data={}", data);
    }
}
