fn main() {
    println!("cargo:rustc-link-search=/Users/ilya_rozhnev/scop/src/minilibx/");
    println!("cargo:rustc-link-lib=static=mlx");
}