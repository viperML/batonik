use batonik::batonik;

fn main() {
    let b = batonik![
        "Hello",
        async { format!("World") },
    ];
}
