// The first submodule "front_of_house::hosting"
// "pub" here means anything with access to "front_of_house" can access "hosting"
// Here we've put "hosting" into its own file, under the dedicated folder "src/front_of_house/"
pub mod hosting;

// The second submodule "front_of_house::serving"
// This is a "sister" module to "hosting"
mod serving {
    // Some functions
    fn _take_order() {}

    fn _serve_order() {}

    fn _take_payment() {}
}