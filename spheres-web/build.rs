use std::process::Command;
fn git(args:&[&str])->String{Command::new("git").args(args).output().ok().filter(|o|o.status.success()).map(|o|String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default()}
fn main(){
    let sha=git(&["rev-parse","--short=12","HEAD"]);
    let dirty=!git(&["status","--porcelain","--untracked-files=no"]).is_empty();
    println!("cargo:rustc-env=SPHERES_REVISION={}{}",if sha.is_empty(){"source-archive"}else{&sha},if dirty{"-modified"}else{""});
    for name in ["HEAD","index"]{let p=git(&["rev-parse","--git-path",name]);if !p.is_empty(){println!("cargo:rerun-if-changed={p}");}}
    let branch=git(&["symbolic-ref","-q","HEAD"]);if !branch.is_empty(){let p=git(&["rev-parse","--git-path",&branch]);println!("cargo:rerun-if-changed={p}");}
    println!("cargo:rerun-if-changed=Cargo.toml");
}
