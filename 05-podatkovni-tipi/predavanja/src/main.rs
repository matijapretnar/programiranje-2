enum Ocena {
    Zadostno6,
    Dobro7,
    PravDobro8,
    PravDobro9,
    Odlicno10,
    // Pozitivno(u8),
    Nezadostno,
    NiPristopil,
    SuperVisokaOcena(u128),
}

impl Ocena {
    fn v_vrednost(&self) -> Option<u8> {
        match self {
            Ocena::Zadostno6 => Some(6),
            Ocena::Dobro7 => Some(7),
            Ocena::PravDobro8 => Some(8),
            Ocena::PravDobro9 => Some(9),
            Ocena::Odlicno10 => Some(10),
            _ => None,
        }
    }
}

struct Student {
    ime: String,
    priimek: String,
    vpisna: u32,
    ocene: Vec<(String, Ocena)>,
}
fn povprecje(student: &Student) -> f32 {
    10.0
}
impl Student {
    fn povprecje(&self) -> f32 {
        let mut vsota = 0;
        let mut dolzina = 0;
        for (_, ocena) in self.ocene.iter() {
            match ocena.v_vrednost() {
                Some(v) => {
                    vsota += v;
                    dolzina += 1;
                }
                None => {}
            }
        }
        (vsota as f32) / (dolzina as f32)
    }
}

fn main() {
    let matija = Student {
        ime: String::from("Matija"),
        priimek: String::from("Pretnar"),
        vpisna: 27004498,
        ocene: vec![
            (String::from("Programiranje 1"), Ocena::PravDobro9),
            (String::from("Programiranje 2"), Ocena::PravDobro8),
        ],
    };
    println!("izračun prek funkcije: {}", povprecje(&matija));
    println!("izračun z metodo: {}", matija.povprecje());
}
