fn main() {
    struct App {
        id: u32,
        name: String,
    }
    let app = App {
        id: 1,
        name: String::from("hello"),
    };
    let key = "id";
   key.split("").collect::<Vec<_>>();

}