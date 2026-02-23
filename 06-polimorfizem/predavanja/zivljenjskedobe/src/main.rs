fn prvi_niz<'a, 'b>(sklic_na_niz1: &'a String, sklic_na_niz2: &'b String) -> &'a String {
    sklic_na_niz1
}

fn daljsi_niz<'a>(sklic_na_niz1: &'a String, sklic_na_niz2: &'a String) -> &'a String {
    if sklic_na_niz1.len() >= sklic_na_niz2.len() {
        sklic_na_niz1
    } else {
        sklic_na_niz2
    }
}
fn daljsi_vektor<'a, T>(sklic_na_vektor1: &'a Vec<T>, sklic_na_vektor2: &'a Vec<T>) -> &'a Vec<T> {
    if sklic_na_vektor1.len() >= sklic_na_vektor2.len() {
        sklic_na_vektor1
    } else {
        sklic_na_vektor2
    }
}

fn main() {
    let daljsi: &String;
    let niz1 = String::from("abcsddsafsd");
    {
        let niz2 = String::from("defg");
        daljsi = daljsi_niz(&niz1, &niz2);
    }
    println!("{daljsi} je daljsi od obeh nizov");
}
