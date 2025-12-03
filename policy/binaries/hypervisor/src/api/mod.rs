use axum::Router;

pub mod agent;
pub mod encrypt;
pub mod openai;
pub mod ping;

pub trait ServerState: Clone + Sync + Send + 'static {}
impl<T: Clone + Sync + Send + 'static> ServerState for T {}

pub trait RouterRegister<V> {
    fn register_api<T>(self, _: impl Fn(Router<V>) -> Router<T>) -> Router<T>;
}

impl<V> RouterRegister<V> for Router<V> {
    fn register_api<T>(self, f: impl Fn(Router<V>) -> Router<T>) -> Router<T> {
        f(self)
    }
}
