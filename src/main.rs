pub mod ace
{
    use std::collections::HashMap;

    pub struct AceTrie
    {
        child: HashMap<char, AceTrie>,
    }
    impl AceTrie
    {
        pub fn new() -> Self
        {
           AceTrie {
            child: HashMap::new(),
           } 
        }
    }
}

pub fn main()
{
}
