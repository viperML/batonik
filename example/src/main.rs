use batonik;

fn main() {
    batonik::App::new()
        .add(async {
            return String::from("Hello");
        })
        .run();
}
