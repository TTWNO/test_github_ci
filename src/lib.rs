#![deny(
  clippy::all,
  //clippy::pedantic,
  //unsafe_code,
  //missing_docs,
)]

// //! My library is to do simple math and show Github CI

// /// We need documentation to make `missing_docs` work
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// TODO: fix clippy error
// local (not public) function which is not used
//fn mult(left: usize, right: usize) -> usize {
//  left * right
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn lol() {
      panic!("Oh no!");
    }
}
