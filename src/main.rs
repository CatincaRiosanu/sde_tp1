use std::vec;

fn main() {
    let mut vector1 = Vector::new();
    vector1.add(5);
    vector1.add(3);
    vector1.add(7);
    vector1.add(8);
    vector1.add(4);
    vector1.add(3);

    for i in 0..vector1.lista.len() {
        println!("{}", vector1.lista[i]);
    }
}

struct Vector {
    lista: Vec<i32>,
}

impl Vector {
    fn new() -> Vector {
        Vector { lista: vec![] }
    }

    // fn add(&mut self, variabila: i32) {
    //     if self.lista.len() == 0 {
    //         self.lista.insert(0, variabila);
    //     } else if self.lista.len() == 1 {
    //         if variabila < self.lista[0] {
    //             self.lista.insert(0, variabila);
    //         } else {
    //             self.lista.insert(1, variabila);
    //         }
    //     } else {
    //         for index in 0..self.lista.len() -1 {
    //             if variabila > self.lista[index] && variabila <= self.lista[index + 1] {
    //                 self.lista.insert(index + 1, variabila);
    //                 break;
    //             } else if variabila > self.lista[index] && index == self.lista.len()-1 {
    //                 self.lista.insert(index, variabila);
    //             }
    //         }
    //     }
    // }

    fn add(&mut self, variabila: i32) {
        let mut pozitie: usize = 0;

        for index in 0..self.lista.len() {
            if self.lista[index] < variabila {
                pozitie += 1;
            }
        }

        self.lista.insert(pozitie, variabila)
    }
}
