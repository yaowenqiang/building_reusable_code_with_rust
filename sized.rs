#![no_main]

struct HasSizedT<T>(T); // T: Sized by default
struct NotSizedT<T: ?Sized>(T);

// [i32] is not sized
struct NoOK(HasSizedT<[i32]);

struct OK(NotSizedT<[i32]);
