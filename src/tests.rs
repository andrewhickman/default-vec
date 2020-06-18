use super::DefaultVec;

#[test]
fn test_basic_usage() {
    let mut vec = DefaultVec::new();

    assert_eq!(*vec.get_mut(0), 0);
    assert_eq!(*vec.get_mut(1), 0);
    assert_eq!(*vec.get_mut(2), 0);
    assert_eq!(*vec.get_mut(3), 0);

    assert_eq!(vec.insert(0, 5), 0);
    assert_eq!(vec.insert(1, 2), 0);
    assert_eq!(vec.insert(2, -6), 0);
    assert_eq!(vec.insert(3, 12), 0);
    assert_eq!(vec.insert(10, 66), 0);

    assert_eq!(*vec.get(0), 5);
    assert_eq!(*vec.get(1), 2);
    assert_eq!(*vec.get(2), -6);
    assert_eq!(*vec.get(3), 12);
    assert_eq!(*vec.get(4), 0);
    assert_eq!(*vec.get(9), 0);
    assert_eq!(*vec.get(10), 66);

    assert_eq!(vec.remove(0), 5);
    assert_eq!(vec.remove(1), 2);
    assert_eq!(vec.remove(2), -6);
    assert_eq!(vec.remove(3), 12);

    assert_eq!(*vec.get_mut(0), 0);
    assert_eq!(*vec.get_mut(1), 0);
    assert_eq!(*vec.get_mut(2), 0);
    assert_eq!(*vec.get_mut(3), 0);
}

#[test]
#[should_panic]
fn test_panic() {
    let mut vec = DefaultVec::new();

    assert_eq!(vec.insert(0, 5), 0);
    assert_eq!(vec.insert(1, 2), 0);
    assert_eq!(vec.insert(2, -6), 0);
    assert_eq!(vec.insert(3, 12), 0);

    assert_eq!(*vec.get(4), 0);
}
