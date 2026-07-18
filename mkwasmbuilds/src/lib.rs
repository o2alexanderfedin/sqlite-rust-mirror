const F_ESM: i32 = 1;
const F_BUNDLER_FRIENDLY: i32 = 2;
const F_UNSUPPORTED: i32 = 4;
const F_NOT_IN_ALL: i32 = 8;
const F_64BIT: i32 = 16;
const F_NODEJS: i32 = 32;
const F_WASMFS: i32 = 64;
const CP_JS: i32 = 1073741824;
const CP_WASM: i32 = -2147483648;
const CP_ALL: i32 = -1073741824;
#[repr(C)]
#[derive(Copy, Clone)]
struct BuildDef {
    z_base_name: *const i8,
    z_emo: *const i8,
    z_dot_wasm: *const i8,
    z_cmpp_d: *const i8,
    z_emcc: *const i8,
    z_emcc_extra: *const i8,
    z_deps: *const i8,
    z_env: *const i8,
    z_if_cond: *const i8,
    flags: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct BuildDefs {
    vanilla: BuildDef,
    vanilla64: BuildDef,
    esm: BuildDef,
    esm64: BuildDef,
    bundler: BuildDef,
    bundler64: BuildDef,
    speedtest1: BuildDef,
    speedtest164: BuildDef,
    node: BuildDef,
    node64: BuildDef,
    wasmfs: BuildDef,
}
static mut o_build_defs: BuildDefs =
    BuildDefs {
        vanilla: BuildDef {
            z_base_name: c"sqlite3".as_ptr() as *const i8,
            z_emo: c"\u{1f366}".as_ptr() as *const i8,
            z_dot_wasm: core::ptr::null(),
            z_cmpp_d: core::ptr::null(),
            z_emcc: core::ptr::null(),
            z_emcc_extra: core::ptr::null(),
            z_deps: core::ptr::null(),
            z_env: c"web,worker".as_ptr() as *const i8,
            z_if_cond: core::ptr::null(),
            flags: CP_ALL,
        },
        vanilla64: BuildDef {
            z_base_name: c"sqlite3-64bit".as_ptr() as *const i8,
            z_emo: c"\u{1f368}".as_ptr() as *const i8,
            z_dot_wasm: core::ptr::null(),
            z_cmpp_d: core::ptr::null(),
            z_emcc: core::ptr::null(),
            z_emcc_extra: c"-sMEMORY64=1 -sWASM_BIGINT=1".as_ptr() as
                *const i8,
            z_deps: core::ptr::null(),
            z_env: core::ptr::null(),
            z_if_cond: core::ptr::null(),
            flags: CP_ALL | F_64BIT | F_NOT_IN_ALL,
        },
        esm: BuildDef {
            z_base_name: c"sqlite3".as_ptr() as *const i8,
            z_emo: c"\u{1f36c}".as_ptr() as *const i8,
            z_dot_wasm: core::ptr::null(),
            z_cmpp_d: c"-Dtarget:es6-module".as_ptr() as *const i8,
            z_emcc: core::ptr::null(),
            z_emcc_extra: core::ptr::null(),
            z_deps: core::ptr::null(),
            z_env: core::ptr::null(),
            z_if_cond: core::ptr::null(),
            flags: CP_JS | F_ESM,
        },
        esm64: BuildDef {
            z_base_name: c"sqlite3-64bit".as_ptr() as *const i8,
            z_emo: c"\u{1f36b}".as_ptr() as *const i8,
            z_dot_wasm: core::ptr::null(),
            z_cmpp_d: c"-Dtarget:es6-module".as_ptr() as *const i8,
            z_emcc: core::ptr::null(),
            z_emcc_extra: c"-sMEMORY64=1 -sWASM_BIGINT=1".as_ptr() as
                *const i8,
            z_deps: core::ptr::null(),
            z_env: core::ptr::null(),
            z_if_cond: core::ptr::null(),
            flags: CP_JS | F_ESM | F_64BIT | F_NOT_IN_ALL,
        },
        bundler: BuildDef {
            z_base_name: c"sqlite3-bundler-friendly".as_ptr() as *const i8,
            z_emo: c"\u{1f45b}".as_ptr() as *const i8,
            z_dot_wasm: c"sqlite3".as_ptr() as *const i8,
            z_cmpp_d: c"$(c-pp.D.esm) -Dtarget:es6-bundler-friendly".as_ptr()
                as *const i8,
            z_emcc: core::ptr::null(),
            z_emcc_extra: core::ptr::null(),
            z_deps: core::ptr::null(),
            z_env: core::ptr::null(),
            z_if_cond: core::ptr::null(),
            flags: CP_JS | F_BUNDLER_FRIENDLY | F_ESM | F_NOT_IN_ALL,
        },
        bundler64: BuildDef {
            z_base_name: c"sqlite3-bundler-friendly-64bit".as_ptr() as
                *const i8,
            z_emo: c"\u{1f4e6}".as_ptr() as *const i8,
            z_dot_wasm: c"sqlite3-64bit".as_ptr() as *const i8,
            z_cmpp_d: c"$(c-pp.D.bundler)".as_ptr() as *const i8,
            z_emcc: core::ptr::null(),
            z_emcc_extra: c"-sMEMORY64=1".as_ptr() as *const i8,
            z_deps: core::ptr::null(),
            z_env: core::ptr::null(),
            z_if_cond: core::ptr::null(),
            flags: CP_JS | F_ESM | F_BUNDLER_FRIENDLY | F_64BIT | F_NOT_IN_ALL,
        },
        speedtest1: BuildDef {
            z_base_name: c"speedtest1".as_ptr() as *const i8,
            z_emo: c"\u{1f6fc}".as_ptr() as *const i8,
            z_dot_wasm: core::ptr::null(),
            z_cmpp_d: core::ptr::null(),
            z_emcc: c"$(emcc.speedtest1) $(emcc.speedtest1.common) $(pre-post.speedtest1.flags) $(cflags.common) -DSQLITE_SPEEDTEST1_WASM $(SQLITE_OPT) -USQLITE_WASM_BARE_BONES -USQLITE_C -DSQLITE_C=$(sqlite3.c) $(speedtest1.exit-runtime0) $(speedtest1.c.in) -lm".as_ptr()
                as *const i8,
            z_emcc_extra: core::ptr::null(),
            z_deps: c"$(speedtest1.c.in) $(EXPORTED_FUNCTIONS.speedtest1)".as_ptr()
                as *const i8,
            z_env: core::ptr::null(),
            z_if_cond: core::ptr::null(),
            flags: CP_ALL,
        },
        speedtest164: BuildDef {
            z_base_name: c"speedtest1-64bit".as_ptr() as *const i8,
            z_emo: c"\u{1f6fc}64".as_ptr() as *const i8,
            z_dot_wasm: core::ptr::null(),
            z_cmpp_d: core::ptr::null(),
            z_emcc: c"$(emcc.speedtest1) $(emcc.speedtest1.common) -sMEMORY64=1 -sWASM_BIGINT=1 $(pre-post.speedtest164.flags) $(cflags.common) -DSQLITE_SPEEDTEST1_WASM $(SQLITE_OPT) -USQLITE_WASM_BARE_BONES -USQLITE_C -DSQLITE_C=$(sqlite3.c) $(speedtest1.exit-runtime0) $(speedtest1.c.in) -lm".as_ptr()
                as *const i8,
            z_emcc_extra: core::ptr::null(),
            z_deps: c"$(speedtest1.c.in) $(EXPORTED_FUNCTIONS.speedtest1)".as_ptr()
                as *const i8,
            z_env: core::ptr::null(),
            z_if_cond: core::ptr::null(),
            flags: CP_ALL | F_NOT_IN_ALL,
        },
        node: BuildDef {
            z_base_name: c"sqlite3-node".as_ptr() as *const i8,
            z_emo: c"\u{1f35f}".as_ptr() as *const i8,
            z_dot_wasm: c"sqlite3".as_ptr() as *const i8,
            z_cmpp_d: c"-Dtarget:node $(c-pp.D.bundler)".as_ptr() as
                *const i8,
            z_emcc: core::ptr::null(),
            z_emcc_extra: core::ptr::null(),
            z_deps: core::ptr::null(),
            z_env: c"node".as_ptr() as *const i8,
            z_if_cond: core::ptr::null(),
            flags: CP_JS | F_UNSUPPORTED | F_ESM | F_NODEJS,
        },
        node64: BuildDef {
            z_base_name: c"sqlite3-node-64bit".as_ptr() as *const i8,
            z_emo: c"\u{1f354}".as_ptr() as *const i8,
            z_dot_wasm: c"sqlite3-64bit".as_ptr() as *const i8,
            z_cmpp_d: c"-Dtarget:node $(c-pp.D.bundler)".as_ptr() as
                *const i8,
            z_emcc: core::ptr::null(),
            z_emcc_extra: core::ptr::null(),
            z_deps: core::ptr::null(),
            z_env: c"node".as_ptr() as *const i8,
            z_if_cond: core::ptr::null(),
            flags: CP_JS | F_UNSUPPORTED | F_ESM | F_NODEJS | F_64BIT,
        },
        wasmfs: BuildDef {
            z_base_name: c"sqlite3-wasmfs".as_ptr() as *const i8,
            z_emo: c"\u{1f4bf}".as_ptr() as *const i8,
            z_dot_wasm: core::ptr::null(),
            z_cmpp_d: c"$(c-pp.D.bundler) -Dwasmfs".as_ptr() as *const i8,
            z_emcc: core::ptr::null(),
            z_emcc_extra: c"-sEXPORT_ES6 -sUSE_ES6_IMPORT_META -sUSE_CLOSURE_COMPILER=0 -pthread -sWASMFS -sPTHREAD_POOL_SIZE=1 -sERROR_ON_UNDEFINED_SYMBOLS=0 -sLLD_REPORT_UNDEFINED -DSQLITE_ENABLE_WASMFS".as_ptr()
                as *const i8,
            z_deps: core::ptr::null(),
            z_env: core::ptr::null(),
            z_if_cond: c"ifeq (1,$(wasmfs.enable))".as_ptr() as *const i8,
            flags: CP_ALL | F_UNSUPPORTED | F_WASMFS | F_ESM,
        },
    };
extern "C" fn mk_prologue() -> () {
    let a_required_vars: [*const i8; 7] =
        [c"dir.top".as_ptr() as *const i8, c"dir.api".as_ptr() as *const i8,
                c"dir.dout".as_ptr() as *const i8,
                c"dir.tmp".as_ptr() as *const i8,
                c"dir.fiddle".as_ptr() as *const i8,
                c"dir.fiddle.debug".as_ptr() as *const i8, core::ptr::null()];
    let mut z_var: *const i8 = core::ptr::null();
    let mut i: i32 = 0;
    unsafe {
        puts(c"\n########################################################################\n# Build setup sanity checks...".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        i = 0;
        '__b0: loop {
            if !(!({ z_var = a_required_vars[i as usize]; z_var }).is_null())
                {
                break '__b0;
            }
            '__c0: loop {
                unsafe {
                    printf(c"ifeq (,$(%s))\n".as_ptr() as *mut i8 as *const i8,
                        z_var)
                };
                unsafe {
                    printf(c"  $(error build process error: expecting make var $$(%s) to have been set up by now)\n".as_ptr()
                                as *mut i8 as *const i8, z_var)
                };
                unsafe { puts(c"endif".as_ptr() as *mut i8 as *const i8) };
                break '__c0;
            }
            { let __p = &mut i; *__p += 1; *__p };
        }
    }
    unsafe {
        puts(c"define label.unsupported-build\n$(emo.fire)$(emo.fire)$(emo.fire)Unsupported build: use at your own risk!\nendef".as_ptr()
                    as *mut i8 as *const i8)
    };
    unsafe {
        puts(c"\n########################################################################\nb.call.wasm-strip = echo \'[$(emo.b.$(1)) $(out.$(1).wasm)] $(emo.strip) wasm-strip\'; $(bin.wasm-strip) $(out.$(1).wasm)\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    unsafe {
        printf(c"\n########################################################################\ndefine b.do.emcc\n$(bin.emcc) -o $@ $(emcc_opt_full) $(emcc.flags) $(emcc.jsflags) -sENVIRONMENT=$(emcc.environment.$(1))  $(pre-post.$(1).flags)  $(emcc.flags.$(1))  $(cflags.common) $(cflags.$(1))  $(SQLITE_OPT)  $(cflags.wasm_extra_init) $(sqlite3-wasm.c.in)\nendef\n".as_ptr()
                    as *mut i8 as *const i8)
    };
    {
        let z_opt_flags: *const i8 =
            c"--enable-bulk-memory-opt --all-features --post-emscripten --strip-debug --local-cse ".as_ptr()
                    as *mut i8 as *const i8;
        unsafe {
            puts(c"\n########################################################################\n# post-compilation WASM file optimization".as_ptr()
                        as *mut i8 as *const i8)
        };
        unsafe {
            puts(c"ifeq (,$(bin.wasm-opt))".as_ptr() as *mut i8 as *const i8)
        };
        {
            unsafe {
                puts(c"b.do.wasm-opt = echo \'$(logtag.$(1)) wasm-opt not available\'".as_ptr()
                            as *mut i8 as *const i8)
            };
        }
        unsafe { puts(c"else".as_ptr() as *mut i8 as *const i8) };
        {
            unsafe {
                puts(c"define b.do.wasm-opt".as_ptr() as *mut i8 as *const i8)
            };
            unsafe {
                printf(c"echo \'[$(emo.b.$(1)) $(out.$(1).wasm)] $(emo.wasm-opt) $(bin.wasm-opt)\';\\\n\ttmpfile=$(dir.dout.$(1))/wasm-opt-tmp.$(1).wasm; \\\n\trm -f $$tmpfile; \\\n\tif $(bin.wasm-opt) $(out.$(1).wasm) -o $$tmpfile \\\n\t\t%s; then \\\n\t\tmv $$tmpfile $(out.$(1).wasm); \\\n\telse \\\n\t\trm -f $$tmpfile; \\\n\t\techo \'$(logtag.$(1)) $(emo.fire) ignoring wasm-opt failure\'; \\\n\tfi\n".as_ptr()
                            as *mut i8 as *const i8, z_opt_flags)
            };
            unsafe { puts(c"endef".as_ptr() as *mut i8 as *const i8) };
        }
        unsafe { puts(c"endif".as_ptr() as *mut i8 as *const i8) };
    }
    unsafe { puts(c"more: all".as_ptr() as *mut i8 as *const i8) };
}
extern "C" fn build_def_jsext(p_b_1: &BuildDef) -> *const i8 {
    return if F_ESM & (*p_b_1).flags as i32 != 0 {
                c".mjs".as_ptr() as *mut i8
            } else { c".js".as_ptr() as *mut i8 } as *const i8;
}
extern "C" fn build_def_basename(p_b_1: &BuildDef) -> *const i8 {
    unsafe {
        return if !((*p_b_1).z_base_name).is_null() {
                (*p_b_1).z_base_name
            } else { o_build_defs.vanilla.z_base_name };
    }
}
extern "C" fn mk_pre_post(z_build_name_1: *const i8, p_b_1: *const BuildDef)
    -> () {
    let z_base_name: *const i8 =
        if !(p_b_1).is_null() {
            build_def_basename(unsafe { &*p_b_1 })
        } else { core::ptr::null() };
    if (z_build_name_1).is_null() as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(c"mk_pre_post".as_ptr() as *const i8,
                c"mkwasmbuilds.c".as_ptr() as *mut i8 as *const i8, 607,
                c"zBuildName".as_ptr() as *mut i8 as *const i8)
        }
    } else { { let _ = 0; } };
    unsafe {
        printf(c"%s# Begin --pre/--post flags for %s\n".as_ptr() as *mut i8 as
                *const i8,
            c"\n########################################################################\n".as_ptr()
                as *mut i8, z_build_name_1)
    };
    unsafe { puts(c"# --pre-js=...".as_ptr() as *mut i8 as *const i8) };
    unsafe {
        printf(c"pre-js.%s.js = $(dir.tmp)/pre-js.%s.js\n".as_ptr() as *mut i8
                as *const i8, z_build_name_1, z_build_name_1)
    };
    if 0 == 0 || (p_b_1).is_null() as i32 != 0 {
        unsafe {
            printf(c"$(eval $(call b.c-pp.target,%s,$(pre-js.in.js),$(pre-js.%s.js),$(c-pp.D.%s)))".as_ptr()
                        as *mut i8 as *const i8, z_build_name_1, z_build_name_1,
                z_build_name_1)
        };
    } else {
        let z_wasm_file: *const i8 =
            if !(unsafe { (*p_b_1).z_dot_wasm }).is_null() {
                unsafe { (*p_b_1).z_dot_wasm }
            } else { unsafe { (*p_b_1).z_base_name } };
        unsafe {
            printf(c"$(pre-js.%s.js): $(pre-js.in.js) $(bin.c-pp) $(MAKEFILE_LIST)".as_ptr()
                        as *mut i8 as *const i8, z_build_name_1)
        };
        if !(unsafe { (*p_b_1).z_dot_wasm }).is_null() {
            unsafe {
                printf(c" $(dir.dout)/%s.wasm".as_ptr() as *mut i8 as
                        *const i8, unsafe { (*p_b_1).z_dot_wasm })
            };
        }
        unsafe { puts(c"".as_ptr() as *mut i8 as *const i8) };
        unsafe {
            printf(c"\t@$(call b.mkdir@); $(call b.c-pp.shcmd,%s,$(pre-js.in.js),$(pre-js.%s.js),$(c-pp.D.%s))\n".as_ptr()
                        as *mut i8 as *const i8, z_build_name_1, z_build_name_1,
                z_build_name_1)
        };
    }
    unsafe { puts(c"\n# --post-js=...".as_ptr() as *mut i8 as *const i8) };
    unsafe {
        printf(c"post-js.%s.js = $(dir.tmp)/post-js.%s.js\n".as_ptr() as
                    *mut i8 as *const i8, z_build_name_1, z_build_name_1)
    };
    unsafe {
        printf(c"post-js.%s.in = $(dir.api)/post-js-header.js $(sqlite3-api.%s.js) $(dir.api)/post-js-footer.js\n".as_ptr()
                    as *mut i8 as *const i8, z_build_name_1, z_build_name_1)
    };
    unsafe {
        printf(c"$(eval $(call b.c-pp.target,%s,$(post-js.%s.in),$(post-js.%s.js),$(c-pp.D.%s)))\n".as_ptr()
                    as *mut i8 as *const i8, z_build_name_1, z_build_name_1,
            z_build_name_1, z_build_name_1)
    };
    unsafe {
        printf(c"$(post-js.%s.js): $(post-js.%s.in) $(bin.c-pp)\n".as_ptr() as
                    *mut i8 as *const i8, z_build_name_1, z_build_name_1)
    };
    unsafe {
        puts(c"\n# --extern-post-js=...".as_ptr() as *mut i8 as *const i8)
    };
    unsafe {
        printf(c"extern-post-js.%s.js = $(dir.tmp)/extern-post-js.%s.js\n".as_ptr()
                    as *mut i8 as *const i8, z_build_name_1, z_build_name_1)
    };
    if 0 != 0 && !(z_base_name).is_null() {
        unsafe {
            printf(c"$(eval $(call b.c-pp.target,%s,$(extern-post-js.in.js),$(extern-post-js.%s.js),$(c-pp.D.%s) --@policy=error -Dsqlite3.wasm=%s.wasm))".as_ptr()
                        as *mut i8 as *const i8, z_build_name_1, z_build_name_1,
                z_build_name_1, z_base_name)
        };
    } else {
        unsafe {
            printf(c"$(eval $(call b.c-pp.target,%s,$(extern-post-js.in.js),$(extern-post-js.%s.js),$(c-pp.D.%s)))".as_ptr()
                        as *mut i8 as *const i8, z_build_name_1, z_build_name_1,
                z_build_name_1)
        };
    }
    unsafe {
        puts(c"\n# --pre/post misc...".as_ptr() as *mut i8 as *const i8)
    };
    unsafe {
        printf(c"pre-post.%s.flags = --extern-pre-js=$(sqlite3-license-version.js) --pre-js=$(pre-js.%s.js) --post-js=$(post-js.%s.js) --extern-post-js=$(extern-post-js.%s.js)\n".as_ptr()
                    as *mut i8 as *const i8, z_build_name_1, z_build_name_1,
            z_build_name_1, z_build_name_1)
    };
    unsafe {
        printf(c"pre-post.%s.deps = $(pre-post-jses.common.deps) $(post-js.%s.js) $(extern-post-js.%s.js) $(dir.tmp)/pre-js.%s.js\n".as_ptr()
                    as *mut i8 as *const i8, z_build_name_1, z_build_name_1,
            z_build_name_1, z_build_name_1)
    };
    unsafe {
        printf(c"# End --pre/--post flags for %s%s".as_ptr() as *mut i8 as
                *const i8, z_build_name_1,
            c"\n########################################################################\n".as_ptr()
                as *mut i8)
    };
}
extern "C" fn emit_compile_start(z_build_name_1: *const i8) -> () {
    unsafe {
        printf(c"\t@$(call b.mkdir@); $(call b.echo,%s,$(emo.compile) building ...)\n".as_ptr()
                    as *mut i8 as *const i8, z_build_name_1)
    };
}
extern "C" fn emit_logtag(z_build_name_1: *const i8) -> () {
    unsafe {
        printf(c"logtag.%s ?= [$(emo.b.%s)$(if $@, $@,)]:\n".as_ptr() as
                    *mut i8 as *const i8, z_build_name_1, z_build_name_1)
    };
    unsafe {
        printf(c"$(info $(logtag.%s) Setting up target b-%s)\n".as_ptr() as
                    *mut i8 as *const i8, z_build_name_1, z_build_name_1)
    };
}
extern "C" fn emit_api_js(z_build_name_1: *const i8) -> () {
    unsafe {
        printf(c"sqlite3-api.%s.js = $(dir.tmp)/sqlite3-api.%s.js\n".as_ptr()
                    as *mut i8 as *const i8, z_build_name_1, z_build_name_1)
    };
    unsafe {
        printf(c"$(eval $(call b.c-pp.target,%s,$(sqlite3-api.jses),$(sqlite3-api.%s.js),$(c-pp.D.%s)))\n".as_ptr()
                    as *mut i8 as *const i8, z_build_name_1, z_build_name_1,
            z_build_name_1)
    };
    unsafe {
        printf(c"$(out.%s.js): $(sqlite3-api.%s.js)\n".as_ptr() as *mut i8 as
                *const i8, z_build_name_1, z_build_name_1)
    };
    unsafe {
        printf(c"$(sqlite3-api.%s.js): $(dir.api)/opfs-common-shared.c-pp.js $(dir.api)/opfs-common-inline.c-pp.js\n".as_ptr()
                    as *mut i8 as *const i8, z_build_name_1)
    };
}
extern "C" fn mk_lib_mode(z_build_name_1: *const i8, p_b_1: *const BuildDef)
    -> () {
    unsafe {
        let z_js_ext: *const i8 = build_def_jsext(unsafe { &*p_b_1 });
        let z_base_name: *const i8 = build_def_basename(unsafe { &*p_b_1 });
        if (o_build_defs.vanilla.z_env).is_null() as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"mk_lib_mode".as_ptr() as *const i8,
                    c"mkwasmbuilds.c".as_ptr() as *mut i8 as *const i8, 765,
                    c"oBuildDefs.vanilla.zEnv".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        if (z_base_name).is_null() as i32 as i64 != 0 {
            unsafe {
                __assert_rtn(c"mk_lib_mode".as_ptr() as *const i8,
                    c"mkwasmbuilds.c".as_ptr() as *mut i8 as *const i8, 766,
                    c"zBaseName".as_ptr() as *mut i8 as *const i8)
            }
        } else { { let _ = 0; } };
        unsafe {
            printf(c"%s# Begin build [%s%s]. flags=0x%02x\n".as_ptr() as
                        *mut i8 as *const i8,
                c"\n########################################################################\n".as_ptr()
                    as *mut i8, unsafe { (*p_b_1).z_emo }, z_build_name_1,
                unsafe { (*p_b_1).flags })
        };
        unsafe {
            printf(c"# zCmppD=%s\n# zBaseName=%s\n".as_ptr() as *mut i8 as
                    *const i8,
                if !(unsafe { (*p_b_1).z_cmpp_d }).is_null() {
                    unsafe { (*p_b_1).z_cmpp_d }
                } else { c"".as_ptr() as *mut i8 as *const i8 }, z_base_name)
        };
        unsafe {
            printf(c"b.names += %s\nemo.b.%s = %s\n".as_ptr() as *mut i8 as
                    *const i8, z_build_name_1, z_build_name_1,
                unsafe { (*p_b_1).z_emo })
        };
        emit_logtag(z_build_name_1);
        if !(unsafe { (*p_b_1).z_if_cond }).is_null() {
            unsafe {
                printf(c"%s\n".as_ptr() as *mut i8 as *const i8,
                    unsafe { (*p_b_1).z_if_cond })
            };
        }
        unsafe {
            printf(c"dir.dout.%s ?= $(dir.dout)/%s\n".as_ptr() as *mut i8 as
                    *const i8, z_build_name_1, z_build_name_1)
        };
        unsafe {
            printf(c"out.%s.base ?= $(dir.dout.%s)/%s\n".as_ptr() as *mut i8
                    as *const i8, z_build_name_1, z_build_name_1, z_base_name)
        };
        unsafe {
            printf(c"out.%s.js ?= $(dir.dout.%s)/%s%s\n".as_ptr() as *mut i8
                    as *const i8, z_build_name_1, z_build_name_1, z_base_name,
                z_js_ext)
        };
        unsafe {
            printf(c"out.%s.wasm ?= $(dir.dout.%s)/%s.wasm\n".as_ptr() as
                        *mut i8 as *const i8, z_build_name_1, z_build_name_1,
                z_base_name)
        };
        unsafe {
            printf(c"dir.dout.%s ?= $(dir.dout)/%s\n".as_ptr() as *mut i8 as
                    *const i8, z_build_name_1, z_build_name_1)
        };
        unsafe {
            printf(c"c-pp.D.%s ?= %s\n".as_ptr() as *mut i8 as *const i8,
                z_build_name_1,
                if !(unsafe { (*p_b_1).z_cmpp_d }).is_null() {
                    unsafe { (*p_b_1).z_cmpp_d }
                } else { c"".as_ptr() as *mut i8 as *const i8 })
        };
        if unsafe { (*p_b_1).flags } & F_64BIT != 0 {
            unsafe {
                printf(c"c-pp.D.%s += $(c-pp.D.64bit)\n".as_ptr() as *mut i8
                        as *const i8, z_build_name_1)
            };
        }
        if unsafe { (*p_b_1).flags } & F_UNSUPPORTED != 0 {
            unsafe {
                printf(c"c-pp.D.%s += -Dunsupported-build\n".as_ptr() as
                            *mut i8 as *const i8, z_build_name_1)
            };
        }
        unsafe {
            printf(c"emcc.environment.%s ?= %s\n".as_ptr() as *mut i8 as
                    *const i8, z_build_name_1,
                if !(unsafe { (*p_b_1).z_env }).is_null() {
                    unsafe { (*p_b_1).z_env }
                } else { o_build_defs.vanilla.z_env })
        };
        if !(unsafe { (*p_b_1).z_emcc_extra }).is_null() {
            unsafe {
                printf(c"emcc.flags.%s = %s\n".as_ptr() as *mut i8 as
                        *const i8, z_build_name_1, unsafe { (*p_b_1).z_emcc_extra })
            };
        }
        if !(unsafe { (*p_b_1).z_deps }).is_null() {
            unsafe {
                printf(c"deps.%s += %s\n".as_ptr() as *mut i8 as *const i8,
                    z_build_name_1, unsafe { (*p_b_1).z_deps })
            };
        }
        emit_api_js(z_build_name_1);
        mk_pre_post(z_build_name_1, p_b_1);
        {
            unsafe {
                printf(c"\n########################################################################\n$(out.%s.js): $(MAKEFILE_LIST) $(sqlite3-wasm.c.in) $(EXPORTED_FUNCTIONS.api) $(deps.%s) $(bin.mkwb) $(pre-post.%s.deps)\n".as_ptr()
                            as *mut i8 as *const i8, z_build_name_1, z_build_name_1,
                    z_build_name_1)
            };
            emit_compile_start(z_build_name_1);
            if F_UNSUPPORTED & unsafe { (*p_b_1).flags } as i32 != 0 {
                unsafe {
                    printf(c"\t@echo \'$(logtag.%s) $(label.unsupported-build)\'\n".as_ptr()
                                as *mut i8 as *const i8, z_build_name_1)
                };
            }
            {
                unsafe {
                    printf(c"\t$(b.cmd@)$(bin.emcc) -o $@ ".as_ptr() as *mut i8
                            as *const i8)
                };
                if !(unsafe { (*p_b_1).z_emcc }).is_null() {
                    unsafe {
                        printf(c"%s $(emcc.flags.%s)\n".as_ptr() as *mut i8 as
                                *const i8, unsafe { (*p_b_1).z_emcc }, z_build_name_1)
                    };
                } else {
                    unsafe {
                        printf(c"$(emcc_opt_full) $(emcc.flags) $(emcc.jsflags) -sENVIRONMENT=$(emcc.environment.%s) $(pre-post.%s.flags) $(emcc.flags.%s) $(cflags.common) $(cflags.%s) $(SQLITE_OPT) $(cflags.wasm_extra_init) $(sqlite3-wasm.c.in)\n".as_ptr()
                                    as *mut i8 as *const i8, z_build_name_1, z_build_name_1,
                            z_build_name_1, z_build_name_1)
                    };
                }
            }
            {
                unsafe {
                    printf(c"\t@if grep -e \'^ *importScripts(\' $@; then echo \'$(logtag.%s) $(emo.bug)$(emo.fire): bug fixed in 65798c09a00662a3 has re-appeared\'; exit 1; fi;\n".as_ptr()
                                as *mut i8 as *const i8, z_build_name_1)
                };
                if F_ESM & unsafe { (*p_b_1).flags } as i32 != 0 ||
                        F_NODEJS & unsafe { (*p_b_1).flags } as i32 != 0 {
                    unsafe {
                        printf(c"\t@$(call b.call.patch-export-default,1,%d,$(logtag.%s))\n".as_ptr()
                                    as *mut i8 as *const i8,
                            if F_WASMFS & unsafe { (*p_b_1).flags } as i32 != 0 {
                                1
                            } else { 0 }, z_build_name_1)
                    };
                }
                unsafe {
                    printf(c"\t@chmod -x $(out.%s.wasm)\n".as_ptr() as *mut i8
                            as *const i8, z_build_name_1)
                };
                unsafe {
                    printf(c"\t@$(call b.call.wasm-strip,%s)\n".as_ptr() as
                                *mut i8 as *const i8, z_build_name_1)
                };
                unsafe {
                    printf(c"\t@$(call b.do.wasm-opt,%s)\n".as_ptr() as *mut i8
                            as *const i8, z_build_name_1)
                };
                unsafe {
                    printf(c"\t@$(call b.strip-js-emcc-bindings,$(logtag.%s))\n".as_ptr()
                                as *mut i8 as *const i8, z_build_name_1)
                };
                if CP_JS & unsafe { (*p_b_1).flags } as i32 != 0 {
                    if !(unsafe { (*p_b_1).z_dot_wasm }).is_null() {
                        unsafe {
                            printf(c"\t@echo \'$(logtag.%s) $(emo.disk) s/\"%s.wasm\"/\"%s.wasm\"/g in $(dir.dout)/$(notdir $@)\'; \\\nsed -e \'s/\"%s.wasm\"/\"%s.wasm\"/g\' -e \"s/\'%s.wasm\'/\'%s.wasm\'/g\" $@ > $(dir.dout)/$(notdir $@);\n".as_ptr()
                                        as *mut i8 as *const i8, z_build_name_1, z_base_name,
                                unsafe { (*p_b_1).z_dot_wasm }, z_base_name,
                                unsafe { (*p_b_1).z_dot_wasm }, z_base_name,
                                unsafe { (*p_b_1).z_dot_wasm })
                        };
                    } else {
                        unsafe {
                            printf(c"\t@$(call b.cp,%s,$@,$(dir.dout))\n".as_ptr() as
                                        *mut i8 as *const i8, z_build_name_1)
                        };
                    }
                }
                if CP_WASM & unsafe { (*p_b_1).flags } as i32 != 0 {
                    unsafe {
                        printf(c"\t@$(call b.cp,%s,$(basename $@).wasm,$(dir.dout))\n".as_ptr()
                                    as *mut i8 as *const i8, z_build_name_1)
                    };
                }
            }
        }
        unsafe {
            printf(c"\t@$(call b.echo,%s,$(emo.done) done!%s)\n".as_ptr() as
                        *mut i8 as *const i8, z_build_name_1,
                if F_UNSUPPORTED & unsafe { (*p_b_1).flags } as i32 != 0 {
                    c" $(label.unsupported-build)".as_ptr() as *mut i8
                } else { c"".as_ptr() as *mut i8 })
        };
        unsafe {
            printf(c"\n%dbit: $(out.%s.js)\n$(out.%s.wasm): $(out.%s.js)\nb-%s: $(out.%s.wasm)\n".as_ptr()
                        as *mut i8 as *const i8,
                if F_64BIT & unsafe { (*p_b_1).flags } as i32 != 0 {
                    64
                } else { 32 }, z_build_name_1, z_build_name_1, z_build_name_1,
                z_build_name_1, z_build_name_1)
        };
        if CP_JS & unsafe { (*p_b_1).flags } as i32 != 0 {
            unsafe {
                printf(c"$(dir.dout)/%s%s: $(out.%s.js)\n".as_ptr() as *mut i8
                        as *const i8, z_base_name, z_js_ext, z_build_name_1)
            };
        }
        if CP_WASM & unsafe { (*p_b_1).flags } as i32 != 0 {
            unsafe {
                printf(c"$(dir.dout)/%s.wasm: $(out.%s.wasm)\n".as_ptr() as
                            *mut i8 as *const i8, z_base_name, z_build_name_1)
            };
        }
        unsafe {
            printf(c"%s: $(out.%s.js)\n".as_ptr() as *mut i8 as *const i8,
                if 0 ==
                        (F_UNSUPPORTED | F_NOT_IN_ALL) &
                            unsafe { (*p_b_1).flags } as i32 {
                    c"all".as_ptr() as *mut i8
                } else { c"more".as_ptr() as *mut i8 }, z_build_name_1)
        };
        if !(unsafe { (*p_b_1).z_if_cond }).is_null() {
            unsafe {
                printf(c"else\n$(info $(logtag.%s) $(emo.stop) disabled by condition: %s)\nendif\n".as_ptr()
                            as *mut i8 as *const i8, z_build_name_1,
                    unsafe { (*p_b_1).z_if_cond })
            };
        }
        unsafe {
            printf(c"# End build [%s]%s".as_ptr() as *mut i8 as *const i8,
                z_build_name_1,
                c"\n########################################################################\n".as_ptr()
                    as *mut i8)
        };
    }
}
extern "C" fn emit_gz(z_build_name_1: *const i8, z_file_ext_1: *const i8)
    -> () {
    unsafe {
        printf(c"\n$(out.%s.%s).gz: $(out.%s.%s)\n\t@$(call b.echo,%s,$(emo.disk))\n\t@gzip < $< > $@\n".as_ptr()
                    as *mut i8 as *const i8, z_build_name_1, z_file_ext_1,
            z_build_name_1, z_file_ext_1, z_build_name_1)
    };
}
extern "C" fn mk_fiddle() -> () {
    {
        let mut i: i32 = 0;
        '__b1: loop {
            if !(i < 2) { break '__b1; }
            '__c1: loop {
                let is_debug: i32 = (i > 0) as i32;
                let z_build_name: *const i8 =
                    if i != 0 {
                            c"fiddle.debug".as_ptr() as *mut i8
                        } else { c"fiddle".as_ptr() as *mut i8 } as *const i8;
                unsafe {
                    printf(c"\n########################################################################\n# Begin build %s\n".as_ptr()
                                as *mut i8 as *const i8, z_build_name)
                };
                if is_debug != 0 {
                    unsafe {
                        printf(c"emo.b.%s = $(emo.b.fiddle)$(emo.bug)\n".as_ptr() as
                                    *mut i8 as *const i8, z_build_name)
                    };
                } else {
                    unsafe {
                        printf(c"emo.b.fiddle = \u{1f3bb}\n".as_ptr() as *mut i8 as
                                *const i8)
                    };
                }
                emit_logtag(z_build_name);
                unsafe {
                    printf(c"dir.%s = %s\nout.%s.js = $(dir.%s)/fiddle-module.js\nout.%s.wasm = $(dir.%s)/fiddle-module.wasm\n$(out.%s.wasm): $(out.%s.js)\n".as_ptr()
                                as *mut i8 as *const i8, z_build_name, z_build_name,
                        z_build_name, z_build_name, z_build_name, z_build_name,
                        z_build_name, z_build_name)
                };
                emit_api_js(z_build_name);
                mk_pre_post(z_build_name, core::ptr::null());
                {
                    unsafe {
                        printf(c"$(out.%s.js): $(MAKEFILE_LIST) $(EXPORTED_FUNCTIONS.fiddle) $(fiddle.c.in) $(pre-post.%s.deps) $(dir.dout)/sqlite3-opfs-async-proxy.js".as_ptr()
                                    as *mut i8 as *const i8, z_build_name, z_build_name)
                    };
                    if is_debug != 0 {
                        unsafe {
                            printf(c" $(dir.fiddle)/fiddle-worker.js $(dir.fiddle)/fiddle.js $(dir.fiddle)/index.html".as_ptr()
                                        as *mut i8 as *const i8)
                        };
                    }
                    unsafe { puts(c"".as_ptr() as *mut i8 as *const i8) };
                    emit_compile_start(z_build_name);
                    unsafe {
                        printf(c"\t@$(call b.mkdir@)\n\t$(b.cmd@)$(bin.emcc) -o $@ $(emcc.flags.%s) $(pre-post.%s.flags) $(fiddle.c.in)\n".as_ptr()
                                    as *mut i8 as *const i8, z_build_name, z_build_name)
                    };
                    unsafe {
                        printf(c"\t@chmod -x $(out.%s.wasm)\n".as_ptr() as *mut i8
                                as *const i8, z_build_name)
                    };
                    unsafe {
                        printf(c"\t@$(call b.call.wasm-strip,%s)\n".as_ptr() as
                                    *mut i8 as *const i8, z_build_name)
                    };
                    unsafe {
                        printf(c"\t@$(call b.strip-js-emcc-bindings,$(logtag.%s))\n".as_ptr()
                                    as *mut i8 as *const i8, z_build_name)
                    };
                    unsafe {
                        printf(c"\t@$(call b.cp,%s,$(dir.dout)/sqlite3-opfs-async-proxy.js,$(dir $@))\n".as_ptr()
                                    as *mut i8 as *const i8, z_build_name)
                    };
                    if is_debug != 0 {
                        unsafe {
                            printf(c"\t@$(call b.cp,%s,$(dir.fiddle)/index.html $(dir.fiddle)/fiddle.js $(dir.fiddle)/fiddle-worker.js $(dir.fiddle)/sqlite3-opfs-async-proxy.js,$(dir $@))\n".as_ptr()
                                        as *mut i8 as *const i8, z_build_name)
                        };
                    }
                    unsafe {
                        printf(c"\t@$(call b.echo,%s,$(emo.done) done!)\n".as_ptr()
                                    as *mut i8 as *const i8, z_build_name)
                    };
                }
                unsafe {
                    printf(c"\n%s: $(out.%s.wasm)\n".as_ptr() as *mut i8 as
                            *const i8,
                        if is_debug != 0 {
                            c"more".as_ptr() as *mut i8
                        } else { c"all".as_ptr() as *mut i8 }, z_build_name)
                };
                emit_gz(z_build_name, c"js".as_ptr() as *mut i8 as *const i8);
                emit_gz(z_build_name,
                    c"wasm".as_ptr() as *mut i8 as *const i8);
                unsafe {
                    printf(c"\n%s: $(out.%s.js).gz $(out.%s.wasm).gz\nb-%s: %s\n".as_ptr()
                                as *mut i8 as *const i8, z_build_name, z_build_name,
                        z_build_name, z_build_name, z_build_name)
                };
                if is_debug != 0 {
                    unsafe {
                        puts(c"fiddle-debug: fiddle.debug".as_ptr() as *mut i8 as
                                *const i8)
                    };
                } else {
                    unsafe {
                        puts(c"all: b-fiddle".as_ptr() as *mut i8 as *const i8)
                    };
                }
                unsafe {
                    printf(c"# End %s\n########################################################################\n".as_ptr()
                                as *mut i8 as *const i8, z_build_name)
                };
                break '__c1;
            }
            { let __p = &mut i; *__p += 1; *__p };
        }
    }
}
extern "C" fn __main_inner(argc: i32, argv: *const *const i8)
    -> Result<(), i32> {
    unsafe {
        let mut rc: i32 = 0;
        let p_b: *const BuildDef = core::ptr::null();
        unsafe {
            printf(c"# What follows was GENERATED by %s. Edit at your own risk.\n".as_ptr()
                        as *mut i8 as *const i8,
                c"/Volumes/ProjectsSSD/Projects/transpilers/cpp-to-rust/Testing/corpuses/sqlite/ext/wasm/mkwasmbuilds.c".as_ptr()
                    as *mut i8)
        };
        if argc > 1 {
            {
                let mut i: i32 = 1;
                '__b2: loop {
                    if !(i < argc) { break '__b2; }
                    '__c2: loop {
                        let z_arg: *const i8 = unsafe { *argv.offset(i as isize) };
                        if 0 ==
                                unsafe {
                                    strcmp(c"vanilla".as_ptr() as *mut i8 as *const i8, z_arg)
                                } {
                            mk_lib_mode(c"vanilla".as_ptr() as *mut i8 as *const i8,
                                &o_build_defs.vanilla);
                        } else if 0 ==
                                unsafe {
                                    strcmp(c"vanilla64".as_ptr() as *mut i8 as *const i8, z_arg)
                                } {
                            mk_lib_mode(c"vanilla64".as_ptr() as *mut i8 as *const i8,
                                &o_build_defs.vanilla64);
                        } else if 0 ==
                                unsafe {
                                    strcmp(c"esm".as_ptr() as *mut i8 as *const i8, z_arg)
                                } {
                            mk_lib_mode(c"esm".as_ptr() as *mut i8 as *const i8,
                                &o_build_defs.esm);
                        } else if 0 ==
                                unsafe {
                                    strcmp(c"esm64".as_ptr() as *mut i8 as *const i8, z_arg)
                                } {
                            mk_lib_mode(c"esm64".as_ptr() as *mut i8 as *const i8,
                                &o_build_defs.esm64);
                        } else if 0 ==
                                unsafe {
                                    strcmp(c"bundler".as_ptr() as *mut i8 as *const i8, z_arg)
                                } {
                            mk_lib_mode(c"bundler".as_ptr() as *mut i8 as *const i8,
                                &o_build_defs.bundler);
                        } else if 0 ==
                                unsafe {
                                    strcmp(c"bundler64".as_ptr() as *mut i8 as *const i8, z_arg)
                                } {
                            mk_lib_mode(c"bundler64".as_ptr() as *mut i8 as *const i8,
                                &o_build_defs.bundler64);
                        } else if 0 ==
                                unsafe {
                                    strcmp(c"speedtest1".as_ptr() as *mut i8 as *const i8,
                                        z_arg)
                                } {
                            mk_lib_mode(c"speedtest1".as_ptr() as *mut i8 as *const i8,
                                &o_build_defs.speedtest1);
                        } else if 0 ==
                                unsafe {
                                    strcmp(c"speedtest164".as_ptr() as *mut i8 as *const i8,
                                        z_arg)
                                } {
                            mk_lib_mode(c"speedtest164".as_ptr() as *mut i8 as
                                    *const i8, &o_build_defs.speedtest164);
                        } else if 0 ==
                                unsafe {
                                    strcmp(c"node".as_ptr() as *mut i8 as *const i8, z_arg)
                                } {
                            mk_lib_mode(c"node".as_ptr() as *mut i8 as *const i8,
                                &o_build_defs.node);
                        } else if 0 ==
                                unsafe {
                                    strcmp(c"node64".as_ptr() as *mut i8 as *const i8, z_arg)
                                } {
                            mk_lib_mode(c"node64".as_ptr() as *mut i8 as *const i8,
                                &o_build_defs.node64);
                        } else if 0 ==
                                unsafe {
                                    strcmp(c"wasmfs".as_ptr() as *mut i8 as *const i8, z_arg)
                                } {
                            mk_lib_mode(c"wasmfs".as_ptr() as *mut i8 as *const i8,
                                &o_build_defs.wasmfs);
                        } else if 0 ==
                                unsafe {
                                    strcmp(c"prologue".as_ptr() as *mut i8 as *const i8, z_arg)
                                } {
                            mk_prologue();
                        } else {
                            unsafe {
                                fprintf(__stderrp,
                                    c"Unknown build name: %s\n".as_ptr() as *mut i8 as
                                        *const i8, z_arg)
                            };
                            rc = 1;
                            break '__b2;
                        }
                        break '__c2;
                    }
                    { let __p = &mut i; *__p += 1; *__p };
                }
            }
        } else {
            mk_prologue();
            mk_lib_mode(c"vanilla".as_ptr() as *mut i8 as *const i8,
                &o_build_defs.vanilla);
            mk_lib_mode(c"vanilla64".as_ptr() as *mut i8 as *const i8,
                &o_build_defs.vanilla64);
            mk_lib_mode(c"esm".as_ptr() as *mut i8 as *const i8,
                &o_build_defs.esm);
            mk_lib_mode(c"esm64".as_ptr() as *mut i8 as *const i8,
                &o_build_defs.esm64);
            mk_lib_mode(c"bundler".as_ptr() as *mut i8 as *const i8,
                &o_build_defs.bundler);
            mk_lib_mode(c"bundler64".as_ptr() as *mut i8 as *const i8,
                &o_build_defs.bundler64);
            mk_lib_mode(c"speedtest1".as_ptr() as *mut i8 as *const i8,
                &o_build_defs.speedtest1);
            mk_lib_mode(c"speedtest164".as_ptr() as *mut i8 as *const i8,
                &o_build_defs.speedtest164);
            mk_lib_mode(c"node".as_ptr() as *mut i8 as *const i8,
                &o_build_defs.node);
            mk_lib_mode(c"node64".as_ptr() as *mut i8 as *const i8,
                &o_build_defs.node64);
            mk_lib_mode(c"wasmfs".as_ptr() as *mut i8 as *const i8,
                &o_build_defs.wasmfs);
            mk_fiddle();
        }
        return Err(rc);
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *const i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() { return 0; }
    return __r.unwrap_err();
}
extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn puts(_: *const i8)
    -> i32;
    fn printf(_: *const i8, ...)
    -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8)
    -> ();
    fn strcmp(__s1: *const i8, __s2: *const i8)
    -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, ...)
    -> i32;
    static mut __stderrp: *mut FILE;
    fn __builtin_expect(_: i64, _: i64)
    -> i64;
}
#[repr(C)]
#[derive(Copy, Clone)]
struct SFILE {
    _opaque: [u8; 0],
}
type FILE = SFILE;