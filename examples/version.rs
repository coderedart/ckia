fn main() {
    let milestone = ckia::SkiaVersion::get_milestone();

    let increment = ckia::SkiaVersion::get_increment();

    let version_str = ckia::SkiaVersion::get_string();
    println!("milestone: {milestone}");
    println!("increment: {increment}");
    println!("version_str: {version_str}");
}
