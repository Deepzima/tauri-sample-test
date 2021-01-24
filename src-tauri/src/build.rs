#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
  if std::path::Path::new("icons/icon.ico").exists() {
    let mut res = winres::WindowsResource::new();
    res.set_icon_with_id("icons/icon.ico", "32512");
    println!("Message from Rust: {}", msg);
    res.compile().expect("Unable to find visual studio tools");
  } else {
    panic!("No Icon.ico found. Please add one or check the path");
  }
}

#[cfg(not(windows))]
fn main() {}
