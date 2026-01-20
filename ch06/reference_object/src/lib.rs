#[derive(Debug)]
struct Student {
    name: String,
    id: u32,
}

impl Student {
    fn new(name: String, id: u32) -> Student {
        Student { name, id }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.id
    }
}

impl<'a> Student {
    fn to_ref(&'a self) -> StudentRef<'a> {
        StudentRef::new(self)
    }
}

#[derive(Debug)]
pub struct StudentList {
    students: Vec<Student>,
}

impl StudentList {
    pub fn new(students: &[(&str, u32)]) -> Self {
        Self {
            students: students
                .iter()
                .map(|(name, id)| Student::new(name.to_string(), *id))
                .collect(),
        }
    }
}

impl<'a> StudentList {
    fn find<F: Fn(&&Student) -> bool>(&'a self, pred: F) -> Option<StudentRef<'a>> {
        self.students.iter().find(pred).map(Student::to_ref)
    }

    pub fn find_student_by_id(&'a self, id: u32) -> Option<StudentRef<'a>> {
        self.find(|s| s.id() == id)
    }

    pub fn find_student_by_name(&'a self, name: &str) -> Option<StudentRef<'a>> {
        self.find(|s| s.name() == name)
    }
}

#[derive(Debug)]
pub struct StudentRef<'a> {
    student: &'a Student,
}

impl<'a> StudentRef<'a> {
    fn new(student: &'a Student) -> StudentRef<'a> {
        StudentRef { student }
    }
}

impl<'a> PartialEq for StudentRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.student.id() == other.student.id()
    }
}
