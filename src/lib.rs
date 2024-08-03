use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "install", &format!("helm@{}", version)])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn install(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "helm", "install", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn lint(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "helm", "lint", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn package(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "helm", "package", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn pull(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "helm", "pull", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn push(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "helm", "push", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn registry(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "helm", "registry", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn rollback(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "helm", "rollback", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn test(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "helm", "test", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn upgrade(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "helm", "upgrade", &args])?
        .stdout()?;
    Ok(stdout)
}
