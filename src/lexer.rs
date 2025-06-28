#[derive(Debug, PartialEq, Eq, Clone,Copy)]
pub enum TokenType {
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Eq,     
    Assign, 
    Integer(i64),
    Variable(char), 
    Eof,
    Illegal,
}

pub struct Lexer {
    input: String,
    position: usize,      // Current reading position in input (index of ch)
    read_position: usize, // Next reading position in input (index after ch)
    ch: char,             // Current character under examination
}

impl Lexer {
    /// Creates a new Lexer instance and initializes it by reading the first character.
    pub fn new(expression: String) -> Lexer {
        let mut lexer = Lexer {
            input: expression,
            position: 0,
            read_position: 0,
            ch: '\0', // Temporary placeholder, will be set by first read_char
        };
        lexer.read_char(); // Read the very first character of the input
        lexer
    }

    /// Reads the next character from the input and advances the lexer's
    /// internal pointers (`position`, `read_position`, `ch`).
    /// If the end of the input is reached, `self.ch` is set to `'\0'`.
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'; // Sentinel for End Of File/Input
        } else {
            // Safely get the character (assuming ASCII for simplicity in this parser)
            // For full Unicode, you'd iterate over `chars()`
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Looks at the character at `read_position` without advancing the lexer's state.
    /// Returns `None` if at the end of input.
    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input.as_bytes().get(self.read_position).map(|&b| b as char)
        }
    }

    /// Skips any whitespace characters by repeatedly calling `read_char`.
    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    /// Reads a sequence of numeric characters starting from `self.ch`
    /// and returns them as a `String`.
    /// This method will advance `self.ch` past the end of the number.
    pub fn read_number(&mut self) -> String {
        let start_position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        // Extract the substring containing the number
        self.input[start_position..self.position].to_string()
    }

    /// Gets the next token from the input stream.
    /// This is the main function called by the parser.
    pub fn get_token(&mut self) -> TokenType {
        self.skip_whitespace(); // Always skip whitespace before identifying a token

        let token_type = match self.ch {
            // Single-character operators and delimiters
            '+' => {
                self.read_char(); // Advance past '+'
                TokenType::Plus
            },
            '-' => {
                self.read_char(); // Advance past '-'
                TokenType::Minus
            },
            '*' => {
                self.read_char(); // Advance past '*'
                TokenType::Multiply
            },
            '/' => {
                self.read_char(); // Advance past '/'
                TokenType::Divide
            },
            '(' => {
                self.read_char(); // Advance past '('
                TokenType::LParen
            },
            ')' => {
                self.read_char(); // Advance past ')'
                TokenType::RParen
            },

            // Handle '=' and '==' (Assign vs. Eq) - requires peek_char
            '=' => {
                if self.peek_char() == Some('=') {
                    self.read_char(); // Consume the first '='
                    self.read_char(); // Consume the second '='
                    TokenType::Eq // Produce '==' token
                } else {
                    self.read_char(); // Consume the single '='
                    TokenType::Assign // Produce '=' token
                }
            },

            // Numbers: '0' through '9'
            c @ '0'..='9' => {
                // read_number handles its own advancement, so no self.read_char() here
                let num_str = self.read_number();
                TokenType::Integer(num_str.parse::<i64>().expect("Failed to parse number"))
            },

            // Variables: Single alphabetic characters
            c if c.is_alphabetic() => {
                self.read_char(); // Advance past the variable char
                TokenType::Variable(c)
            },

            // End of File
            '\0' => TokenType::Eof, // No advancement needed for Eof

            // Anything else is an illegal character
            _ => {
                self.read_char(); // Advance past the illegal char
                TokenType::Illegal
            },
        };

        // Removed the final redundant `match token_type` block that was causing double advancement.
        // Each arm in the first match now correctly handles its own advancement.

        token_type // Return the identified token
    }
}
