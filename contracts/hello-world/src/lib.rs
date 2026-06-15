#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env,
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Balance(Address),
}

#[contract]
pub struct CampusLoyaltyToken;

#[contractimpl]
impl CampusLoyaltyToken {

    // Khởi tạo admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // Cấp token cho sinh viên khi tham gia lớp học
    pub fn reward_attendance(
        env: Env,
        admin: Address,
        student: Address,
        amount: i128,
    ) {
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        admin.require_auth();

        if admin != stored_admin {
            panic!("Unauthorized");
        }

        let key = DataKey::Balance(student.clone());

        let current_balance: i128 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&key, &(current_balance + amount));
    }

    // Xem số token của sinh viên
    pub fn balance(env: Env, student: Address) -> i128 {
        let key = DataKey::Balance(student);

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(0)
    }

    // Đổi token lấy quyền lợi
    pub fn redeem(
        env: Env,
        student: Address,
        amount: i128,
    ) {
        student.require_auth();

        let key = DataKey::Balance(student.clone());

        let balance: i128 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(0);

        if balance < amount {
            panic!("Insufficient tokens");
        }

        env.storage()
            .persistent()
            .set(&key, &(balance - amount));
    }
}
mod test;