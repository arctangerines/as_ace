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
        pub fn default() -> Self { AceTrie::new() }
        pub fn insert(
            &mut self,
            s: String,
        )
        {
            let mut next = self;
            for c in s.chars()
            {
                // NOTE: I... I was calling this function itself *facepalm*
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
            for x in b'a'..=b'z'
            {
                println!("{}", x as char);
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
