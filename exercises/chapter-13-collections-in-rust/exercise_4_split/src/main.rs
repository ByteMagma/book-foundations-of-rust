fn main() {
    let countries = "United States,Canada,Mexico,Brazil,Argentina";

    for country in countries.split(',') {
        println!("{}", country);
    }
}
