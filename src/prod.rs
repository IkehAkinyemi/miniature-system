pub struct List<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    front: Link<T>,
    back: Link<T>
}