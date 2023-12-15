use std::io;

// Struktur untuk merepresentasikan barang
#[derive(Debug)]
struct Barang {
    nama: String,
    jumlah: u32,
}

// Fungsi utama aplikasi
fn main() {
    // Vektor untuk menyimpan daftar barang
    let mut daftar_barang: Vec<Barang> = Vec::new();

    loop {
        println!("Pilih operasi:");
        println!("1. Tambah Barang");
        println!("2. Lihat Daftar Barang");
        println!("3. Hapus Barang");
        println!("4. Keluar");

        // Baca input dari pengguna
        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).expect("Gagal membaca baris");
        let pilihan: u32 = match pilihan.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Masukkan nomor yang valid!");
                continue;
            }
        };

        match pilihan {
            1 => tambah_barang(&mut daftar_barang),
            2 => lihat_daftar_barang(&daftar_barang),
            3 => hapus_barang(&mut daftar_barang),
            4 => {
                println!("Keluar dari aplikasi.");
                break;
            }
            _ => println!("Pilihan tidak valid!"),
        }
    }
}

// Fungsi untuk menambahkan barang ke daftar
fn tambah_barang(daftar_barang: &mut Vec<Barang>) {
    println!("Masukkan nama barang:");
    let mut nama_barang = String::new();
    io::stdin().read_line(&mut nama_barang).expect("Gagal membaca baris");
    let nama_barang = nama_barang.trim().to_string();

    println!("Masukkan jumlah barang:");
    let mut jumlah_barang = String::new();
    io::stdin().read_line(&mut jumlah_barang).expect("Gagal membaca baris");
    let jumlah_barang: u32 = match jumlah_barang.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Masukkan jumlah yang valid!");
            return;
        }
    };

    // Buat objek Barang dan tambahkan ke vektor
    let barang = Barang {
        nama: nama_barang,
        jumlah: jumlah_barang,
    };
    daftar_barang.push(barang);

    println!("Barang berhasil ditambahkan!");
}

// Fungsi untuk melihat daftar barang
fn lihat_daftar_barang(daftar_barang: &Vec<Barang>) {
    if daftar_barang.is_empty() {
        println!("Daftar barang kosong.");
    } else {
        println!("Daftar Barang:");
        for barang in daftar_barang {
            println!("{:?}", barang);
        }
    }
}

// Fungsi untuk menghapus barang dari daftar berdasarkan nama
fn hapus_barang(daftar_barang: &mut Vec<Barang>) {
    if daftar_barang.is_empty() {
        println!("Daftar barang kosong.");
        return;
    }

    println!("Masukkan nama barang yang akan dihapus:");
    let mut nama_barang = String::new();
    io::stdin().read_line(&mut nama_barang).expect("Gagal membaca baris");
    let nama_barang = nama_barang.trim();

    // Cari indeks barang berdasarkan nama
    if let Some(index) = daftar_barang.iter().position(|barang| barang.nama == nama_barang) {
        // Hapus barang dari vektor
        daftar_barang.remove(index);
        println!("Barang berhasil dihapus!");
    } else {
        println!("Barang tidak ditemukan.");
    }
}
