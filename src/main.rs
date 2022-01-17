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

    enum DataType {
        Unsigned,
        Integer,
        Boolean
    }

    #[derive(Eq, Debug)]
    struct GenericUpdate<T> {
        pub timestamp: u32,
        data: T
    }

    type IntegerUpdate = GenericUpdate<Integer>;
    type UnsignedUpdate = GenericUpdate<Unsigned>;
    type BooleanUpdate = GenericUpdate<Boolean>;



    enum Update {
        IntegerUpdate,
        UnsignedUpdate,
        BooleanUpdate
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

    let i1 = IntegerUpdate {
        timestamp: 2,
        data: Integer {
            val: 1
        }
    };

    let i2 = IntegerUpdate {
        timestamp: 3,
        data: Integer {
            val: 1
        }
    };

    let f1 = UnsignedUpdate {
        timestamp: 3,
        data: Unsigned {
            val: 2
        }
    };

    let f2 = UnsignedUpdate {
        timestamp: 2,
        data: Unsigned {
            val: 2
        }
    };

    let b1 = BooleanUpdate {
        timestamp: 0,
        data: Boolean {
            val: false
        }
    };

    let b2 = BooleanUpdate {
        timestamp: 9,
        data: Boolean {
            val: true
        }
    };

    let mut vs = vec![i2,i1];
    println!("{:?}", vs);
    vs.sort();
    println!("{:?}", vs);

}
