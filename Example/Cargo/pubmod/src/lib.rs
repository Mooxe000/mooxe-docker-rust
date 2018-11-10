pub mod network;
pub mod client;

#[cfg(test)]
mod tests {

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn connect_network() {
    use network::*;
    connect();
  }

  #[test]
  fn connect_client() {
    use client::*;
    connect();
  }
}
