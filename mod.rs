use crate::arena;
use crate::crust::libc::*;
use crate::ir::*;
use crate::lexer::*;
use crate::missingf;
use crate::nob::*;
use crate::targets::TargetAPI;
use core::ffi::*;

pub unsafe fn get_apis(targets: *mut Array<TargetAPI>) {
    da_append(
        targets,
        TargetAPI::V1 {
            name: c!("basm"),
            file_ext: c!(".bm"),
            new,
            build: generate_program,
            run: run_program,
        },
    );
}

struct BasmTarget {
    output: String_Builder,
    cmd: Cmd,
}

pub unsafe fn new(a: *mut arena::Arena, _args: *const [*const c_char]) -> Option<*mut c_void> {
    let gen = arena::alloc_type::<BasmTarget>(a);
    memset(gen as _, 0, size_of::<BasmTarget>());

    Some(gen as *mut c_void)
}

pub unsafe fn load_arg(loc: Loc, arg: Arg, output: *mut String_Builder, data: *const [u8]) {
    match arg {
        Arg::Bogus => unreachable!("bogus-amogus"),
        Arg::AutoVar(index) => {
            sb_appendf(output, c!("    push AV_%zu\n"), index);
            sb_appendf(output, c!("    read64u\n"));
        }
        Arg::Deref(..) => missingf!(loc, c!("Deref\n")),
        Arg::RefAutoVar(..) => missingf!(loc, c!("RefAutoVar\n")),
        Arg::RefExternal(..) => missingf!(loc, c!("RefExternal\n")),
        Arg::External(..) => missingf!(loc, c!("External\n")),
        Arg::Literal(literal) => {
            sb_appendf(output, c!("    push %zu\n"), literal);
        }
        Arg::DataOffset(off) => {
            sb_appendf(output, c!("    push \""));
            let mut i = 0;
            while (*data)[off + i] != 0 {
                if (*data)[off + i] == b'\n' {
                    sb_appendf(output, c!("\\n"));
                } else {
                    sb_appendf(output, c!("%c"), (*data)[off + i] as c_int);
                }
                i += 1;
            }
            sb_appendf(output, c!("%c\n"), '"');
        }
    }
}

pub unsafe fn call_arg(
    loc: Loc,
    fun: Arg,
    out: *mut String_Builder,
    _arity: usize,
    extrns: *const [*const i8],
) {
    match fun {
        Arg::Bogus => unreachable!("bogus-amogus"),
        Arg::AutoVar(..) => missingf!(loc, c!("AutoVar\n")),
        Arg::Deref(..) => missingf!(loc, c!("Deref\n")),
        Arg::RefAutoVar(..) => missingf!(loc, c!("RefAutoVar\n")),
        Arg::RefExternal(..) => missingf!(loc, c!("RefExternal\n")),
        Arg::External(name) => {
            let mut is_extrn = false;
            for i in 0..extrns.len() {
                let it = (*extrns)[i];
                if strcmp(it, name) == 0 {
                    sb_appendf(out, c!("    native %s\n"), name);
                    is_extrn = true;
                    break;
                }
            }
            if !is_extrn {
                sb_appendf(out, c!("    call %s\n"), name);
            }
        }
        Arg::Literal(..) => missingf!(loc, c!("Literal\n")),
        Arg::DataOffset(..) => missingf!(loc, c!("DataOffset\n")),
    }
}

pub unsafe fn generate_function(
    func: Func,
    output: *mut String_Builder,
    data: *const [u8],
    extrns: *const [*const i8],
) {
    if strcmp(func.name, c!("main")) == 0 {
        sb_appendf(output, c!("%%entry "));
    }

    sb_appendf(output, c!("%s:\n%%scope\n"), func.name);

    if func.auto_vars_count > 0 {
        for i in 0..func.auto_vars_count {
            sb_appendf(output, c!("    %%const AV_%zu = byte_array(8, 0)\n"), i);
        }
    }
    sb_appendf(output, c!("\n"));

    for i in 0..func.body.count {
        let op = *func.body.items.add(i);
        match op.opcode {
            Op::Bogus => unreachable!("bogus-amogus"),
            Op::UnaryNot { .. } => missingf!(op.loc, c!("Op::UnaryNot\n")),
            Op::Negate { .. } => missingf!(op.loc, c!("Op::Negate\n")),
            Op::Asm { stmts } => {
                for i in 0..stmts.count {
                    let stmt = *stmts.items.add(i);
                    sb_appendf(output, c!("    %s\n"), stmt.line);
                }
            }
            Op::Binop {
                binop, lhs, rhs, ..
            } => {
                load_arg(op.loc, lhs, output, data);
                load_arg(op.loc, rhs, output, data);
                match binop {
                    Binop::Plus => missingf!(op.loc, c!("Binop::Add\n")),
                    Binop::Minus => missingf!(op.loc, c!("Binop::Minus\n")),
                    Binop::Mult => missingf!(op.loc, c!("Binop::Mult\n")),
                    Binop::Div => missingf!(op.loc, c!("Binop::Div\n")),
                    Binop::Mod => missingf!(op.loc, c!("Binop::Rem\n")),
                    Binop::Equal => missingf!(op.loc, c!("Binop::Equal\n")),
                    Binop::NotEqual => missingf!(op.loc, c!("Binop::NotEqual\n")),
                    Binop::Less => missingf!(op.loc, c!("Binop::Less\n")),
                    Binop::LessEqual => missingf!(op.loc, c!("Binop::LessEqual\n")),
                    Binop::Greater => missingf!(op.loc, c!("Binop::Greater\n")),
                    Binop::GreaterEqual => missingf!(op.loc, c!("Binop::GreaterEqual\n")),
                    Binop::BitOr => missingf!(op.loc, c!("Binop::BitOr\n")),
                    Binop::BitAnd => missingf!(op.loc, c!("Binop::BitAnd\n")),
                    Binop::BitShl => missingf!(op.loc, c!("Binop::BitShl\n")),
                    Binop::BitShr => missingf!(op.loc, c!("Binop::BitShr\n")),
                };
            }
            Op::Index { .. } => missingf!(op.loc, c!("Op::Index\n")),
            Op::AutoAssign { index, arg } => {
                sb_appendf(output, c!("    push AV_%zu\n"), index);
                load_arg(op.loc, arg, output, data);
                sb_appendf(output, c!("    write64\n"));
            }
            Op::ExternalAssign { .. } => missingf!(op.loc, c!("Op::ExternalAssign\n")),
            Op::Store { .. } => missingf!(op.loc, c!("Op::Store\n")),
            Op::Funcall { fun, args, .. } => {
                for i in 0..args.count {
                    load_arg(op.loc, *args.items.add(i), output, data);
                }
                call_arg(op.loc, fun, output, args.count, extrns);
            }
            Op::Label { .. } => missingf!(op.loc, c!("Op::Label\n")),
            Op::JmpLabel { .. } => missingf!(op.loc, c!("Op::JmpLabel\n")),
            Op::JmpIfNotLabel { .. } => missingf!(op.loc, c!("Op::JmpNotLabel\n")),
            Op::Return { .. } => missingf!(op.loc, c!("Op::Return\n")),
        }
    }

    sb_appendf(output, c!("%%end\n"));

    if strcmp(func.name, c!("main")) == 0 {
        sb_appendf(output, c!("    halt\n"));
    } else {
        sb_appendf(output, c!("    ret\n"));
    }
}

pub unsafe fn generate_functions(
    funcs: *const [Func],
    output: *mut String_Builder,
    data: *const [u8],
    extrns: *const [*const i8],
) {
    for i in 0..funcs.len() {
        generate_function((*funcs)[i], output, data, extrns);
    }
}

pub unsafe fn generate_program(
    genptr: *mut c_void,
    program: *const Program,
    program_path: *const c_char,
    garbage_base: *const c_char,
    _nostdlib: bool,
    _debug: bool,
) -> Option<()> {
    let gen = genptr as *mut BasmTarget;
    let output = &mut (*gen).output;

    sb_appendf(output, c!("%%const WRITE_BUFFER = byte_array(128, 0)\n"));

    let extrns = da_slice((*program).extrns);
    for i in 0..extrns.len() {
        let ext = (*extrns)[i];
        sb_appendf(output, c!("%%native %s\n"), ext);
    }

    generate_functions(
        da_slice((*program).funcs),
        output,
        da_slice((*program).data),
        extrns,
    );

    let output_basm_path = temp_sprintf(c!("%s.basm"), garbage_base);
    write_entire_file(
        output_basm_path,
        (*output).items as *const c_void,
        (*output).count,
    )?;

    log(Log_Level::INFO, c!("generate %s"), output_basm_path);

    let cmd = &mut (*gen).cmd;

    cmd_append! {
        cmd, c!("basm"), output_basm_path, c!("-o"), program_path
    }

    if !cmd_run_sync_and_reset(cmd) {
        return None;
    }

    Some(())
}

pub unsafe fn run_program(
    genptr: *mut c_void,
    program_path: *const c_char,
    run_args: *const [*const c_char],
) -> Option<()> {
    let gen = genptr as *mut BasmTarget;
    let cmd = &mut (*gen).cmd;

    cmd_append! {
        cmd, c!("bme"), program_path, c!("-n"), c!("./build/lib.so"),
    }

    da_append_many(cmd, run_args);
    if !cmd_run_sync_and_reset(cmd) {
        return None;
    }

    Some(())
}
