use pr0t0n_orch_db::new_pool;
fn main() {
    let pool = new_pool();
    let _conn = pool.get().unwrap();
}
