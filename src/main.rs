extern crate quickersort;

#[derive(Debug)]

struct Student {
    full_name: String,
    group: u32,
    appraisals: [u32; 5]
}

fn main() {
    let students: [&Student; 10] = [
        &Student {full_name: String::from("Зинин Н.А."), group: 2, appraisals: [2, 3, 4, 5, 2]},
        &Student {full_name: String::from("Антонов П.П."), group: 3, appraisals: [2, 2, 4, 5, 2]},
        &Student {full_name: String::from("Андреев В.В."), group: 4, appraisals: [2, 5, 4, 5, 2]},
        &Student {full_name: String::from("Путин В.В."), group: 10, appraisals: [2, 4, 4, 5, 2]},
        &Student {full_name: String::from("Милонов В.А."), group: 7, appraisals: [2, 3, 2, 5, 2]},
        &Student {full_name: String::from("Меладзе П.П."), group: 6, appraisals: [2, 3, 4, 5, 2]},
        &Student {full_name: String::from("Кучма М.М."), group: 11, appraisals: [2, 3, 5, 5, 2]},
        &Student {full_name: String::from("Порошенко А.Б."), group: 5, appraisals: [2, 5, 5, 5, 2]},
        &Student {full_name: String::from("Обама Б.Б."), group: 12, appraisals: [2, 2, 5, 2, 2]},
        &Student {full_name: String::from("Байден Д.Ж."), group: 5, appraisals: [5, 4, 4, 5, 5]},
    ];

    let mut students_good = vec![];

    for i in 0..students.len() {
        for j in 0..students[i].appraisals.len() {
            if students[i].appraisals[j] == 4 || students[i].appraisals[j] == 5 {
                students_good.push(students[i]);
            } else if students_good.len() != 0 && students[i].appraisals[j] == 2
                        || students_good.len() != 0 && students[i].appraisals[j] == 3 {
                students_good.remove(students_good.len() - 1);
            }
        }
    }

    for i in 0..students_good.len() {
        if i >= 1 {
            if students_good[i].full_name != students_good[i-1].full_name {
                println!("{:?}", students_good[i]);                
            }
        } else {
            println!("Учащийся {} в группе {} имеет оценки только 4 и 5!", students_good[i].full_name, students_good[i].group);
        }
    }
}