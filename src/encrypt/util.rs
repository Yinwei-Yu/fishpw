use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use std::num::{NonZeroIsize, NonZeroU32};

const ARGON2_MEMORY: NonZeroU32 = NonZeroU32::new(1024 * 1024 * 2).unwrap(); // 2MB
const ARGON2_ITERATIONS: NonZeroU32 = NonZeroU32::new(4).unwrap(); // 4次迭代
const ARGON2_PARALLELISM:NonZeroU32 = NonZeroU32::new(4).unwrap(); // 4个并行线程进行加密

pub fn encrypt_string_with_argon2(password: &str, plaintext: &str) -> Vec<u8> {
    // 1. 生成随机盐 (Salt)
    let salt = SaltString::generate(&mut OsRng);

    // 2. 使用 Argon2 进行密钥派生 (Key Derivation)
    // 这里我们使用推荐的 Argon2id 变种，并设置了自定义的内存和迭代参数
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(
            ARGON2_MEMORY.get(),
            ARGON2_ITERATIONS.get(),
            ARGON2_PARALLELISM.get(),
            Some(32),
        )
        .unwrap(),
    );
    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    let hash_bytes = hash.hash.unwrap();
    let key_bytes = hash_bytes.as_bytes();

    // 3. 生成随机初始化向量 (IV/Nonce)
    // AES-GCM 使用的 Nonce 为 12 字节
    let mut iv_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut iv_bytes);
    let iv = Nonce::from_slice(&iv_bytes);

    // 4. 使用 AES-GCM 进行加密
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(&key);
    let ciphertext = cipher.encrypt(iv, plaintext.as_bytes()).expect("加密失败");

    // 5. 组装并返回加密结果
    // 为了方便存储和解密，我们将 salt、iv 和密文拼接在一起
    // 格式：[salt_length][salt][iv_length][iv][ciphertext]
    let mut result_vec = Vec::new();
    result_vec.extend_from_slice(&salt.as_str().as_bytes().len().to_le_bytes()); // 盐长度
    result_vec.extend_from_slice(salt.as_str().as_bytes()); // 盐
    result_vec.extend_from_slice(&iv_bytes.len().to_le_bytes()); // IV 长度
    result_vec.extend_from_slice(&iv_bytes); // IV
    result_vec.extend_from_slice(&ciphertext); // 密文

    result_vec
}
