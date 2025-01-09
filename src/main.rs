use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Selamat datang di game tebak angka!");
    println!("Siapa namanya bro?");

    let mut nama = String::new();
    let result = io::stdin().read_line(&mut nama);
    match result {
        Ok(_) => println!("Halo {}, silahkan pilih angka dari 1 smpe 100", nama.trim()),
        Err(_) => println!("Gagal membaca input user"),
    };

    let angka_random: u32 = rand::thread_rng().gen_range(0..100);
    let mut jumlah_percobaan: u32 = 0;

    loop {
        let mut input_user = String::new();
        io::stdin().read_line(&mut input_user).expect("gagal membaca input user");

        let input: u32 = match input_user.trim().parse() {
            Ok(angka) => angka,
            Err(_) => {
                println!("Masukan angka yang benr bro");
                continue;
            }
        };

        println!("angka tebakan lu : {}", input);

        match input.cmp(&angka_random){
            Ordering::Less => {
                jumlah_percobaan+=1;
                println!("Terlalu kecil, gedein lagi");
            },
            Ordering::Greater => {
                jumlah_percobaan+=1;
                println!("Terlalu besar, kecilin lagi");
            },
            Ordering::Equal => {
                jumlah_percobaan+=1;
                println!("CONGRATSS! Tebakan lu bener dalam {} kali percobaan", jumlah_percobaan);
                break;
            }
        }
    }
}
