# rust-nbe-for-mltt

This originally started as a Rust port of Danny Gratzer's implementation of
[Normalization by Evaluation for Martin-Löf Type Theory][nbe-for-mltt], but it
has a slightly different architecture and some additional language features.
The algorithm for the insertion and unification of metavariables was partly
taken from Andras Korvacs' [Minimal TT Exampls][minimal-tt-examples] and
[smalltt][smalltt] (although gluing is not yet implemented here).
It will probably become the basis for a new front-end for
[Pikelet](https://github.com/pikelet-lang/pikelet).

In traditional type checking and normalization that uses [DeBruijn indices][de-bruijn-indices],
you are required to shift variable indices whenever you open up binders. This
is extremely expensive, and rules out future optimizations, like [using
visitors][visitors] to reduce the number of intermediate allocations as the AST
is traversed. This implementation avoids these problems by using a "semantic
type checking"  algorithm that uses DeBruijn indices for the core syntax, and
DeBruijn levels in the syntax of the semantic domain.

| Syntax        | Binding method              | Example                         |
|---------------|-----------------------------|---------------------------------|
| Concrete      | Nominal                     | `λz. (λy. y (λx. x)) (λx. z x)` |
| Core          | Nameless (DeBruijn Indices) | `λ . (λ . 0 (λ . 0)) (λ . 1 0)` |
| Domain        | Nameless (DeBruijn Levels)  | `λ . (λ . 1 (λ . 2)) (λ . 0 1)` |

[nbe-for-mltt]: https://github.com/jozefg/nbe-for-mltt
[minimal-tt-examples]: https://github.com/AndrasKovacs/minimal-tt-examples
[smalltt]: https://github.com/AndrasKovacs/smalltt
[de-bruijn-indices]: https://en.wikipedia.org/wiki/De_Bruijn_index
[visitors]: https://github.com/pikelet-lang/pikelet/issues/75

## TODO

- [x] Convert data types to Rust
- [x] Port NbE and bidirectional type checking
- [x] Add a parser for the concrete syntax
- [x] Desugaring of concrete syntax to core syntax
- [x] Resugaring of core syntax to concrete syntax
- [ ] Pretty printing
    - [x] Basic pretty printing
    - [x] Preserve pretty names through type checking and normalization
    - [ ] Unfold metavariables when pretty printing values
    - [ ] Attempt to avoid unfolding variables when pretty printing values
- [x] Add a REPL
- [x] Add span information to ASTs to improve diagnostics
- [ ] Pattern matching elaboration
    - [x] Simple cases
    - [ ] Nested cases
    - [ ] Multiple scrutinees
    - [ ] Lambda case
- [x] Dependent record types
- [x] Primitive operations
- [ ] Unification
    - [x] Basic unification
    - [x] Function eta rules
    - [x] Record eta rules
    - [ ] [Pruning](https://gitter.im/pikelet-lang/Lobby?at=5cd519e60f381d0a768e7811)
    - [ ] [Skolemization](https://gitter.im/pikelet-lang/Lobby?at=5cd129ca6a84d76ed85bbefd)
- [x] Metavariable insertion
- [ ] Integration tests
  - [ ] Parse (pass)
  - [ ] Parse (fail)
  - [x] Elaboration (pass)
  - [ ] Elaboration (fail)
  - [ ] Normalization tests
  - [x] Sample modules
- [ ] Error recovery in:
  - [x] Lexer
  - [ ] Parser
  - [ ] Elaborator
  - [ ] Validator
