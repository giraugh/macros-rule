#[allow(unused, dead_code)]
mod id_without_macros {

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct HouseholdId(usize);

    impl HouseholdId {
        pub fn new(index: usize) -> Self {
            Self(index)
        }

        pub fn index(self) -> usize {
            self.0
        }

        pub fn next(&self) -> Self {
            Self(self.clone().index() + 1)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct FamilyId(usize);

    impl FamilyId {
        pub fn new(index: usize) -> Self {
            Self(index)
        }

        pub fn index(self) -> usize {
            self.0
        }

        pub fn next(&self) -> Self {
            Self(self.clone().index() + 1)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct CastleId(usize);

    impl CastleId {
        pub fn new(index: usize) -> Self {
            Self(index)
        }

        pub fn index(self) -> usize {
            self.0
        }

        pub fn next(&self) -> Self {
            Self(self.clone().index() + 1)
        }
    }
}

#[allow(unused, dead_code)]
mod id_with_macros {

    macro_rules! newtype_id {
        ($struct_name:ident) => {
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
            struct $struct_name(usize);

            impl $struct_name {
                pub fn new(index: usize) -> Self {
                    Self(index)
                }

                pub fn index(self) -> usize {
                    self.0
                }

                pub fn next(&self) -> Self {
                    Self(self.clone().index() + 1)
                }
            }
        };
    }

    newtype_id!(HouseholdId);
    newtype_id!(FamilyId);
    newtype_id!(CastleId);
}
