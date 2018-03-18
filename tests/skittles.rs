#[macro_use]
extern crate skittles;

mod color {
    #[test]
    fn black() {
        assert_eq!(black!("test"), "\u{1b}[30mtest\u{1b}[0m");
    }

    #[test]
    fn blue() {
        assert_eq!(blue!("test"), "\u{1b}[34mtest\u{1b}[0m");
    }

    #[test]
    fn cyan() {
        assert_eq!(cyan!("test"), "\u{1b}[36mtest\u{1b}[0m");
    }

    #[test]
    fn green() {
        assert_eq!(green!("test"), "\u{1b}[32mtest\u{1b}[0m");
    }

    #[test]
    fn purple() {
        assert_eq!(purple!("test"), "\u{1b}[35mtest\u{1b}[0m");
    }

    #[test]
    fn red() {
        assert_eq!(red!("test"), "\u{1b}[31mtest\u{1b}[0m");
    }

    #[test]
    fn white() {
        assert_eq!(white!("test"), "\u{1b}[37mtest\u{1b}[0m");
    }

    #[test]
    fn yellow() {
        assert_eq!(yellow!("test"), "\u{1b}[33mtest\u{1b}[0m");
    }
}

mod format {
    #[test]
    fn blink() {
        assert_eq!(blink!("test"), "\u{1b}[5;37mtest\u{1b}[0m");
    }

    #[test]
    fn bold() {
        assert_eq!(bold!("test"), "\u{1b}[1;37mtest\u{1b}[0m");
    }

    #[test]
    fn dimmed() {
        assert_eq!(dimmed!("test"), "\u{1b}[2;37mtest\u{1b}[0m");
    }

    #[test]
    fn hidden() {
        assert_eq!(hidden!("test"), "\u{1b}[8;37mtest\u{1b}[0m");
    }

    #[test]
    fn italic() {
        assert_eq!(italic!("test"), "\u{1b}[3;37mtest\u{1b}[0m");
    }

    #[test]
    fn reverse() {
        assert_eq!(reverse!("test"), "\u{1b}[7;37mtest\u{1b}[0m");
    }

    #[test]
    fn strikethrough() {
        assert_eq!(strikethrough!("test"), "\u{1b}[9;37mtest\u{1b}[0m");
    }

    #[test]
    fn underline() {
        assert_eq!(underline!("test"), "\u{1b}[4;37mtest\u{1b}[0m");
    }
}
