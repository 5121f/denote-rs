/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod signature {
    use denote::Signature;

    #[test]
    fn parse() {
        assert_eq!(Signature::parse("").to_string(), "");
    }
}

mod title {
    use denote::Title;

    #[test]
    fn parse() {
        assert_eq!(Title::parse("").to_string(), "");
    }
}

mod keywords {
    use denote::Keywords;

    #[test]
    fn prase() {
        assert_eq!(Keywords::parse_user_input("").to_string(), "");
        assert_eq!(Keywords::parse_schemed_string("").to_string(), "");
    }
}

mod extension {
    use denote::Extension;

    #[test]
    fn test() {
        assert_eq!(Extension::new(String::new()).to_string(), "");
    }
}

mod identifier {
    use denote::{Identifier, IdentifierError};

    #[test]
    fn test() {
        let id = Identifier::parse("");
        assert!(matches!(id, Err(IdentifierError::ParseDate)));
    }
}
