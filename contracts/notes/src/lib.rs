#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data produk (pengganti Note)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Product {
    id: u64,
    name: String,
    price: u64,
    stock: u64,
}

// Storage key untuk data produk
const PRODUCT_DATA: Symbol = symbol_short!("PRODUCT");

#[contract]
pub struct ShoppingContract;

#[contractimpl]
impl ShoppingContract {

    // Ambil semua produk
    pub fn get_products(env: Env) -> Vec<Product> {
        return env.storage().instance().get(&PRODUCT_DATA).unwrap_or(Vec::new(&env));
    }

    // Tambah produk baru
    pub fn add_product(env: Env, name: String, price: u64, stock: u64) -> String {
        let mut products: Vec<Product> =
            env.storage().instance().get(&PRODUCT_DATA).unwrap_or(Vec::new(&env));

        let product = Product {
            id: env.prng().gen::<u64>(),
            name: name,
            price: price,
            stock: stock,
        };

        products.push_back(product);

        env.storage().instance().set(&PRODUCT_DATA, &products);

        return String::from_str(&env, "Produk berhasil ditambahkan");
    }

    // Hapus produk berdasarkan id
    pub fn delete_product(env: Env, id: u64) -> String {
        let mut products: Vec<Product> =
            env.storage().instance().get(&PRODUCT_DATA).unwrap_or(Vec::new(&env));

        for i in 0..products.len() {
            if products.get(i).unwrap().id == id {
                products.remove(i);
                env.storage().instance().set(&PRODUCT_DATA, &products);

                return String::from_str(&env, "Produk berhasil dihapus");
            }
        }

        return String::from_str(&env, "Produk tidak ditemukan");
    }

    // Fungsi beli produk (mengurangi stock)
    pub fn buy_product(env: Env, id: u64, qty: u64) -> String {
        let mut products: Vec<Product> =
            env.storage().instance().get(&PRODUCT_DATA).unwrap_or(Vec::new(&env));

        for i in 0..products.len() {
            let mut product = products.get(i).unwrap();

            if product.id == id {
                if product.stock < qty {
                    return String::from_str(&env, "Stok tidak cukup");
                }

                product.stock -= qty;
                products.set(i, product);

                env.storage().instance().set(&PRODUCT_DATA, &products);

                return String::from_str(&env, "Pembelian berhasil");
            }
        }

        return String::from_str(&env, "Produk tidak ditemukan");
    }
}

mod test;