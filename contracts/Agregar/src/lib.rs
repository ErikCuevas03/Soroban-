#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Vec, Map, symbol_short};

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {
    pub fn add_product(env: Env, name: String, quantity: i32, price: i32) {
        let storage_key = symbol_short!("PRODUCTS");
        let mut products: Map<String, Vec<i32>> = env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env));

        if products.contains_key(name.clone()) {
            panic!("Producto ya existe");
        }

        products.set(name, Vec::from_array(&env, [quantity, price]));
        env.storage().persistent().set(&storage_key, &products);
    }

    pub fn get_product(env: Env, name: String) -> Vec<i32> {
        let storage_key = symbol_short!("PRODUCTS");
        let products: Map<String, Vec<i32>> = env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env));

        products.get(name).unwrap_or_else(|| Vec::new(&env))
    }

    pub fn update_product(env: Env, name: String, new_quantity: i32, new_price: i32) {
        let storage_key = symbol_short!("PRODUCTS");
        let mut products: Map<String, Vec<i32>> = env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env));

        if !products.contains_key(name.clone()) {
            panic!("Producto no encontrado");
        }

        products.set(name, Vec::from_array(&env, [new_quantity, new_price]));
        env.storage().persistent().set(&storage_key, &products);
    }

    pub fn delete_product(env: Env, name: String) {
        let storage_key = symbol_short!("PRODUCTS");
        let mut products: Map<String, Vec<i32>> = env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env));

        if !products.contains_key(name.clone()) {
            panic!("Producto no encontrado");
        }

        products.remove(name);
        env.storage().persistent().set(&storage_key, &products);
    }

    pub fn list_products(env: Env) -> Map<String, Vec<i32>> {
        let storage_key = symbol_short!("PRODUCTS");
        env
            .storage()
            .persistent()
            .get(&storage_key)
            .unwrap_or(Map::new(&env))
    }
}