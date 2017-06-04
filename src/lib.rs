#[macro_export]
macro_rules! if_chain {
    ( [ let $l:pat $( if $c:expr ),* => $r:expr ] $b:block ) => {{
        #[allow(unreachable_patterns)]
        match $r {
            $l $(if $c),* => $b
            _ => {}
        }
    }};

    ([ let $l:pat $( if $c:expr ),* => $r:expr ] $b1:block else $b2:block ) => {{
        #[allow(unreachable_patterns)]
        match $r {
            $l $( if $c ),* => $b1
            _ => $b2
        }
    }};

    ([ let $l:pat $( if $c:expr ),* => $r:expr $(, let $ll:pat $( if $cc:expr ),* => $rr:expr )+ ] $b:block ) => {{
        #[allow(unreachable_patterns)]
        match $r {
            $l $( if $c ),* => if_chain!([$( let $ll $( if $cc ),* => $rr ),+] $b),
            _ => {}
        }
    }};

    ([ let $l:pat $( if $c:expr ),* => $r:expr $(, let $ll:pat $( if $cc:expr ),* => $rr:expr )+ ] $b1:block else $b2:block) => {{
        #[allow(unreachable_patterns)]
        match $r {
            $l $(if $c ),* => if_chain!([$( let $ll $( if $cc ),* => $rr ),+] $b1 else $b2),
            _ => $b2
        }
    }};
}

//
// no else, no guard
//

#[test]
fn match_without_else_succeeds() {
    let x = Some(3);

    if_chain!([let Some(y) => x] {
        assert_eq!(y, 3);
        return;
    });

    panic!("match failed when success was expected!");
}

#[test]
fn match_without_else_fails() {
    let x: Option<()> = None;

    if_chain!([let Some(_) => x] {
        panic!("match succeeded when failure was expected!");
    });
}

#[test]
fn multi_match_without_else_succeeds() {
    let x = Some(3);
    let y: Result<&str, ()> = Ok("foo");
    let z = (false, -117);

    if_chain!([let Some(a) => x,
               let Ok(b) => y,
               let (c, d) => z] {
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

    if_chain!([let Some(_) => x,
               let Ok(_) => y,
               let (_, _) => z] {
        panic!("match succeeded when failure was expected!");
    });
}

//
// with else, no guard
//

#[test]
fn match_with_else_succeeds() {
    let x = Some(3);

    if_chain!([let Some(y) => x] {
        assert_eq!(y, 3);
    } else {
        panic!("match failed when success was expected!");
    });
}

#[test]
fn match_with_else_fails() {
    let x: Option<()> = None;

    if_chain!([let Some(_) => x] {
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

    if_chain!([let Some(a) => x,
               let Ok(b) => y,
               let (c, d) => z] {
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
    let x = Some(3);
    let y: Result<&str, ()> = Err(());
    let z = (false, -117);

    if_chain!([let Some(_) => x,
               let Ok(_) => y,
               let (_, _) => z] {
        panic!("match succeeded when failure was expected");
    } else {
        assert!(true);
    });
}

//
// no else, with guard
//

#[test]
fn match_without_else_with_guard_succeeds() {
    let x = Some(3);

    if_chain!([let Some(y) if y > 2 => x] {
        assert_eq!(y, 3);
        return;
    });

    panic!("match failed when success was expected!");
}

#[test]
fn match_without_else_with_guard_fails() {
    let x: Option<i8> = None;

    if_chain!([let Some(_y) if _y > 2 => x] {
        panic!("match succeeded when failure was expected!");
    });
}

#[test]
fn match_without_else_with_guard_fails_from_guard() {
    let x = Some(3);

    if_chain!([let Some(_y) if _y > 4 => x] {
        panic!("match succeeded when failure was expected!");
    });
}

#[test]
fn multi_match_without_else_with_guard_succeeds() {
    let x = Some(3);
    let y: Result<&str, ()> = Ok("foo");
    let z = (false, -117);

    if_chain!([let Some(a) if a > 2 => x,
               let Ok(b) => y,
               let (c, d) if !c => z] {
        assert_eq!(a, 3);
        assert_eq!(b, "foo");
        assert_eq!(c, false);
        assert_eq!(d, -117);
        return;
    });

    panic!("match failed when success was expected");
}

#[test]
fn multi_match_without_else_with_guard_fails() {
    let x: Option<()> = None;
    let y: Result<i8, ()> = Ok(3);
    let z = (false, -117);

    if_chain!([let Some(_) => x,
               let Ok(_b) if _b > 2 => y,
               let (_, _) => z] {
        panic!("match succeeded when failure was expected!");
    });
}

#[test]
fn multi_match_without_else_with_guard_fails_from_guard() {
    let x = Some(3);
    let y: Result<&str, ()> = Ok("foo");
    let z = (false, -117);

    if_chain!([let Some(_a) => x,
               let Ok(_b) => y,
               let (_c, _) if _c => z] {
        panic!("match succeeded when failure was expected!");
    });
}

//
// with else, with guard
//

#[test]
fn match_with_else_with_guard_succeeds() {
    let x = Some(3);

    if_chain!([let Some(y) if y > 2 => x] {
        assert_eq!(y, 3);
    } else {
        panic!("match failed when success was expected!");
    });
}

#[test]
fn match_with_else_with_guard_fails() {
    let x: Option<bool> = None;

    if_chain!([let Some(_y) if _y => x] {
        panic!("match succeeded when failure was expected!");
    } else {
        assert!(true);
    });
}

#[test]
fn match_with_else_with_guard_fails_from_guard() {
    let x = Some(false);

    if_chain!([let Some(_y) if _y => x] {
        panic!("match succeeded when failure was expected!");
    } else {
        assert!(true);
    });
}

#[test]
fn multi_match_with_else_with_guard_succeeds() {
    let x = Some(3);
    let y: Result<&str, ()> = Ok("foo");
    let z = (false, -117);

    if_chain!([let Some(a) => x,
               let Ok(b) if !b.is_empty() => y,
               let (c, d) if d < 0 => z] {
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
fn multi_match_with_else_with_guard_fails() {
    let x = Some(3);
    let y: Result<&str, ()> = Err(());
    let z = (false, -117);

    if_chain!([let Some(_a) if _a > 2 => x,
               let Ok(_) => y,
               let (_, _) => z] {
        panic!("match succeeded when failure was expected");
    } else {
        assert!(true);
    });
}

#[test]
fn multi_match_with_else_with_guard_fails_from_guard() {
    let x = Some(3);
    let y: Result<&str, ()> = Err(());
    let z = (false, -117);

    if_chain!([let Some(_a) if _a > 2 => x,
               let Ok(_) => y,
               let (_, _) => z] {
        panic!("match succeeded when failure was expected");
    } else {
        assert!(true);
    });
}
