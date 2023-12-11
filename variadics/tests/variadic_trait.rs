use variadics::variadic_trait2;

variadic_trait2! {
    /// A variadic list of `Debug` items.
    pub variadic<Item> DebugList where Item: std::fmt::Display {
        fn to_strings(item: Item) -> String {
            item.to_string()
        }
    }
}
