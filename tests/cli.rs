use std::fs;

#[test]
fn input_output_file_args_work() {
    let dir = tempfile::tempdir().expect("tempdir");
    let in_path = dir.path().join("in.txt");
    let out_path = dir.path().join("out.txt");

    fs::write(&in_path, "Hello File").expect("write input");

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("str2futhark");
    cmd.args([
        "-i",
        in_path.to_str().unwrap(),
        "-o",
        out_path.to_str().unwrap(),
    ])
    .assert()
    .success();

    let out = fs::read_to_string(out_path).expect("read output");
    assert_eq!(out.trim(), "ᚺᛖᛚᛚᛟ ᚠᛁᛚᛖ");
}
