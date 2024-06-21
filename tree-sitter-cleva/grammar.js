/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

var { c_grammar, c_PREC} = require('./c_grammar');

var c_exported_grammar = c_grammar;
c_exported_grammar.name = 'cleva';

module.exports = grammar(c_exported_grammar);
