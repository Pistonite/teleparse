use teleparse::prelude::*;

#[teleparse_derive(TokenType)]
pub enum TokenU32 {
    #[teleparse(terminal(X00000 = "0000"))]
    X00000,
    #[teleparse(terminal(X00001 = "0000"))]
    X00001,
    #[teleparse(terminal(X00010 = "0000"))]
    X00010,
    #[teleparse(terminal(X00011 = "0000"))]
    X00011,
    #[teleparse(terminal(X00100 = "0000"))]
    X00100,
    #[teleparse(terminal(X00101 = "0000"))]
    X00101,
    #[teleparse(terminal(X00110 = "0000"))]
    X00110,
    #[teleparse(terminal(X00111 = "0000"))]
    X00111,
    #[teleparse(terminal(X01000 = "0000"))]
    X01000,
    #[teleparse(terminal(X01001 = "0000"))]
    X01001,
    #[teleparse(terminal(X01010 = "0000"))]
    X01010,
    #[teleparse(terminal(X01011 = "0000"))]
    X01011,
    #[teleparse(terminal(X01100 = "0000"))]
    X01100,
    #[teleparse(terminal(X01101 = "0000"))]
    X01101,
    #[teleparse(terminal(X01110 = "0000"))]
    X01110,
    #[teleparse(terminal(X01111 = "0000"))]
    X01111,
    #[teleparse(terminal(X10000 = "0000"))]
    X10000,
    #[teleparse(terminal(X10001 = "0000"))]
    X10001,
    #[teleparse(terminal(X10010 = "0000"))]
    X10010,
    #[teleparse(terminal(X10011 = "0000"))]
    X10011,
    #[teleparse(terminal(X10100 = "0000"))]
    X10100,
    #[teleparse(terminal(X10101 = "0000"))]
    X10101,
    #[teleparse(terminal(X10110 = "0000"))]
    X10110,
    #[teleparse(terminal(X10111 = "0000"))]
    X10111,
    #[teleparse(terminal(X11000 = "0000"))]
    X11000,
    #[teleparse(terminal(X11001 = "0000"))]
    X11001,
    #[teleparse(terminal(X11010 = "0000"))]
    X11010,
    #[teleparse(terminal(X11011 = "0000"))]
    X11011,
    #[teleparse(terminal(X11100 = "0000"))]
    X11100,
    #[teleparse(terminal(X11101 = "0000"))]
    X11101,
    #[teleparse(terminal(X11110 = "0000"))]
    X11110,
    #[teleparse(terminal(X11111 = "0000"))]
    X11111,
}
