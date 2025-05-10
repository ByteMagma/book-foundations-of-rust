use generics_demo_four::KVEntry;

fn main() {
  let session = KVEntry {
    key: 1001,
    value: "user123",
  };

  let setting = KVEntry {
      key: "theme",
      value: "dark",
  };

  let cache = KVEntry {
      key: ("user_id", 42),
      value: vec!["file1.txt", "file2.txt"],
  };
}
