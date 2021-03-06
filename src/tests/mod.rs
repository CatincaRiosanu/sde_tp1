use crate::Vector;

#[test]
fn ver_add() {
    let mut vector1 = Vector::new();
    vector1.add(5);
    vector1.add(3);
    vector1.add(7);

    assert!(vector1.lista[0] == 3 && vector1.lista[2] == 7);
}

#[test]
fn ver_remove() {
    let mut vector1 = Vector::new();
    vector1.add(5);
    vector1.add(3);
    vector1.add(7);

    vector1.remove(5);
    // vector1.remove(9); Cum verific daca vreau sa dau remove la o valoare care nu exista?

    assert!(vector1.lista[1] == 7);
}

#[test]

fn ver_prim() {
    let mut vector1 = Vector::new();

    vector1.add(5);
    vector1.add(4);
    vector1.add(3);
    vector1.add(7);
    vector1.add(6);

    let vector2 = vector1.prim();

    assert!(vector2.lista[0] == 3);
}

#[test]

fn ver_cuprins() {

    let mut vector1 =Vector::new();

    vector1.add(5);
    vector1.add(4);
    vector1.add(3);
    vector1.add(7);
    vector1.add(6);

    let vector2 = vector1.cuprins(2, 5);

    assert!(vector2.lista[1] == 4)

}
