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
                // TODO: Investigate what next.insert() does
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
                match next.child.get_mut(&c)
                {
                    Some(x) => next = x,
                    // This is where we go to start suggesting/correcting
                    None => break,
                }
            }
        }
    }
}

pub fn main()
{
}
