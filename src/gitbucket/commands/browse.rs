use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;

use crate::bitbucket::get_current_repo_id;
use crate::common_git::{get_current_branch, BaseUrlConfig, Repository};
use crate::error::Error;

pub struct Browse;

impl Browse {
    pub fn handle<Arg: AsRef<str>, Conf>(
        args: &[Arg],
        repo: &Repository,
        config: &Conf,
        path: &Path,
    ) -> Result<(), Error>
    where
        Conf: BaseUrlConfig,
    {
        let repo_id = get_current_repo_id(&repo, config).ok_or(Error::InvalidRepo)?;
        let branch = get_current_branch(&repo).ok_or(Error::Detached)?;

        let mut url = repo_id.url(&config.base_url());
        let mut open_pr = false;

        {
            let mut segments = url.path_segments_mut().unwrap();

            match (
                args.get(0).map(AsRef::<_>::as_ref),
                args.get(1).map(AsRef::<_>::as_ref),
            ) {
                (Some("pr"), Some(id)) => {
                    let parsed_id =
                        u16::from_str(&id).map_err(|_| Error::InvalidPrId(id.to_string()))?;
                    segments.push("pull-requests");
                    segments.push(&format!("{}", parsed_id));
                    open_pr = true;
                }
                _ => {
                    segments.push("browse");

                    let relative_path =
                        repo.workdir().map(|p| path.strip_prefix(&p).ok()).flatten();

                    if let Some(relative_path) = relative_path {
                        for comp in relative_path.components().map(|comp| comp.as_os_str()) {
                            if let Some(comp) = comp.to_str() {
                                segments.push(comp);
                            }
                        }
                    }
                }
            }
        }

        if !open_pr {
            url.query_pairs_mut().append_pair("at", &branch);
        }

        Command::new("open")
            .arg(url.as_str())
            .stdout(Stdio::null())
            .spawn()
            .map(|_| ())
            .map_err(|err| Error::OpenUrl(err, url))
    }
}
