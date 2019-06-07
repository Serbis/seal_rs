pub trait Promise<T> {
    fn try_complete(result: Result<T>);

    fn complete() {

    }

    fn success() {

    }

    fn try_success() {

    }

    fn failure() {

    }

    fn tryFailure() {

    }
}