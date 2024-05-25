fn main() {
    println!("Hello, world!");
}

// pub struct RegenData {
// }
//
// // the language needs a token type
// // the token type is the smallest unit of building the AST
// // needs to be an enum with no fields
// // order does not matter for the enum variants, except for internal representation
// #[derive(llnparse::TokenType)]
// pub enum RegenTokenType {
//     #[ignore] // means will be missing from lexer output. Like whitespaces
//     Ignore,
//     Keyword,
//     Ident,
//     RegExp,
//     Literal,
//     Symbol,
//     #[extract] // means will be not put into AST
//     Comment,
//     #[unknown] // must have this for tokens that fail to parse
//     Unknown,
// }
//
// // convience type
// type RegenToken = llnparse::Token<RegenTokenType>;
//
// // with the tokens defined, it's time to define what patterns match each token
// // and derive a tokenizer (lexer)
// // order matters for each rule
// // rule can be regex or a literal
// // regex are compiled statically
// #[derive(llnparse::Lexer)]
// #[lexer(RegenTokenType)] // define the token type
// // define each rules. order matters, as the first rule that matches will be used
// // use regex"..." for regular expressions, and just "..." for literals
//
// #[regex(Ignore, r#"\s+"#)]  // ignore whitespaces
// #[regex(Comment, r#"\/\/[^\n]*\n?"#)]; // single line comments
// #[regex(Comment, r#"\/\*([^\*]|(\*[^\/]))*\*\/"#)]; // block comments
//
// // Literal and RegExp, which are surrounded by either "" or //
// #[regex(Literal, r#""((\\.)|[^\\"])*""#)];
// #[regex(RegExp,  r#"\/((\\.)|[^\\\/])*\/"#)];
//
// // Keywords are literals
// #[literal(Keyword, "ignore")];
// #[literal(Keyword,"extract")];
// #[literal(Keyword,"token")];
// #[literal(Keyword,"semantic")];
// #[literal(Keyword,"rule")];
// #[literal(Keyword,"optional")];
// #[literal(Keyword,"context")];
//
// // Special Symbols
// #[regex(Symbol,r#"[{};|()=,:\.\[\]\+]"#)];
//
// // Identifier is alphanumeric and underscore, but does not start with digit
// #[regex(Identifier,r#"[_a-zA-Z]\w*"#)];
// pub struct RegenLexer; // great, we have a lexer now!
// // Lexer: (&Source) -> RefTokenStream<RegenToken>
// // Lexer: (Source) -> TokenStream<RegenToken> source lives with the tokens
// // each token has: span + token type
//
// // optionally, you can define semantic types
// // semantic types are basically tags for the AST nodes
// // for example, the token type could be Ident, but semantic type could
// // be Variable, Function, etc.
// #[derive(llnparse::SemanticType)]
// pub enum RegenSemanticType {
//     Variable,
//     Token,
//     Semantic,
//     Rule,
//     HookName,
//     HookType,
//     ContextType,
// }
//
// // with the tokens, it's time to construct our Abstract Syntax Tree (AST)
// // derive the SyntaxTree trait
// //
// // optionally you can specify a context type, which will be mutably accessible
// // when parsing the AST (ContextSyntaxTree needed)
// // this is needed if you need to share information between nodes that are not parent-child
// #[derive(llnparse::ContextSyntaxTree)]
// #[context(RegenData)] // define the context type
// #[token(RegenTokenType)] // define the token type
// #[semantic(RegenSemanticType)] // define the semantic type
// pub struct TopLevelStatement {
//     // there must be one field that is an AST node type (Node or NodeSem)
//     // this also serves to declare the token and semantic type
//     // by default, first is node unless there is an explicit #[node] attribute
//     #[node]
//     pub node: NodeSem<RegenTokenType, RegenSemanticType>,
//     // subtrees can just use their type, or Boxed
//     pub body: TopLevelDefine,
//     // use token attribute to match a specific token + litral content of that token
//     // for example, this will only allow a token of type Symbol with content ";"
//     // the type is fake and specify the type of the token
//     // the final AST will contain the token Span + type
//     #[token(Symbol, ";")] 
//     pub semi: RegenToken,
// }
//
// // for union types, use the enum syntax
// // you can box some to keep the size of the enum sane. It's required for recursive types
// #[derive(llnparse::ContextSyntaxTree)]
// #[context(RegenData)] // define the context type
// #[token(RegenTokenType)] // define the token type
// #[semantic(RegenSemanticType)] // define the semantic type
// pub enum TopLevelDefine {
//     // consume means that the AST node will be consumed by the context (moved to the context)
//     // leaving only a (cheaply-cloned) node in its place
//     // you must manually implement the ConsumeBy<YourContext> trait for the AST node
//     #[consume(DefineIncludeStatement)]
//     DefineIncludeStatement(RegenAstNode),
//     #[consume(DefineContextStatement)]
//     DefineContextStatement(RegenAstNode),
//     #[consume(DefineRuleStatement)]
//     DefineRuleStatement(RegenAstNode),
//     DefineTokenTypeStatement(DefineTokenTypeStatement),
//     DefineIgnoreTokenRuleStatement(DefineIgnoreTokenRuleStatement),
//     DefineTokenRuleStatement(DefineTokenRuleStatement),
//     DefineSemanticRuleStatement(DefineSemanticRuleStatement),
// }
//
// #[derive(llnparse::ContextSyntaxTree)]
// #[context(RegenData)] // define the context type
// #[token(RegenTokenType)] // define the token type
// #[semantic(RegenSemanticType)] // define the semantic type
// pub struct DefineIncludeStatement {
//     #[node]
//     pub node: RegenAstNode,
//     // without a literal string, the token attribute will match any token of that type
//     #[token(Literal)]
//     pub path: RegenToken,
// }
//
// impl llnparse::ConsumeBy<RegenData> for DefineIncludeStatement {
//     fn consume(self, _context: &mut RegenData) {
//         // let context consume this node and record its data
//         // effectively take data out of the AST
//     }
// }
//
// pub struct DefineContextStatement {
//     #[node]
//     pub node: RegenAstNode,
//     #[token(Keyword, "context")]
//     pub kw_context: RegenToken,
//     // use semantic attribute to automatically apply a semantic type to this node
//     // the following means:
//     // - match a token of type Literal
//     // - mark it as the ContextType semantic type
//     #[semantic(ContextType)]
//     #[token(Literal)]
//     pub context_type: RegenToken,
// }
//
// pub struct DefineRuleStatement {
//     #[node]
//     pub node: RegenAstNode,
//     #[token(Keyword, "rule")]
//     pub kw_rule: RegenToken,
//     // use the Option type to allow for optional fields when parsing
//     // you can also use the optional attribute to force treat the field as an Option
//     //
//     // from attribute allows you to consume the node entirely
//     // and replace it with another type that is not a AST node
//     // Of course, Hook needs to implement From<HookAttribute> 
//     #[from(HookAttribute)]
//     pub hook_attr: Option<Node<Hook>>,
//     #[token(Identifier)]
//     #[semantic(Rule)]
//     pub rule_name: RegenToken,
//     pub rule_body: RuleBody,
// }
//
// pub struct UnionRuleBody {
//     #[node]
//     pub node: RegenAstNode,
//     #[token(Symbol, "=")]
//     pub sym_eq: RegenToken,
//     // use the list attribute and llnparse::List type to parse a list of items
//     // currently it's internally VecDeque, but it might change
//     #[list(token(Identifier), semantic(Rule), no_trailing_sep)] 
//     // ^ this can be inferred if the type is List or llnparse::List, or SepList
//     // but if you need to specify extra attributes like token, semantic, etc,
//     // then it needs to be explicit
//     #[seperator(token(Symbol, "|"))]
//     #[semantic(Rule)] // semantic will be applied to each item of the list
//     pub rules: llnparse::SepList<RegenToken, RegenToken>,
//     // more note for list:
//     // - if it must be non-empty, use #[list(non_empty)] explicity
//     // - list is not compatible with optional attribute, as empty list = None
//     // - if you want to not allow trailing separator, use #[list(no_trailing_sep)]
// }
