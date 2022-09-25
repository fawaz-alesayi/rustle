// #[derive(Clone, Debug)]
// pub enum Node {
//     AssignmentProperty(AssignmentProperty),
//     CatchClause(CatchClause),
//     Class(Class),
//     ClassBody(ClassBody),
//     Expression(Expression),
//     Function(Function),
//     Identifier(Identifier),
//     Literal(Literal),
//     MethodDefinition(MethodDefinition),
//     ModuleDeclaration(ModuleDeclaration),
//     ModuleSpecifier(ModuleSpecifier),
//     Pattern(Pattern),
//     PrivateIdentifier(PrivateIdentifier),
//     Program(Program),
//     Property(Property),
//     PropertyDefinition(PropertyDefinition),
//     SpreadElement(SpreadElement),
//     Statement(Statement),
//     Super(Super),
//     SwitchCase(SwitchCase),
//     TemplateElement(TemplateElement),
//     VariableDeclarator(VariableDeclarator),
//     //Testing purpose
//     Empty,
// }

// impl Node {
//     pub fn start(&self) -> Option<u32> {
//         match self {
//             Node::CatchClause(node) => node.base.start,
//             Node::Class(node) => match node {
//                 Class::Expr(expr) => expr.base.start,
//                 Class::Decl(expr) => expr.base.start,
//             },
//             Node::ClassBody(node) => node.base.start,
//             Node::Expression(node) => match node {
//                 Expression::Array(expr) => expr.base.start,
//                 Expression::Assignment(expr) => expr.base.start,
//                 Expression::Binary(expr) => expr.base.start,
//                 Expression::Call(expr) => expr.base.start,
//                 Expression::Conditional(expr) => expr.base.start,
//                 Expression::Func(expr) => expr.base.start,
//                 Expression::Id(expr) => expr.base.start,
//                 Expression::Literal(expr) => match expr {
//                     Literal::String(expr) => expr.base.start,
//                     Literal::Numeric(expr) => expr.base.start,
//                     Literal::Null(expr) => expr.base.start,
//                     Literal::Boolean(expr) => expr.base.start,
//                     Literal::RegExp(expr) => expr.base.start,
//                     Literal::Template(expr) => expr.base.start,
//                     Literal::BigInt(expr) => expr.base.start,
//                     Literal::Decimal(expr) => expr.base.start,
//                 },
//                 Expression::Logical(expr) => expr.base.start,
//                 Expression::Member(expr) => expr.base.start,
//                 Expression::New(expr) => expr.base.start,
//                 Expression::Object(expr) => expr.base.start,
//                 Expression::Sequence(expr) => expr.base.start,
//                 Expression::Parenthesized(expr) => expr.base.start,
//                 Expression::This(expr) => expr.base.start,
//                 Expression::Unary(expr) => expr.base.start,
//                 Expression::Update(expr) => expr.base.start,
//                 Expression::ArrowFunc(expr) => expr.base.start,
//                 Expression::Class(expr) => expr.base.start,
//                 Expression::MetaProp(expr) => expr.base.start,
//                 Expression::Super(expr) => expr.base.start,
//                 Expression::TaggedTemplate(expr) => expr.base.start,
//                 Expression::TemplateLiteral(expr) => expr.base.start,
//                 Expression::Yield(expr) => expr.base.start,
//                 Expression::Await(expr) => expr.base.start,
//                 Expression::Import(expr) => expr.base.start,
//                 Expression::OptionalMember(expr) => expr.base.start,
//                 Expression::OptionalCall(expr) => expr.base.start,
//                 Expression::TypeCast(expr) => expr.base.start,
//                 Expression::JSXElement(expr) => expr.base.start,
//                 Expression::JSXFragment(expr) => expr.base.start,
//                 Expression::Bind(expr) => expr.base.start,
//                 Expression::PipelinePrimaryTopicRef(expr) => expr.base.start,
//                 Expression::Do(expr) => expr.base.start,
//                 Expression::Record(expr) => expr.base.start,
//                 Expression::Tuple(expr) => expr.base.start,
//                 Expression::Module(expr) => expr.base.start,
//                 Expression::TSAs(expr) => expr.base.start,
//                 Expression::TSTypeAssertion(expr) => expr.base.start,
//                 Expression::TSNonNull(expr) => expr.base.start,
//             },
//             Node::Function(node) => match node {
//                 Function::Decl(expr) => expr.base.start,
//                 Function::Expr(expr) => expr.base.start,
//                 Function::ObjectMethod(expr) => expr.base.start,
//                 Function::Arrow(expr) => expr.base.start,
//                 Function::ClassMethod(expr) => expr.base.start,
//                 Function::ClassPrivateMethod(expr) => expr.base.start,
//             },
//             Node::Identifier(node) => node.base.start,
//             Node::Literal(node) => match node {
//                 Literal::String(expr) => expr.base.start,
//                 Literal::Numeric(expr) => expr.base.start,
//                 Literal::Null(expr) => expr.base.start,
//                 Literal::Boolean(expr) => expr.base.start,
//                 Literal::RegExp(expr) => expr.base.start,
//                 Literal::Template(expr) => expr.base.start,
//                 Literal::BigInt(expr) => expr.base.start,
//                 Literal::Decimal(expr) => expr.base.start,
//             },
//             Node::ModuleDeclaration(node) => match node {
//                 ModuleDeclaration::ExportAll(expr) => expr.base.start,
//                 ModuleDeclaration::ExportDefault(expr) => expr.base.start,
//                 ModuleDeclaration::ExportNamed(expr) => expr.base.start,
//                 ModuleDeclaration::Import(expr) => expr.base.start,
//             },
//             Node::ModuleSpecifier(node) => match node {
//                 ModuleSpecifier::Export(expr) => expr.base.start,
//                 ModuleSpecifier::ImportDefault(expr) => expr.base.start,
//                 ModuleSpecifier::ImportNamespace(expr) => expr.base.start,
//                 ModuleSpecifier::Import(expr) => expr.base.start,
//                 ModuleSpecifier::ExportNamespace(expr) => expr.base.start,
//                 ModuleSpecifier::ExportDefault(expr) => expr.base.start,
//             },
//             Node::Pattern(node) => match node {
//                 Pattern::Assignment(expr) => expr.base.start,
//                 Pattern::Array(expr) => expr.base.start,
//                 Pattern::Object(expr) => expr.base.start,
//             },
//             Node::Program(node) => node.base.start,
//             Node::Property(node) => match node {
//                 Property::ObjectProp(expr) => expr.base.start,
//                 Property::ClassProp(expr) => expr.base.start,
//                 Property::ClassPrivateProp(expr) => expr.base.start,
//             },
//             Node::SpreadElement(node) => node.base.start,
//             Node::Statement(node) => match node {
//                 Statement::Block(expr) => expr.base.start,
//                 Statement::Break(expr) => expr.base.start,
//                 Statement::Continue(expr) => expr.base.start,
//                 Statement::Debugger(expr) => expr.base.start,
//                 Statement::DoWhile(expr) => expr.base.start,
//                 Statement::Empty(expr) => expr.base.start,
//                 Statement::Expr(expr) => expr.base.start,
//                 Statement::ForIn(expr) => expr.base.start,
//                 Statement::For(expr) => expr.base.start,
//                 Statement::FuncDecl(expr) => expr.base.start,
//                 Statement::If(expr) => expr.base.start,
//                 Statement::Labeled(expr) => expr.base.start,
//                 Statement::Return(expr) => expr.base.start,
//                 Statement::Switch(expr) => expr.base.start,
//                 Statement::Throw(expr) => expr.base.start,
//                 Statement::Try(expr) => expr.base.start,
//                 Statement::VarDecl(expr) => expr.base.start,
//                 Statement::While(expr) => expr.base.start,
//                 Statement::With(expr) => expr.base.start,
//                 Statement::ClassDecl(expr) => expr.base.start,
//                 Statement::ExportAllDecl(expr) => expr.base.start,
//                 Statement::ExportDefaultDecl(expr) => expr.base.start,
//                 Statement::ExportNamedDecl(expr) => expr.base.start,
//                 Statement::ForOf(expr) => expr.base.start,
//                 Statement::ImportDecl(expr) => expr.base.start,
//                 Statement::DeclClass(expr) => expr.base.start,
//                 Statement::DeclFunc(expr) => expr.base.start,
//                 Statement::DeclInterface(expr) => expr.base.start,
//                 Statement::DeclModule(expr) => expr.base.start,
//                 Statement::DeclareModuleExports(expr) => expr.base.start,
//                 Statement::DeclTypeAlias(expr) => expr.base.start,
//                 Statement::DeclOpaqueType(expr) => expr.base.start,
//                 Statement::DeclVar(expr) => expr.base.start,
//                 Statement::DeclExportDeclaration(expr) => expr.base.start,
//                 Statement::DeclExportAllDeclaration(expr) => expr.base.start,
//                 Statement::InterfaceDecl(expr) => expr.base.start,
//                 Statement::OpaqueType(expr) => expr.base.start,
//                 Statement::TypeAlias(expr) => expr.base.start,
//                 Statement::EnumDecl(expr) => expr.base.start,
//                 Statement::TSDeclFunc(expr) => expr.base.start,
//                 Statement::TSInterfaceDecl(expr) => expr.base.start,
//                 Statement::TSTypeAliasDecl(expr) => expr.base.start,
//                 Statement::TSEnumDecl(expr) => expr.base.start,
//                 Statement::TSModuleDecl(expr) => expr.base.start,
//                 Statement::TSImportEqualsDecl(expr) => expr.base.start,
//                 Statement::TSExportAssignment(expr) => expr.base.start,
//                 Statement::TSNamespaceExportDecl(expr) => expr.base.start,
//             },
//             Node::Super(node) => node.base.start,
//             Node::SwitchCase(node) => node.base.start,
//             Node::TemplateElement(node) => node.base.start,
//             Node::VariableDeclarator(node) => node.base.start,
//             Node::AssignmentProperty(node) => match &node.property {
//                 Property::ObjectProp(prop) => prop.base.start,
//                 Property::ClassProp(prop) => prop.base.start,
//                 Property::ClassPrivateProp(prop) => prop.base.start,
//             },
//             Node::MethodDefinition(node) => node.base.start,
//             Node::PrivateIdentifier(node) => node.base.start,
//             Node::PropertyDefinition(node) => node.base.start,
//             Node::Empty => panic!("only for testing"),
//         }
//     }

//     pub fn end(&self) -> Option<u32> {
//         match self {
//             Node::CatchClause(node) => node.base.end,
//             Node::Class(node) => match node {
//                 Class::Expr(expr) => expr.base.end,
//                 Class::Decl(expr) => expr.base.end,
//             },
//             Node::ClassBody(node) => node.base.end,
//             Node::Expression(node) => match node {
//                 Expression::Array(expr) => expr.base.end,
//                 Expression::Assignment(expr) => expr.base.end,
//                 Expression::Binary(expr) => expr.base.end,
//                 Expression::Call(expr) => expr.base.end,
//                 Expression::Conditional(expr) => expr.base.end,
//                 Expression::Func(expr) => expr.base.end,
//                 Expression::Id(expr) => expr.base.end,
//                 Expression::Literal(expr) => match expr {
//                     Literal::String(expr) => expr.base.start,
//                     Literal::Numeric(expr) => expr.base.start,
//                     Literal::Null(expr) => expr.base.start,
//                     Literal::Boolean(expr) => expr.base.start,
//                     Literal::RegExp(expr) => expr.base.start,
//                     Literal::Template(expr) => expr.base.start,
//                     Literal::BigInt(expr) => expr.base.start,
//                     Literal::Decimal(expr) => expr.base.start,
//                 },
//                 Expression::Logical(expr) => expr.base.end,
//                 Expression::Member(expr) => expr.base.end,
//                 Expression::New(expr) => expr.base.end,
//                 Expression::Object(expr) => expr.base.end,
//                 Expression::Sequence(expr) => expr.base.end,
//                 Expression::Parenthesized(expr) => expr.base.end,
//                 Expression::This(expr) => expr.base.end,
//                 Expression::Unary(expr) => expr.base.end,
//                 Expression::Update(expr) => expr.base.end,
//                 Expression::ArrowFunc(expr) => expr.base.end,
//                 Expression::Class(expr) => expr.base.end,
//                 Expression::MetaProp(expr) => expr.base.end,
//                 Expression::Super(expr) => expr.base.end,
//                 Expression::TaggedTemplate(expr) => expr.base.end,
//                 Expression::TemplateLiteral(expr) => expr.base.end,
//                 Expression::Yield(expr) => expr.base.end,
//                 Expression::Await(expr) => expr.base.end,
//                 Expression::Import(expr) => expr.base.end,
//                 Expression::OptionalMember(expr) => expr.base.end,
//                 Expression::OptionalCall(expr) => expr.base.end,
//                 Expression::TypeCast(expr) => expr.base.end,
//                 Expression::JSXElement(expr) => expr.base.end,
//                 Expression::JSXFragment(expr) => expr.base.end,
//                 Expression::Bind(expr) => expr.base.end,
//                 Expression::PipelinePrimaryTopicRef(expr) => expr.base.end,
//                 Expression::Do(expr) => expr.base.end,
//                 Expression::Record(expr) => expr.base.end,
//                 Expression::Tuple(expr) => expr.base.end,
//                 Expression::Module(expr) => expr.base.end,
//                 Expression::TSAs(expr) => expr.base.end,
//                 Expression::TSTypeAssertion(expr) => expr.base.end,
//                 Expression::TSNonNull(expr) => expr.base.end,
//             },
//             Node::Function(node) => match node {
//                 Function::Decl(expr) => expr.base.end,
//                 Function::Expr(expr) => expr.base.end,
//                 Function::ObjectMethod(expr) => expr.base.end,
//                 Function::Arrow(expr) => expr.base.end,
//                 Function::ClassMethod(expr) => expr.base.end,
//                 Function::ClassPrivateMethod(expr) => expr.base.end,
//             },
//             Node::Identifier(node) => node.base.end,
//             Node::Literal(node) => match node {
//                 Literal::String(expr) => expr.base.end,
//                 Literal::Numeric(expr) => expr.base.end,
//                 Literal::Null(expr) => expr.base.end,
//                 Literal::Boolean(expr) => expr.base.end,
//                 Literal::RegExp(expr) => expr.base.end,
//                 Literal::Template(expr) => expr.base.end,
//                 Literal::BigInt(expr) => expr.base.end,
//                 Literal::Decimal(expr) => expr.base.end,
//             },
//             Node::ModuleDeclaration(node) => match node {
//                 ModuleDeclaration::ExportAll(expr) => expr.base.end,
//                 ModuleDeclaration::ExportDefault(expr) => expr.base.end,
//                 ModuleDeclaration::ExportNamed(expr) => expr.base.end,
//                 ModuleDeclaration::Import(expr) => expr.base.end,
//             },
//             Node::ModuleSpecifier(node) => match node {
//                 ModuleSpecifier::Export(expr) => expr.base.end,
//                 ModuleSpecifier::ImportDefault(expr) => expr.base.end,
//                 ModuleSpecifier::ImportNamespace(expr) => expr.base.end,
//                 ModuleSpecifier::Import(expr) => expr.base.end,
//                 ModuleSpecifier::ExportNamespace(expr) => expr.base.end,
//                 ModuleSpecifier::ExportDefault(expr) => expr.base.end,
//             },
//             Node::Pattern(node) => match node {
//                 Pattern::Assignment(expr) => expr.base.end,
//                 Pattern::Array(expr) => expr.base.end,
//                 Pattern::Object(expr) => expr.base.end,
//             },
//             Node::Program(node) => node.base.end,
//             Node::Property(node) => match node {
//                 Property::ObjectProp(expr) => expr.base.end,
//                 Property::ClassProp(expr) => expr.base.end,
//                 Property::ClassPrivateProp(expr) => expr.base.end,
//             },
//             Node::SpreadElement(node) => node.base.end,
//             Node::Statement(node) => match node {
//                 Statement::Block(expr) => expr.base.end,
//                 Statement::Break(expr) => expr.base.end,
//                 Statement::Continue(expr) => expr.base.end,
//                 Statement::Debugger(expr) => expr.base.end,
//                 Statement::DoWhile(expr) => expr.base.end,
//                 Statement::Empty(expr) => expr.base.end,
//                 Statement::Expr(expr) => expr.base.end,
//                 Statement::ForIn(expr) => expr.base.end,
//                 Statement::For(expr) => expr.base.end,
//                 Statement::FuncDecl(expr) => expr.base.end,
//                 Statement::If(expr) => expr.base.end,
//                 Statement::Labeled(expr) => expr.base.end,
//                 Statement::Return(expr) => expr.base.end,
//                 Statement::Switch(expr) => expr.base.end,
//                 Statement::Throw(expr) => expr.base.end,
//                 Statement::Try(expr) => expr.base.end,
//                 Statement::VarDecl(expr) => expr.base.end,
//                 Statement::While(expr) => expr.base.end,
//                 Statement::With(expr) => expr.base.end,
//                 Statement::ClassDecl(expr) => expr.base.end,
//                 Statement::ExportAllDecl(expr) => expr.base.end,
//                 Statement::ExportDefaultDecl(expr) => expr.base.end,
//                 Statement::ExportNamedDecl(expr) => expr.base.end,
//                 Statement::ForOf(expr) => expr.base.end,
//                 Statement::ImportDecl(expr) => expr.base.end,
//                 Statement::DeclClass(expr) => expr.base.end,
//                 Statement::DeclFunc(expr) => expr.base.end,
//                 Statement::DeclInterface(expr) => expr.base.end,
//                 Statement::DeclModule(expr) => expr.base.end,
//                 Statement::DeclareModuleExports(expr) => expr.base.end,
//                 Statement::DeclTypeAlias(expr) => expr.base.end,
//                 Statement::DeclOpaqueType(expr) => expr.base.end,
//                 Statement::DeclVar(expr) => expr.base.end,
//                 Statement::DeclExportDeclaration(expr) => expr.base.end,
//                 Statement::DeclExportAllDeclaration(expr) => expr.base.end,
//                 Statement::InterfaceDecl(expr) => expr.base.end,
//                 Statement::OpaqueType(expr) => expr.base.end,
//                 Statement::TypeAlias(expr) => expr.base.end,
//                 Statement::EnumDecl(expr) => expr.base.end,
//                 Statement::TSDeclFunc(expr) => expr.base.end,
//                 Statement::TSInterfaceDecl(expr) => expr.base.end,
//                 Statement::TSTypeAliasDecl(expr) => expr.base.end,
//                 Statement::TSEnumDecl(expr) => expr.base.end,
//                 Statement::TSModuleDecl(expr) => expr.base.end,
//                 Statement::TSImportEqualsDecl(expr) => expr.base.end,
//                 Statement::TSExportAssignment(expr) => expr.base.end,
//                 Statement::TSNamespaceExportDecl(expr) => expr.base.end,
//             },
//             Node::Super(node) => node.base.end,
//             Node::SwitchCase(node) => node.base.end,
//             Node::TemplateElement(node) => node.base.end,
//             Node::VariableDeclarator(node) => node.base.end,
//             Node::AssignmentProperty(node) => match &node.property {
//                 Property::ObjectProp(prop) => prop.base.end,
//                 Property::ClassProp(prop) => prop.base.end,
//                 Property::ClassPrivateProp(prop) => prop.base.end,
//             },
//             Node::MethodDefinition(node) => node.base.end,
//             Node::PrivateIdentifier(node) => node.base.end,
//             Node::PropertyDefinition(node) => node.base.end,
//             Node::Empty => panic!("only for testing"),
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub enum Kind {
//     Init,
//     Constructor,
//     Method,
//     Get,
//     Set,
// }

// #[derive(Clone, Debug)]
// pub struct AssignmentProperty {
//     property: Property,
//     value: Pattern,
//     method: bool,
//     kind: Kind, // "init"
// }

// impl AssignmentProperty {
//     pub fn kind() -> Kind {
//         Kind::Init
//     }
// }

// #[derive(Clone, Debug)]
// pub struct MethodDefinition {
//     base: BaseNode,
//     key: Key,
//     value: FunctionExpression,
//     kind: Kind, // "constructor" | "method" | "get" | "set"
//     computed: bool,
//     is_static: bool,
// }

// impl MethodDefinition {
//     pub fn get_type() -> String {
//         "MethodDefinition".to_string()
//     }
// }

// #[derive(Clone, Debug)]
// pub enum Key {
//     Expression(Expression),
//     PrivateIdentifier(PrivateIdentifier),
// }

// #[derive(Clone, Debug)]
// pub struct PrivateIdentifier {
//     base: BaseNode,
//     name: String,
// }

// impl PrivateIdentifier {
//     pub fn get_type() -> String {
//         "PrivateIdentifier".to_string()
//     }
// }

// #[derive(Clone, Debug)]
// pub struct PropertyDefinition {
//     base: BaseNode,
//     key: Key,
//     value: Option<Expression>,
//     computed: bool,
//     is_static: bool,
// }

// impl PropertyDefinition {
//     pub fn get_type() -> String {
//         "PropertyDefinition".to_string()
//     }
// }
