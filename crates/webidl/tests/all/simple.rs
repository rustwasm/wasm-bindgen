use super::backend;

assert_parse!(empty, "", backend::ast::Program::default());
