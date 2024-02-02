use ace::AceTrie;

pub mod ace
{
    use std::{collections::HashMap, hash::Hash};

    pub struct KeyWeights
    {
        /// The letters should be already ordered from
        /// heaviest to lightest so we can just
        /// loop through it, we can make a
        /// we can make this an array later on 
        /// but for now it should be a vector
        weight: HashMap<char, Vec<u8>>,
    }

    enum KeyboardRow
    {
        R1,
        R2,
        R3,
    }
    impl KeyWeights
    {
        pub fn new() -> Self
        {
            KeyWeights { weight: HashMap::new() }
        }
    }

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
        pub fn default() -> Self { AceTrie::new() }
        pub fn insert(
            &mut self,
            s: String,
        )
        {
            let mut next = self;
            for c in s.chars()
            {
                next.child.insert(c, AceTrie::default());
                next = match next.child.get_mut(&c)
                {
                    Some(x) => x,
                    // idk why it wouldnt insert
                    None => break,
                }
            }
        }
        pub fn search(
            &mut self,
            s: String,
        )
        {
            let mut next = self;
            for c in s.chars()
            {
                match next.child.contains_key(&c)
                {
                    true =>
                    {
                        next = next
                            .child
                            .get_mut(&c)
                            .expect("Somehow key wasnt matched?")
                    }
                    false => next.suggest(),
                }
            }
        }
        /// NOTE: The implementation FOR NOW will be terrible
        /// Im going to loop through the alphabet and hope for a match
        /// Later on I will actually use my idea of assigning weight to keys
        /// and searching based on weight first
        fn suggest(&mut self)
        {
            for x in 'a'..='z'
            {
                match self.child.contains_key(&x)
                {
                    true =>
                    {
                        println!("{} found", x);
                        break;
                    }
                    false => println!("{}", x),
                }
            }
        }
        // TODO: A function that explores the tree literally
        pub fn explore(&mut self)
        {
        }
    }
}

pub fn main()
{
}
