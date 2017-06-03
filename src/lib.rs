#[macro_export]
macro_rules! if_chain {
    (let $l:pat = $r:expr ;; => $b:block ) => {{
        if let $l = $r $b
    }};

    (let $l:pat = $r:expr ;; => $b1:block else $b2:block ) => {{
        if let $l = $r $b1 else $b2
    }};

    (let $l:pat = $r:expr $(, let $ll:pat = $rr:expr )+ ;; => $b:block ) => {{
        if let ( $l, $( $ll ),+ ) = ( $r, $( $rr ),+) $b
    }};

    (let $l:pat = $r:expr $(, let $ll:pat = $rr:expr )+ ;; => $b1:block else $b2:block) => {{
        if let ( $l, $( $ll ),+ ) = ( $r, $( $rr ),+) $b1 else $b2
    }};
}

#[test]
fn match_without_else_succeeds() {
    let x = Some(3);

    if_chain!(let Some(y) = x;; => {
        assert_eq!(y, 3);
        return;
    });

    panic!("match failed when success was expected!");
}

#[test]
fn match_without_else_fails() {
    let x: Option<()> = None;

    if_chain!(let Some(_) = x;; => {
        panic!("match succeeded when failure was expected!");
    });
}

#[test]
fn multi_match_without_else_succeeds() {
    let x = Some(3);
    let y: Result<&str, ()> = Ok("foo");
    let z = (false, -117);

    if_chain!(let Some(a) = x, let Ok(b) = y, let (c, d) = z;; => {
        assert_eq!(a, 3);
        assert_eq!(b, "foo");
        assert_eq!(c, false);
        assert_eq!(d, -117);
        return;
    });

    panic!("match failed when success was expected");
}

#[test]
fn multi_match_without_else_fails() {
    let x: Option<()> = None;
    let y: Result<&str, ()> = Ok("foo");
    let z = (false, -117);

    if_chain!(let Some(_) = x, let Ok(_) = y, let (_, _) = z;; => {
        panic!("match succeeded when failure was expected!");
    });
}

#[test]
fn match_with_else_succeeds() {
    let x = Some(3);

    if_chain!(let Some(y) = x;; => {
        assert_eq!(y, 3);
    } else {
        panic!("match failed when success was expected!");
    });
}

#[test]
fn match_with_else_fails() {
    let x: Option<()> = None;

    if_chain!(let Some(_) = x;; => {
        panic!("match succeeded when failure was expected!");
    } else {
        assert!(true);
    });
}

#[test]
fn multi_match_with_else_succeeds() {
    let x = Some(3);
    let y: Result<&str, ()> = Ok("foo");
    let z = (false, -117);

    if_chain!(let Some(a) = x, let Ok(b) = y, let (c, d) = z;; => {
        assert_eq!(a, 3);
        assert_eq!(b, "foo");
        assert_eq!(c, false);
        assert_eq!(d, -117);
        return;
    } else {
        panic!("match failed when success was expected");
    });
}

#[test]
fn multi_match_with_else_fails() {
    let x: Option<()> = None;
    let y: Result<&str, ()> = Ok("foo");
    let z = (false, -117);

    if_chain!(let Some(_) = x, let Ok(_) = y, let (_, _) = z;; => {
        panic!("match succeeded when failure was expected");
    } else {
        assert!(true);
    });
}
