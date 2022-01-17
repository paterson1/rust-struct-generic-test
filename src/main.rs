use std::cmp::Ordering;

fn main() {

    #[derive(Eq, PartialEq, Debug)]
    struct Unsigned {
        val: u8
    }

    #[derive(Eq, PartialEq, Debug)]
    struct Integer {
        val: i32
    }

    #[derive(Eq, PartialEq, Debug)]
    struct Boolean {
        val: bool
    }


    #[derive(Eq, Debug)]
    struct GenericUpdate<T> {
        pub timestamp: u32,
        data: T
    }


    #[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
    enum Update {
        UnsignedUpdate(GenericUpdate<Unsigned>),
        IntegerUpdate(GenericUpdate<Integer>),
        BooleanUpdate(GenericUpdate<Boolean>)
    }


    impl<T: Eq> PartialEq for GenericUpdate<T> {
        fn eq(&self, other: &Self) -> bool {
            self.timestamp == other.timestamp
        }
    }


    impl<T: Eq> PartialOrd for GenericUpdate<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.timestamp.cmp(&other.timestamp))
        }
    }

    impl<T: Eq> Ord for GenericUpdate<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.timestamp.cmp(&other.timestamp)
        }
    }

    let i1 = Update::IntegerUpdate(GenericUpdate::<Integer> {
        timestamp: 3,
        data: Integer {
            val: -1
        }
    });

    let i2 = Update::IntegerUpdate( GenericUpdate::<Integer> {
        timestamp: 3,
        data: Integer {
            val: -13
        }
    });

    let u1 = Update::UnsignedUpdate( GenericUpdate::<Unsigned> {
        timestamp: 5,
        data: Unsigned {
            val: 1
        }
    });

    let u2 = Update::UnsignedUpdate( GenericUpdate::<Unsigned> {
        timestamp: 3,
        data: Unsigned {
            val: 2
        }
    });

    let b1 = Update::BooleanUpdate( GenericUpdate::<Boolean> {
        timestamp: 10,
        data: Boolean {
            val: true
        }
    });

    let b2 = Update::BooleanUpdate( GenericUpdate::<Boolean> {
        timestamp: 12,
        data: Boolean {
            val: false
        }
    });

    let mut vs: Vec<Update> = vec![i1,i2,b1,b2,u1,u2];
    println!("{:?}", vs);
    vs.sort();
    println!("{:?}", vs);

}
