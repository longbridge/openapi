use longport_proto::quote::SubType;

bitflags::bitflags! {
    /// Subscription flags
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SubFlags: u8 {
        /// Quote
        const QUOTE = 0x1;

        /// Depth
        const DEPTH = 0x2;

        /// Broker
        const BROKER = 0x4;

        /// Trade
        const TRADE = 0x8;
    }
}

impl From<SubFlags> for Vec<i32> {
    fn from(flags: SubFlags) -> Self {
        let mut sub_types = Vec::new();

        if flags.contains(SubFlags::QUOTE) {
            sub_types.push(SubType::Quote.into());
        }

        if flags.contains(SubFlags::DEPTH) {
            sub_types.push(SubType::Depth.into());
        }

        if flags.contains(SubFlags::BROKER) {
            sub_types.push(SubType::Brokers.into());
        }

        if flags.contains(SubFlags::TRADE) {
            sub_types.push(SubType::Trade.into());
        }

        sub_types
    }
}

impl From<Vec<i32>> for SubFlags {
    fn from(sub_types: Vec<i32>) -> Self {
        let mut flags = SubFlags::empty();

        for sub_type in sub_types {
            if let Some(sub_type) = SubType::from_i32(sub_type) {
                match sub_type {
                    SubType::Quote => flags |= SubFlags::QUOTE,
                    SubType::Depth => flags |= SubFlags::DEPTH,
                    SubType::Brokers => flags |= SubFlags::BROKER,
                    SubType::Trade => flags |= SubFlags::TRADE,
                    _ => {}
                }
            }
        }

        flags
    }
}
