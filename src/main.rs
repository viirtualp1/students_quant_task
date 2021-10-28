#[derive(Debug)]

struct Student {
    full_name: String,
    group: u16,
    appraisals: [u8; 5]
}

impl Student {
    fn new(full_name: String, group: u16, appraisals: [u8; 5]) -> Self {
        Student {
            full_name,
            group,
            appraisals
        }
    }

    fn mean(&self) -> f32 {
        let mut ret = 0.0;

        for i in self.appraisals {
            ret += i as f32
        }

        ret / self.appraisals.len() as f32
    }
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    students.push(Student::new("Зинин Н.А.".to_string(), 101, [2, 3, 4, 3, 5])); // 3.4
    students.push(Student::new("Антонов Н.А.".to_string(), 121, [2, 2, 4, 3, 5])); // 3.2
    students.push(Student::new("Путин Н.А.".to_string(), 131, [2, 5, 3, 3, 5])); // 3.6
    students.push(Student::new("Милонов Н.А.".to_string(), 141, [2, 5, 2, 3, 5])); // 3.4
    students.push(Student::new("Байден Н.А.".to_string(), 151, [2, 5, 4, 2, 5])); // 3.6
    students.push(Student::new("Обама Н.А.".to_string(), 161, [2, 5, 4, 5, 5])); // 4.2
    students.push(Student::new("Кучма Н.А.".to_string(), 171, [2, 5, 4, 3, 5])); // 3.8
    students.push(Student::new("Порошенко Н.А.".to_string(), 181, [2, 5, 2, 2, 5])); // 3.2
    students.push(Student::new("Меладзе Н.А.".to_string(), 191, [2, 5, 4, 4, 5])); // 4
    students.push(Student::new("Антонов Н.А.".to_string(), 100, [2, 5, 4, 5, 5])); // 4.2

    // 3.2
    // 3.2
    // 3.4
    // 3.4
    // 3.6
    
    // 3.6 //

    // 3.8
    // 4
    // 4.2

    let mut student_array_sort: Vec<Student> = Vec::new();

    for stud in students.into_iter() {
        if student_array_sort.is_empty() {
            student_array_sort.push(stud);
            continue;
        }

        // println!("{}, {}", stud.mean(), student_array_sort.last().unwrap().mean());

        if stud.mean() > student_array_sort.last().unwrap().mean() ||
            stud.mean() == student_array_sort.last().unwrap().mean() {
            student_array_sort.push(stud);
            continue
        }

        for (i, sort_stud) in student_array_sort.iter().enumerate() {
            if sort_stud.mean() > stud.mean() {
                student_array_sort.insert(i, stud);

                break;
            } 
        }
    }

    for (i, stud) in student_array_sort.iter().enumerate() {
        println!("{} - {} - {}", i, stud.full_name, stud.mean());
    }
}