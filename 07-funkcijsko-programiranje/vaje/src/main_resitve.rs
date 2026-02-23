// Napišite različne funkcije, ki kot argument sprejmejo zaprtje (closure) in ga pokličejo z različnimi argumenti, ki ustrezajo sledečim ocaml tipom:
// Tip zaprtja naj bo čim bolj splošen (Fn, FnOnce, FnMut).
//  apply_int: (int -> int) -> int -> int
//  apply: ('a -> 'b) -> 'a -> 'b
//  apply2: ('a -> 'a -> 'b) -> 'a -> 'a -> 'b
//  map: ('a -> 'b) -> 'a list -> 'b list  (Uporabite Vec<T> namesto list)
//  ponavljaj: int -> ('a -> 'a) -> 'a -> 'a // Ponovi funkcijo n-krat
//  filter: ('a -> bool) -> 'a list -> 'a list // Vrne seznam elementov, ki zadoščajo pogoju - uporabite Vec<T> namesto list in že vgrajeno funkcijo filter

// apply_int: (int -> int) -> int -> int
// Za funkcijo bomo vzeli vse možnosti in vsako od njih preverili z različnimi argumenti

// 1. Najbolj splošen način je uporaba traita Fn, take funkcije ne smejo niti spremeniti okolja, niti prevzeti lastništva
// To je primerljivo če na okolje gledamo kot na dodatni argument tipa &T

// Uporabimo &dyn -> torej argument f kaže na na nekaj, kar lahko uporabimo kot funkcijo
fn apply_int_dyn(f: &dyn Fn(i64) -> i64, x: i64) -> i64 {
    return f(x);
}

fn use_dynamic() {
    let add_one = |n: i64| n + 1;
    let multiply_by_two = |n: i64| n * 2;

    // Ker dve zaprtji NIKOLI nimata istega tipa uporabimo &dyn za tip vektorja in vzamemo referenco na to, da implementirata Fn
    let functions: Vec<&dyn Fn(i64) -> i64> = vec![&add_one, &multiply_by_two];

    for f in functions {
        println!("{}", apply_int_dyn(f, 10)); // Calls dynamically
    }
}
// Vse spodnje implementacije so enake, le da jih zapišemo na različne načine
// Za razliko od &dyn tukaj sprejmemo nekaj kar implementira Fn (torej ne kazalca) in se lahko pokliče
fn apply_int_impl(f: impl Fn(i64) -> i64, x: i64) -> i64 {
    return f(x);
}
pub fn apply_int_param_1<F: Fn(i64) -> i64>(f: F, x: i64) -> i64 {
    f(x)
}
pub fn apply_int_param_2<F>(f: F, x: i64) -> i64
where
    F: Fn(i64) -> i64,
{
    f(x)
}

// To dela ker: https://doc.rust-lang.org/reference/type-coercions.html#r-coerce.unsize.trait-object
fn use_impl() {
    let add_one = |n: i64| n + 1;
    let multiply_by_two = |n: i64| n * 2;

    // Naredimo seznam, kjer so to strukture, ki implementirajo trait Fn
    let functions = vec![add_one, multiply_by_two];
    // Ali pa naredimo seznam, kjer so to kazalci na strukture, ki implementirajo trait Fn
    let functions2: Vec<&dyn Fn(i64) -> i64> = vec![&add_one, &multiply_by_two];

    for f in functions {
        println!("{}", apply_int_impl(f, 10));
    }

    // To dela ker: https://doc.rust-lang.org/reference/type-coercions.html#r-coerce.unsize.trait-object
    for f in functions2 {
        println!("{}", apply_int_dyn(f, 10)); // Pokliče s kazalce na objekt, ki pa samo ovije trait
    }
}

fn fn_is_too_restrictive() {
    let mut number_of_calls = 0;
    let add_one = |n: i64| {
        number_of_calls += 1; // To sedaj zadevo naredi FnMut
        n + 1
    };
    // Katerakoli od spodnjih vrstic ne bo delovala
    // println!("{}", apply_int_dyn(&add_one, 10));
    // println!("{}", apply_int_impl(add_one, 10));
}

fn fn_is_too_restricitve2() {
    let s = String::from("Consumed");
    let mut dummy = vec![];
    let return_and_move = |x: i64| {
        // Niz `s` (ki je na kopici) je sedaj premaknjen v to okolje in nad njim prevzamemo lastništvo - to se lahko zgodi samo enkrat.
        dummy.push(s);
        // drop(s); // To bi bilo ekvivalentno da dosežemo, da je to FnOnce (ampak popolnoma neuporabno)
        return x;
    };
    // println!("{}", apply_int_dyn(&return_and_move, 10));
    // println!("{}", apply_int_impl(return_and_move, 10));
}

// V zgornjih funkcijah smo imeli težave, saj smo "preveč" omejili tip fukncije, ki ga lahko sprejmemo
// V resnici je za to, da funkcijo pokličemo v `apply_int` dovolj, da impelmentira trait FnOnce (ki ga je lažje implementirati)
// Torej lahko uporabimo enako definicijo, le da smo sedaj bolj fleksibilni pri tem, kaj lahko sprejmemo
fn apply_int_once(f: impl FnOnce(i64) -> i64, x: i64) -> i64 {
    return f(x);
}

// Dinamična oblika je nekoliko bolj zapletena, saj moramo uporabiti kazalec na funkcijo, ker FnOnce prevzame lastništvo nad sabo in tega traiti ne dovolijo
// Tudi uporaba se nekoliko zaplete
fn apply_int_once_dyn(f: Box<dyn FnOnce(i64) -> i64>, x: i64) -> i64 {
    return f(x);
}

// Če naredimo to za FnMut moramo še eksplicitno povedati da je f mutable, saj potrebuje &mut self pri klicu (pri FnOnce tega ne potrebujemo, saj prevzame lastništvo in s tem implicitno tudi to)
fn apply_int_mut(mut f: impl FnMut(i64) -> i64, x: i64) -> i64 {
    return f(x);
}

// Sedaj bi lahko uporabili tudi zgornja dva primera, ki sta bila preveč omejena

// Vsak FnMut je tudi FnOnce (v drugo smer pa ne drži)
fn fn_is_too_restrictive_not_anymore() {
    let mut number_of_calls = 0;
    let add_one = |n: i64| {
        number_of_calls += 1; // To sedaj zadevo naredi FnMut
        n + 1
    };
    // Seveda je sedaj ne smemo poklicati dvakrat
    println!("{}", apply_int_once(add_one, 10));
    // println!("{}", apply_int_once(add_one, 10)); // To ne gre več
}

fn fn_is_too_restricitve2_not_anymore() {
    let s = String::from("Consumed");
    let mut dummy = vec![];
    let return_and_move = |x: i64| {
        // Niz `s` (ki je na kopici) je sedaj premaknjen v to okolje in nad njim prevzamemo lastništvo - to se lahko zgodi samo enkrat.
        dummy.push(s);
        // drop(s); // To bi bilo ekvivalentno da dosežemo, da je to FnOnce (ampak popolnoma neuporabno)
        return x;
    };
    println!("{}", apply_int_once(return_and_move, 10));
    // println!("{}", apply_int_once(return_and_move, 10));// To ne gre več
}

// uporaba za dyn je sedaj nekoliko bolj zapletena -
fn fn_is_too_restrictive_not_anymore_box_problem() {
    let s = String::from("Consumed");
    use std::sync::{Arc, Mutex};

    let dummy = Arc::new(Mutex::new(vec![]));
    let return_and_move = {
        let dummy = Arc::clone(&dummy);
        move |x: i64| {
            let mut dummy = dummy.lock().unwrap();
            dummy.push(s);
            return x;
        }
    };
    // Seveda je sedaj ne smemo poklicati dvakrat
    println!("{}", apply_int_once_dyn(Box::new(return_and_move), 10));
    println!("Final: {:?}", dummy.lock().unwrap());
}

// 2. Za poljuben T je to podobno

fn apply_dyn<T>(f: &dyn Fn(T) -> T, x: T) -> T {
    return f(x);
}

fn apply_impl<T>(f: impl Fn(T) -> T, x: T) -> T {
    return f(x);
}

fn apply<T, F>(f: F, x: T) -> T
where
    F: Fn(T) -> T,
{
    return f(x);
}

// In še FnOnce
fn apply_dynOnce(f: Box<dyn FnOnce(i64) -> i64>, x: i64) -> i64 {
    return f(x);
}

fn apply_traitOnce<T>(f: impl FnOnce(T) -> T, x: T) -> T {
    return f(x);
}

// 3. Za dva poljubna T je to podobno

fn apply2_dyn<T>(f: &dyn Fn(T, T) -> T, x: T, y: T) -> T {
    return f(x, y);
}

fn apply2_impl<T>(f: impl Fn(T, T) -> T, x: T, y: T) -> T {
    return f(x, y);
}

fn apply2_impl_borrow<T>(f: impl Fn(&T, &T) -> T, x: &T, y: &T) -> T {
    return f(x, y);
}

// Seveda se moramo vedno zavedati, da je lahko problematično, če funkcija prevzame lastništvo nad argumenti -

fn use_apply() {
    let len_concat = |a: String, b: String| (a.len() + b.len()).to_string();
    let s1 = "Hello".to_string();
    let s2 = "World".to_string();
    println!("{}", apply2_impl(len_concat, s1, s2));
    // println!("{}", s1); // To ne gre

    let len_concat = |a: &String, b: &String| (a.len() + b.len()).to_string();
    let s3 = "Hello".to_string();
    let s4 = "World".to_string();
    println!("{}", apply2_impl_borrow(len_concat, &s3, &s4));
    println!("{}", s3); // To gre, ker gre za izposojo s3 (in tudi s4)
    println!("{}", s4); // To gre, ker gre za izposojo s3 (in tudi s4)
}

// 4.
// Seveda deluje za Fn, ampak FnMut dovoljuje več fleksibilnosti, zato implementiramo samo to
fn map_std_mut<T, U, F>(f: F, list: &Vec<T>) -> Vec<U>
where
    F: FnMut(&T) -> U,
{
    return list.iter().map(f).collect();
}

fn map_hand_mut<T, U, F>(mut f: F, list: Vec<T>) -> Vec<U>
where
    F: FnMut(&T) -> U,
{
    let mut result = vec![];
    for x in list.iter() {
        result.push(f(x));
    }
    return result;
}

// 5.

// Lahko imamo

fn ponavljaj<T>(n: i32, mut f: impl FnMut(T) -> T, x: T) -> T {
    let mut result = x;
    for _ in 0..n {
        result = f(result);
    }
    return result;
}

// Ne deluje, ker klic na FnOnce prevzame lastništvo nad f in ga ne moremo več uporabiti
// fn ponavljajOnce<T>(n: i32, f: impl FnOnce(T) -> T, x: T) -> T {
//     let mut result = x;
//     for _ in 0..n {
//         result = f(result);
//     }
//     return result;
// }

fn main() {
    // use_dynamic();
    // use_impl();
    // fn_is_too_restrictive_not_anymore_box_problem();
    // use_apply();
    ponavljaj(10, |x| x + 1, 0);
    let s = "Hello".to_string();

    let add_dot = |s: String| {
        println!("Processing: {}", s);
        return s + "."; // Just return the same reference (no modification)
    };
    println!("{}", ponavljaj(10, add_dot, s));
}

// Iteratorji

// Napišite funkcijo, ki sprejme vektor XYZ in s pomočjo iteratorja naredi W za sledeče naloge:
// števil in izpiše vsako v svojo vrstico
// nizov in izpiše njihove dolžine
// nizov in vrne vsoto njihovih dolžin
// vektor parov (i32, i32) in vrne vsoto njihovih pozitivnih produktov
// dva vektorja <i32> in vrne vektor, ki vsebuje vsote parov
// dva vektorja <i32> in vrne vsoto poparjenih pozitivni produktov
// vektor Option<T> in izpiše vse T-je
// vektor Option<T> in vrne število Some-ov
// odfiltrira števila deljiva s 3

fn print_xyz(v: Vec<i32>) {
    v.iter().for_each(|x| println!("{}", x));
}

fn print_lengths(v: Vec<String>) {
    v.iter().for_each(|x| println!("{}", x.len()));
}

fn sum_lengths(v: Vec<String>) -> usize {
    return v.iter().map(|x| x.len()).sum();
}

fn sum_positive_products(v: Vec<(i32, i32)>) -> i32 {
    return v.iter().map(|(a, b)| a * b).filter(|x| x > &0).sum();
}

fn sum_pairs(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    return v1.iter().zip(v2.iter()).map(|(a, b)| a + b).collect();
}

fn sum_positive_products2(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    return sum_positive_products(v1.iter().zip(v2.iter()).map(|(a, b)| (*a, *b)).collect());
}

use std::fmt::{format, Display};

fn print_options<T: Display>(v: Vec<Option<T>>) {
    v.iter().for_each(|x| match x {
        Some(x) => println!("{}", x),
        None => (),
    });
}

fn count_options<T>(v: Vec<Option<T>>) -> i32 {
    return v.iter().filter(|x| x.is_some()).count() as i32;
}

fn filter_out_divisible_by_3(v: Vec<i64>) -> Vec<i64> {
    return v
        .iter()
        .filter(|&&x| x % 3 == 0) // Fuj, ker imamo referenco, leše bi bilo z into_iter
        .map(|x| {
            return *x;
        })
        .collect();
}

// Dopolnite spodnjo funkcijo, da vrne niz, kjer so vse prve črke posameznih besed velike
// ["Just,", " ", "hello", " ", "world", "!"] -> "Just, Hello World", "!"
pub fn capitalize_words_string(words: &[&str]) -> String {
    return words
        .iter()
        .map(|x| {
            let mut chars = x.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ");
}
// Napišite funkcijo `fakulteta`, ki izračuna fakulteto števila n. Uporabite iteratorje (torej brez lastne for zanke, rekurzije)
// Namig: fold, reduce, `..`...

pub fn fakulteta(n: u64) -> u64 {
    return (1..=n).fold(1, |acc, x| acc * x);
}

// -------------------------------------------------------------------------------------------------
// Dodatno:
// Koda vzeta iz googlvih rust vaj:
// Vse se da lepo narediti samo z iteratorji (brez indeksov, brez for zank, brez mutabilnosti)

/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences<N>(offset: usize, values: Vec<N>) -> Vec<N>
where
    N: Copy + std::ops::Sub<Output = N>,
{
    let a = values.iter();
    let b = values.iter().cycle().skip(offset);
    return a.zip(b).map(|(a, b)| *b - *a).collect();
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_custom_type() {
    assert_eq!(
        offset_differences(1, vec![1.0, 11.0, 5.0, 0.0]),
        vec![10.0, -6.0, -5.0, 1.0]
    );
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}
