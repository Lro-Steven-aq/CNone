Program: Program {
    decls: [
        Function(
            FunctionDecl {
                return_type: Int,
                name: "main",
                params: [],
                body: Some(
                    Block {
                        stmts: [
                            VaribleDecl(
                                VaribleDecl {
                                    typ: Int,
                                    name: "kode",
                                    init: Some(
                                        Integer(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            VaribleDecl(
                                VaribleDecl {
                                    typ: Int,
                                    name: "num",
                                    init: Some(
                                        BinaryExp(
                                            Mul,
                                            Integer(
                                                9,
                                            ),
                                            Identifier(
                                                "kode",
                                            ),
                                        ),
                                    ),
                                },
                            ),
                            VaribleDecl(
                                VaribleDecl {
                                    typ: Int,
                                    name: "m",
                                    init: Some(
                                        Integer(
                                            8,
                                        ),
                                    ),
                                },
                            ),
                            VaribleDecl(
                                VaribleDecl {
                                    typ: Short,
                                    name: "bin",
                                    init: Some(
                                        CallFunction(
                                            "func",
                                            [],
                                        ),
                                    ),
                                },
                            ),
                            If(
                                Identifier(
                                    "num",
                                ),
                                Block(
                                    Block {
                                        stmts: [
                                            Expr(
                                                CallFunction(
                                                    "func__",
                                                    [
                                                        UnaryExp(
                                                            AddressOf,
                                                            Identifier(
                                                                "kode",
                                                            ),
                                                        ),
                                                    ],
                                                ),
                                            ),
                                        ],
                                    },
                                ),
                                None,
                            ),
                            Return(
                                Some(
                                    Integer(
                                        0,
                                    ),
                                ),
                            ),
                        ],
                    },
                ),
            },
        ),
    ],
}
