use std::process::Command;
fn git(args:&[&str])->String{Command::new("git").args(args).output().ok().filter(|o|o.status.success()).map(|o|String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default()}
fn main(){
    let sha=git(&["rev-parse","--short=12","HEAD"]);
    let dirty=!git(&["status","--porcelain","--untracked-files=no"]).is_empty();
    println!("cargo:rustc-env=SPHERES_REVISION={}{}",if sha.is_empty(){"source-archive"}else{&sha},if dirty{"-modified"}else{""});
    let branch_name=git(&["rev-parse","--abbrev-ref","HEAD"]);
    println!("cargo:rustc-env=SPHERES_BRANCH={}",if branch_name.is_empty(){"source-archive"}else{&branch_name});
    let epoch=std::env::var("SOURCE_DATE_EPOCH").ok().and_then(|v|v.parse::<u64>().ok()).unwrap_or_else(||std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs());
    println!("cargo:rustc-env=SPHERES_BUILD_EPOCH={epoch}");
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    for name in ["HEAD","index"]{let p=git(&["rev-parse","--git-path",name]);if !p.is_empty(){println!("cargo:rerun-if-changed={p}");}}
    let branch=git(&["symbolic-ref","-q","HEAD"]);if !branch.is_empty(){let p=git(&["rev-parse","--git-path",&branch]);println!("cargo:rerun-if-changed={p}");}
    println!("cargo:rerun-if-changed=Cargo.toml");
}
