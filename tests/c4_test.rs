use c4id::c4::c4_file;

#[test]
fn c4_file_test() {
    let real = "c45ofc7UjV8ZVECmYD1tZbHXWafRaX7hg56bV9LqEpe73g4HedomAU1niYBcmcTA1sqxV8FLxhbUHFtutvfszL6JWM";
    let file = "tests/assets/sample.mhl";
    let result = c4_file(file);
    assert_eq!(result, real);
}

#[test]
fn c4_90_char() {
    let samples = vec![
        (
            "tests/assets/0001.mhl",
            "c41hn27Ns92UvAWFt2qrmUhCRxGzXvL91grAptHjE3647yUUVeB9kDf8NZEcEJqfk8cTYyrVQEFtZc8EpXmwo1vPA1",
        ),
        (
            "tests/assets/0003.mhl",
            "c41GiCJz2Va6wLZ4H8W5VsPqBizirfmUBnoLiPUaxNPhC58hcFk5gddhzZEzW4Gaiz5huMUrqp3awLv3VEHebeUu7p",
        ),
    ];

    for (file, hash) in samples {
        let c4id = c4_file(file);
        dbg!(&c4id, &hash);
        assert!(c4id.len() == 90);
        assert_eq!(c4id, hash);
    }
}
