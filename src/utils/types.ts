export type RobEntry = {
    entry_valid: boolean;
    phys_rd: number;
    arch_rd: number;
    commit_ready: boolean;
}

export enum AluCmd {
    ADD,
    SUB,
    XOR,
    OR,
    AND,
    SRL,
    SRA,
    SLL,
    EQ,
    NE,
    LT,
    GE,
    LTU,
    GEU,
    BIT_C,
    SLT,
    SLTU,
    ILL,
}

export enum OpType {
    REG,
    IMM,
}

export type IsqEntry = {
    entry_valid: boolean;
    alu_cmd: AluCmd;
    op1_valid: boolean;
    op1_data: number;
    op2_valid: boolean;
    op2_type: OpType;
    op2_data: number;
    phys_rd: number;
    bank_addr: number;
    rob_addr: number;
}