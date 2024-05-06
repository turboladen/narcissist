pub(crate) mod install;
pub(crate) mod is_installed;

pub(crate) trait Builtin {
    type Output;

    fn run() -> anyhow::Result<Self::Output>;
    fn dry_run() -> anyhow::Result<Self::Output>;
}
