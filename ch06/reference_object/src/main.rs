use reference_object::StudentList;

fn main() {
    let student_list = StudentList::new(&[("Lyle", 621), ("Anna", 286)]);
    dbg!(&student_list);
    dbg!(student_list.find_student_by_id(621));
    dbg!(student_list.find_student_by_name("Anna"));

    let student_ref_621 = student_list.find_student_by_id(621).unwrap();
    let student_ref_286 = student_list.find_student_by_id(286).unwrap();
    dbg!(student_ref_621 == student_ref_286);
    dbg!(student_ref_621 != student_ref_286);
}
