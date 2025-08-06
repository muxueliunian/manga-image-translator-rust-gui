fn main() {
    let mut build = cc::Build::new();

    build
        .cpp(true)
        .include("cpp/pydensecrf/pydensecrf/densecrf/external/liblbfgs/include")
        .include("cpp/pydensecrf/pydensecrf/densecrf/include")
        .file("cpp/pydensecrf/pydensecrf/densecrf/src/densecrf.cpp")
        .file("cpp/pydensecrf/pydensecrf/densecrf/src/labelcompatibility.cpp")
        .file("cpp/pydensecrf/pydensecrf/densecrf/src/objective.cpp")
        .file("cpp/pydensecrf/pydensecrf/densecrf/src/optimization.cpp")
        .file("cpp/pydensecrf/pydensecrf/densecrf/src/pairwise.cpp")
        .file("cpp/pydensecrf/pydensecrf/densecrf/src/permutohedral.cpp")
        .file("cpp/pydensecrf/pydensecrf/densecrf/src/unary.cpp")
        .file("cpp/pydensecrf/pydensecrf/densecrf/src/util.cpp")
        .file("cpp/pydensecrf/pydensecrf/densecrf/external/liblbfgs/lib/lbfgs.c")
        .file("cpp/wrapper.cpp")
        .flag_if_supported("-std=c++11")
        .compile("densecrf");

    println!("cargo:rerun-if-changed=cpp/");
}
