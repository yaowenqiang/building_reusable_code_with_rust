mod animal {
    fn move_around() {}
    mod cat {
        fn meow() {}
    }
    mod dog {
        fn bakr() {}
    }
}

/*
animal::move_around()
animal::cat::meow()
animal::dog::bark()
*/
