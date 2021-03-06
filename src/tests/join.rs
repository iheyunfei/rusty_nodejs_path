use crate as nodejs_path;

#[test]
fn universal() {
    let join_tests = [
        (vec![".", "x/b", "..", "/b/c.js"], "x/b/c.js"),
        (vec![], "."),
        (vec!["/.", "x/b", "..", "/b/c.js"], "/x/b/c.js"),
        (vec!["/foo", "../../../bar"], "/bar"),
        (vec!["foo", "../../../bar"], "../../bar"),
        (vec!["foo/", "../../../bar"], "../../bar"),
        (vec!["foo/x", "../../../bar"], "../bar"),
        (vec!["foo/x", "./bar"], "foo/x/bar"),
        (vec!["foo/x/", "./bar"], "foo/x/bar"),
        (vec!["foo/x/", ".", "bar"], "foo/x/bar"),
        (vec!["./"], "./"),
        (vec![".", "./"], "./"),
        (vec![".", ".", "."], "."),
        (vec![".", "./", "."], "."),
        (vec![".", "/./", "."], "."),
        (vec![".", "/////./", "."], "."),
        (vec!["."], "."),
        (vec!["", "."], "."),
        (vec!["", "foo"], "foo"),
        (vec!["foo", "/bar"], "foo/bar"),
        (vec!["", "/foo"], "/foo"),
        (vec!["", "", "/foo"], "/foo"),
        (vec!["", "", "foo"], "foo"),
        (vec!["foo", ""], "foo"),
        (vec!["foo/", ""], "foo/"),
        (vec!["foo", "", "/bar"], "foo/bar"),
        (vec!["./", "..", "/foo"], "../foo"),
        (vec!["./", "..", "..", "/foo"], "../../foo"),
        (vec![".", "..", "..", "/foo"], "../../foo"),
        (vec!["", "..", "..", "/foo"], "../../foo"),
        (vec!["/"], "/"),
        (vec!["/", "."], "/"),
        (vec!["/", ".."], "/"),
        (vec!["/", "..", ".."], "/"),
        (vec![""], "."),
        (vec!["", ""], "."),
        (vec![" /foo"], " /foo"),
        (vec![" ", "foo"], " /foo"),
        (vec![" ", "."], " "),
        (vec![" ", "/"], " /"),
        (vec![" ", ""], " "),
        (vec!["/", "foo"], "/foo"),
        (vec!["/", "/foo"], "/foo"),
        (vec!["/", "//foo"], "/foo"),
        (vec!["/", "", "/foo"], "/foo"),
        (vec!["", "/", "foo"], "/foo"),
        (vec!["", "/", "/foo"], "/foo"),
    ];
    join_tests.iter().for_each(|(input, right)| {
        assert_eq!(
            &nodejs_path::posix::join_impl(&input),
            right,
            "for input {:?}",
            input
        );
    });
}
