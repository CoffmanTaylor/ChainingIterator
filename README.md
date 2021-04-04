# Chaining Iterator
This is a simple runtime iterator chain. All iterators in the chain must be of the same type. It can also be used to chain boxed iterator objects if you want to chain iterators of different types.

## Design
The underling data structure is a `VecDeque` that stores each iterator until they are exhausted. This allows polling from the front and pushing to the back, as well as polling from the back and pushing to the front if the underling iterators implement `DoubleEndedIterator`.