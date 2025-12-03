pub fn hash_multi(data: &[impl AsRef<[u8]>]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();

    for d in data {
        hasher.update(d.as_ref());
    }

    hasher.finalize().into()
}
