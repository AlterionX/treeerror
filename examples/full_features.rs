treeerror::treeerror! {
    #[derive(Debug)]
    Hello {
        #[derive(Debug)]
        World @unit,
        FlatWorld @flatunit,
        #[derive(Debug)]
        OtherWorld {
            #[derive(Debug)]
            W0 @unit,
            #[derive(Debug)]
            W1 @unit,
            #[derive(Debug)]
            W2 @unit,
            #[derive(Debug)]
            W3 @flatunit,
        },
        Terminal(String),
        LifetimeTerminal(&'static str),
        #[derive(Debug)]
        Test {
            A @flatunit,
        },
        #[derive(Debug)]
        Depth0 {
            #[derive(Debug)]
            Depth1 {
                #[derive(Debug)]
                Depth2 {
                    #[derive(Debug)]
                    Depth3 {
                        #[derive(Debug)]
                        Depth4 {
                            #[derive(Debug)]
                            Depth5 {
                                #[derive(Debug)]
                                Depth6 @unit
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn main() {
}
