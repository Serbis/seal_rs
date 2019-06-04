
# About

Seal_rs is a set of various modules for highly concurrent applications, developing under strongly impact from the titans of the asynchronous programming world, such us Erlang, Scala and Akka.

Library includes next submodules:
* [actors](https://docs.rs/seal_rs/*/seal_rs/actors/index.html) - Actor-based concurrent runtime, based on the untyped actors and paradigms which actively used in Akka framework and Erlang language.
* future - Future-based runtime based on the classic computer-science definition of 'Future/Promise' paradigm. ( under developing )
* executor - Set of various concurrent executors, actively used by other modules of the library, and which may be used by the user.
* [testkit](https://docs.rs/seal_rs/*/seal_rs/testkit/index.html) - Test framework for deep and seamless testing of code developed based on this library.

This library have a very reach documentation with big count of examples and explanations of internal library architecture. Read on the [docs.rs](https://docs.rs/seal_rs/).

# New in release

Developed basic features of the actor's testkit, which may be used for isolated unit testing of separate actors. You may read about how to use this features in the [documentation](https://docs.rs/seal_rs/*/seal_rs/testkit/actors/index.html).

See [changelog](https://github.com/Serbis/seal_rs/blob/master/changelog.md) for info about new releases.

# Why experimental?

Comments about why the library in the experimental state. This library while does not used in my any real project. Reason for this in that I need develop basic fetures of actors / futures . Without this, real usage of this library is very difficult. This situation lead to fact, that for now, library probably contain a very big count of bags. And I will don't see it, until I start to use this library for real development. Accordingly this, while the library will stay in experimental state, I does not may give any guarantee, that separate components of the library work correctly.