/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod signature {
    use denote::Signature;

    #[test]
    fn empty_input() {
        assert!(Signature::parse("").is_none());
    }
}

mod title {
    use denote::Title;

    #[test]
    fn empty_input() {
        assert!(Title::parse("").is_none());
    }
}

mod keywords {
    use denote::Keywords;

    #[test]
    fn empty_input() {
        assert!(Keywords::parse_user_input("").is_none());
        assert!(Keywords::parse_schemed_string("").is_none());
    }
}

mod identifier {
    use denote::Identifier;

    #[test]
    fn empty_input() {
        let id = Identifier::parse("");
        assert!(id.is_none());
    }

    #[test]
    fn file_metadata() {
        let project_dir = env!("CARGO_MANIFEST_DIR");
        let testfile = format!("{project_dir}/tests/testfile");
        let id = Identifier::from_file_metadata(&testfile).unwrap();
        assert!(id.to_string().starts_with("20240910T220434"));
    }
}
