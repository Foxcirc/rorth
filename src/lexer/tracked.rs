
pub(crate) struct Tracked<I: Iterator> {
    iter: I,
    pos: usize,
}

impl<I: Iterator> Tracked<I> {
    
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            pos: 0,
        }
    }

    #[inline(always)]
    pub(crate) fn pos(&self) -> usize {
        self.pos
    }

}

impl<I: Iterator> Iterator for Tracked<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;        
        self.iter.next()
    }
}

pub(crate) trait IntoTracked<I: Iterator> {
    fn track(self) -> Tracked<I>;
}

impl<I: Iterator> IntoTracked<I> for I {
    fn track(self) -> Tracked<I> {
        Tracked::new(self)
    }
}
