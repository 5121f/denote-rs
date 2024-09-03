/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod extention {
    use crate::Extention;

    #[test]
    fn test() {
        assert_eq!(Extention::new(String::from("ext")).to_string(), ".ext");
        assert_eq!(Extention::new(String::new()).to_string(), "");
    }
}

mod keywords {
    use crate::Keywords;

    #[test]
    fn prase() {
        assert_eq!(Keywords::parse_user_input("").to_string(), "");
        assert_eq!(Keywords::parse_schemed_string("").to_string(), "");
    }
}