/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod signature {
    use zeroten_denote::Signature;

    #[test]
    fn empty_input() {
        assert!(Signature::parse("").is_none());
    }
}

mod title {
    use zeroten_denote::Title;

    #[test]
    fn empty_input() {
        assert!(Title::parse("").is_none());
    }
}

mod keywords {
    use zeroten_denote::Keywords;

    #[test]
    fn empty_input() {
        assert!(Keywords::parse_user_input("").is_none());
        assert!(Keywords::parse_schemed_string("").is_none());
    }
}

mod identifier {
    use zeroten_denote::Identifier;

    #[test]
    fn empty_input() {
        let id = Identifier::parse("");
        assert!(id.is_none());
    }
}

mod extension {
    use zeroten_denote::Extension;

    #[test]
    fn empty_input() {
        let extension = Extension::new("");
        assert!(extension.is_none());
    }

    #[test]
    fn extension() {
        let ext = Extension::new("txt").unwrap();
        assert_eq!(ext.to_string(), ".txt");
        let ext = Extension::new(".md").unwrap();
        assert_eq!(ext.to_string(), ".md");
    }
}

mod denote {
    #[test]
    fn doc() {
        use zeroten_denote::{Denote, Extension, Identifier, Signature, Title};

        // You can use something like `Identifier::now()` but for example, we will take an already
        // formatted identifier
        let identifier = Identifier::parse("20240912T13015412").unwrap();
        let denote = Denote::new(identifier)
            .title(Title::parse("Some title").unwrap())
            .signature(Signature::parse("1b").unwrap())
            .extension(Extension::new("txt").unwrap())
            .to_string();
        assert_eq!(denote, "20240912T13015412==1b--some-title.txt");
    }
}
