use std::vec;

fn main() {
    let mut vector1 = Vector::new();
    vector1.add(5);
    vector1.add(3);
    vector1.add(7);
    vector1.add(8);
    vector1.add(4);
    vector1.add(3);
    vector1.add(2);
    vector1.add(1);
    vector1.add(10);

    for i in 0..vector1.lista.len() {
        println!("{}", vector1.lista[i]);
    }

    vector1.remove(5);
    vector1.remove(9);

    for i in 0..vector1.lista.len() {
        println!("{}", vector1.lista[i]);
    }

    println!();

    let vector2 = vector1.prim();

    for i in 0..vector2.lista.len() {
        println!("{}", vector2.lista[i]);
    }

    println!();

    let vector3 = vector1.cuprins(2, 5);

    for i in 0..vector3.lista.len() {
        println!("{}", vector3.lista[i])
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

    fn remove(&mut self, variabila: i32) {
        let mut pozitie = None;

        for index in 0..self.lista.len() {
            if variabila == self.lista[index] {
                pozitie = Some(index);
            }
        }

        match pozitie {
            Some(pozitie) => {
                //?
                self.lista.remove(pozitie);
            }
            None => println!("Valoarea introdusa nu exista in vector!"),
        }
    }

    fn prim(&self) -> Vector {
        let mut aux = Vector::new();

        for element in &self.lista {
            let mut p_prim = true;

            if element <= &1 {
                p_prim = false;
            } else {
                for index in 2..*element {
                    if element % index == 0 {
                        p_prim = false;
                    }
                }
            }

            if p_prim {
                aux.add(*element);
            }
        }

        return aux;
    }

    fn cuprins(&self, minim: i32, maxim: i32) -> Vector {
        let mut aux = Vector::new();

        for index in 0..self.lista.len() {
            if self.lista[index] >= minim && self.lista[index] <= maxim {
                aux.add(self.lista[index]);
            }
        }

        return aux;
    }
}

#[cfg(test)]
mod tests;
//  {

//     use super::*;
//     #[test]
//     fn ver_add() {
//         let mut vector1 = Vector::new();
//         vector1.add(5);
//         vector1.add(3);
//         vector1.add(7);

//         assert!(vector1.lista[0] == 3 && vector1.lista[2] == 7);
//     }

//     #[test]
//     fn ver_remove() {
//         let mut vector1 = Vector::new();
//         vector1.add(5);
//         vector1.add(3);
//         vector1.add(7);

//         vector1.remove(5);
//        // vector1.remove(9); Cum verific daca vreau sa dau remove la o functie in test? 

//        assert!(vector1.lista[1] == 7);
//     }
// }
