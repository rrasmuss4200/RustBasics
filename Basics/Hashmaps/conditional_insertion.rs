//ex from rustlings
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}
fn main() {
    fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
        let fruit_kinds = vec![
            Fruit::Apple,
            Fruit::Banana,
            Fruit::Mango,
            Fruit::Lychee,
            Fruit::Pineapple,
        ];

        for fruit in fruit_kinds {
            //this will only insert the fruit if it is not already present
            //it will insert it with a default value of 5
            basket.entry(fruit).or_insert(5);
        }
    }
}