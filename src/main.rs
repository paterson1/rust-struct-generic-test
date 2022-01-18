use std::cmp::Ordering;

fn main() {

    #[derive(Eq, PartialEq, Debug)]
    struct Unsigned {
        pub timestamp: u32,
        val: u8
    }

    #[derive(Eq, PartialEq, Debug)]
    struct Integer {
        pub timestamp: u32,
        val: i32
    }

    #[derive(Eq, PartialEq, Debug)]
    struct Boolean {
        pub timestamp: u32,
        val: bool
    }



    #[derive(Eq, PartialEq, Debug)]
    enum Update {
        UnsignedUpdate(Unsigned),
        IntegerUpdate(Integer),
        BooleanUpdate(Boolean)
    }



    let i1 = Update::IntegerUpdate(Integer {
        timestamp: 3,
        val: -1
    });

    let i2 = Update::IntegerUpdate( Integer {
        timestamp: 3,
        val: -13
    });

    let u1 = Update::UnsignedUpdate( Unsigned {
        timestamp: 5,
        val: 1
    });

    let u2 = Update::UnsignedUpdate( Unsigned {
        timestamp: 300,
        val: 2
    });

    let b1 = Update::BooleanUpdate( Boolean {
        timestamp: 10,
            val: true
    });

    let b2 = Update::BooleanUpdate( Boolean {
        timestamp: 1,
        val: false
    });

    let mut vs: Vec<Update> = vec![i1,i2,b1,b2,u1,u2];
    println!("{:?}", vs);
    vs.sort_by(|a, b|  
        match a {
            Update::IntegerUpdate(a) => {
                match b  {
                    Update::IntegerUpdate(b)  => {
                        a.timestamp.cmp(&b.timestamp)
                    },
                    Update::BooleanUpdate(b)  => {
                        a.timestamp.cmp(&b.timestamp)
                    },
                    Update::UnsignedUpdate(b)  => {
                        a.timestamp.cmp(&b.timestamp)
                    },
                    
                }
            },
            Update::BooleanUpdate(a) => {
                match b  {
                    Update::IntegerUpdate(b)  => {
                        a.timestamp.cmp(&b.timestamp)
                    },
                    Update::BooleanUpdate(b)  => {
                        a.timestamp.cmp(&b.timestamp)
                    },
                    Update::UnsignedUpdate(b)  => {
                        a.timestamp.cmp(&b.timestamp)
                    },
                    
                }
            },
            Update::UnsignedUpdate(a) => {
                match b  {
                    Update::IntegerUpdate(b)  => {
                        a.timestamp.cmp(&b.timestamp)
                    },
                    Update::BooleanUpdate(b)  => {
                        a.timestamp.cmp(&b.timestamp)
                    },
                    Update::UnsignedUpdate(b)  => {
                        a.timestamp.cmp(&b.timestamp)
                    },
                    
                }
            }
        }
    );
        
    println!("{:?}", vs);

}
