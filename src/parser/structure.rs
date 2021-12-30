
#[derive(Debug)]
pub(crate) struct Structure {
    layout: Vec<Structure>,
    pub(crate) size: u16,
    pub(crate) align: u8,
}

impl Structure {

    pub(crate) fn _custom<const SIZE: usize>(layout: [Structure; SIZE]) -> Self {
        
        // calculate size and alignment depending on the other structures
        let (size, align) = 
            layout
                .iter()
                .map(|a| (a.size, a.align))
                .reduce(|a, b| match (a.0 >= b.0, a.1 >= b.1) {
                    (true, true) => (a.0, a.1),
                    (true, false) => (a.0, b.1),
                    (false, true) => (b.0, a.1),
                    (false, false) => (b.0, b.1),
                })
                .unwrap_or((0, 2));

        Self {
            layout: Vec::from(layout),
            size,
            align,
        }
    }

    pub(crate) fn primitive(size: u16, align: u8) -> Self {
        Self {
            layout: Vec::new(),
            size,
            align,
        }
    }

}

