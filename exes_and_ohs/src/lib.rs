pub fn xo(string: &'static str) -> bool {
    let mut xos = string
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| *c == 'o' || *c == 'x')
        .collect::<Vec<char>>();
    xos.sort();
    xos.dedup();

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
      assert_eq!(xo("xo"), true);
      assert_eq!(xo("Xo"), true);
      assert_eq!(xo("xxOo"), true);
      assert_eq!(xo("xxxm"), false);
      assert_eq!(xo("Oo"), false);
      assert_eq!(xo("ooom"), false);
    }
}
