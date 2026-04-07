mod izraz;
mod zaporedje;

use izraz::*;
use zaporedje::*;

// Pomožna funkcija za izpis prvih n členov poljubnega zaporedja.
fn izpisi_zaporedje<T: std::fmt::Display>(zap: &impl Zaporedje<T>, n: u64) {
    print!("{}: ", zap.name());
    for i in 1..=n {
        if i > 1 {
            print!(", ");
        }
        print!("{}", zap.k_th(i));
    }
    println!();
}

fn main() {
    // ============================================================
    // 1. GENERIČNI IZRAZI
    // ============================================================

    println!("=== Generični izrazi (i64) ===");

    // 1 + (2 - 3) = 0
    let e1: Izraz<i64> = Izraz::op(
        Izraz::konst(1),
        BinOperacija::Plus,
        Izraz::op(Izraz::konst(2), BinOperacija::Minus, Izraz::konst(3)),
    );
    println!("{} = {}", e1.izpis(), e1.eval_brez_spr());

    // (1 + 2) * 3 = 9
    let e2: Izraz<i64> = Izraz::op(
        Izraz::op(Izraz::konst(1), BinOperacija::Plus, Izraz::konst(2)),
        BinOperacija::Times,
        Izraz::konst(3),
    );
    println!("{} = {}", e2.izpis(), e2.eval_brez_spr());

    // Izraz s spremenljivko: x + 2 * y
    let e3: Izraz<i64> = Izraz::op(
        Izraz::spr("x"),
        BinOperacija::Plus,
        Izraz::op(Izraz::konst(2), BinOperacija::Times, Izraz::spr("y")),
    );
    println!("{}", e3.izpis());
    let vrednost = e3.eval(&|ime| match ime {
        "x" => 10,
        "y" => 5,
        _ => panic!("Neznana spremenljivka: {}", ime),
    });
    println!("  pri x=10, y=5: {}", vrednost);

    // collect
    println!("e1 collect = {}", e1.collect());
    println!("e3 collect = {}", e3.collect());

    // Display
    println!("e1 = {}", e1);

    println!("\n=== Generični izrazi (f64) ===");
    let ef: Izraz<f64> = Izraz::op(Izraz::konst(1.5), BinOperacija::Plus, Izraz::konst(2.5));
    println!("{} = {}", ef.izpis(), ef.eval_brez_spr());

    // ============================================================
    // 2. ZNAČILNOST Zaporedje<T>
    // ============================================================

    println!("\n=== Konstantno zaporedje ===");
    let konst = KonstantnoZaporedje::new("c=42", 42i64);
    izpisi_zaporedje(&konst, 5);
    println!("Vsebuje 42? {}", konst.contains(&42));
    println!("Vsebuje 7?  {}", konst.contains(&7));

    println!("\n=== Aritmetično zaporedje (i64) ===");
    let arit = AritmeticnoZaporedje::new("a", 1i64, 3i64);
    izpisi_zaporedje(&arit, 6); // 1, 4, 7, 10, 13, 16
    println!("Vsebuje 10? {}", arit.contains(&10));
    // println!("Vsebuje 5?  {}", arit.contains(&5)); // teče v nedogled

    println!("\n=== Aritmetično zaporedje (f64) ===");
    let arit_f = AritmeticnoZaporedje::new("b", 0.5f64, 1.5f64);
    izpisi_zaporedje(&arit_f, 5); // 0.5, 2.0, 3.5, 5.0, 6.5

    println!("\n=== PartialEq za aritmetična zaporedja ===");
    let a1 = AritmeticnoZaporedje::new("x", 1i64, 2i64);
    let a2 = AritmeticnoZaporedje::new("y", 1i64, 2i64);
    let a3 = AritmeticnoZaporedje::new("z", 1i64, 3i64);
    println!("a1 == a2? {} (pričakovano: true)", a1 == a2);
    println!("a1 == a3? {} (pričakovano: false)", a1 == a3);

    println!("\n=== Zmnožek aritmetičnih zaporedij ===");
    let prod = a1.zmnozi(&a3);
    izpisi_zaporedje(&prod, 5);

    println!("\n=== Geometrijsko zaporedje ===");
    let geom = GeometrijskoZaporedje::new("g", 2i64, 3i64);
    izpisi_zaporedje(&geom, 6); // 2, 6, 18, 54, 162, 486
    println!("Vsebuje 18? {}", geom.contains(&18));

    println!("\n=== Fibonaccijevo zaporedje ===");
    let fib = FibonacciZaporedje::new("fib", 0i64, 1i64);
    izpisi_zaporedje(&fib, 10); // 0, 1, 1, 2, 3, 5, 8, 13, 21, 34
    println!("Vsebuje 21? {}", fib.contains(&21));
    // println!("Vsebuje 4?  {}", fib.contains(&4)); // teče v nedogled (4 ni Fibonaccijevo število)

    println!("\n=== Zamaknjeno zaporedje (generična verzija) ===");
    let zamaknjeno = ZamaknjenoZaporedje::new(&fib, 5);
    izpisi_zaporedje(&zamaknjeno, 5); // F(6)..F(10) = 5, 8, 13, 21, 34   <- tu potrebujemo Blanket impl

    println!("\n=== Zamaknjeno zaporedje (dyn verzija) ===");
    let zamaknjeno_d = ZamaknjenoDyn::new(&fib, 5);
    izpisi_zaporedje(&zamaknjeno_d, 5); // enako: 5, 8, 13, 21, 34

    // ============================================================
    // 3. KOMBINIRANO ZAPOREDJE
    // ============================================================

    println!("\n=== Kombinirano zaporedje ===");
    // Izraz: a + 2 * g
    // a = aritmetično (1, 4, 7, 10, ...), g = geometrijsko (2, 6, 18, 54, ...)
    // Combined: 1+2*2=5, 4+2*6=16, 7+2*18=43, 10+2*54=118, ...
    let izraz_combined: Izraz<i64> = Izraz::op(
        Izraz::spr(arit.name()),
        BinOperacija::Plus,
        Izraz::op(
            Izraz::konst(2),
            BinOperacija::Times,
            Izraz::spr(geom.name()),
        ),
    );
    let combined = Combined::new("a + 2*g", &izraz_combined, vec![&arit, &geom]);
    izpisi_zaporedje(&combined, 5);
}
