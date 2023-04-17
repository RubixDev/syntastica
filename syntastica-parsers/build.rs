#[cfg(not(feature = "all"))]
fn main() {}

#[cfg(feature = "all")]
macro_rules! langs {
    ($($name:literal, $url:literal, $branch:literal, external_c = $c:tt, external_cpp = $cpp:tt);* $(;)?) => {$(
        // clone repo into `parsers/{name}`, or pull if it already exists
        let repo_dir = std::path::Path::new(concat!("parsers/", $name));
        if repo_dir.exists() {
            pull::pull(git2::Repository::open(repo_dir)?, "origin", $branch)?;
        } else {
            git2::Repository::clone_recurse($url, &repo_dir)?;
        }

        let src_dir = repo_dir.join("src");

        let mut c_config = cc::Build::new();
        c_config.include(&src_dir);
        c_config
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-but-set-variable")
            .flag_if_supported("-Wno-trigraphs");
        let parser_path = src_dir.join("parser.c");
        c_config.file(&parser_path);

        langs!(@c $c, src_dir, c_config);

        println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
        c_config.compile(concat!("parser_", $name));

        langs!(@cpp $cpp, src_dir, $name);
    )*};
    (@c true, $src_dir:ident, $c_config:ident) => {
        let scanner_path = $src_dir.join("scanner.c");
        $c_config.file(&scanner_path);
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    };
    (@c false, $src_dir:ident, $c_config:ident) => {};
    (@cpp true, $src_dir:ident, $name:literal) => {
        let mut cpp_config = cc::Build::new();
        cpp_config.cpp(true);
        cpp_config.include(&$src_dir);
        cpp_config
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-but-set-variable");
        let scanner_path = $src_dir.join("scanner.cc");
        cpp_config.file(&scanner_path);
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
        cpp_config.compile(concat!("scanner_", $name));
    };
    (@cpp false, $src_dir:ident, $name:literal) => {};
}

#[cfg(feature = "all")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    langs! {
        "regex", "https://github.com/tree-sitter/tree-sitter-regex", "master", external_c = false, external_cpp = false;
    }
    Ok(())
}

///////////////////////////////////////////////////////////////////////
// The following is mostly taken from git2's `pull` example: //////////
// https://github.com/rust-lang/git2-rs/blob/master/examples/pull.rs //
///////////////////////////////////////////////////////////////////////

#[cfg(feature = "all")]
mod pull {
    fn do_fetch<'a>(
        repo: &'a git2::Repository,
        refs: &[&str],
        remote: &'a mut git2::Remote,
    ) -> Result<git2::AnnotatedCommit<'a>, git2::Error> {
        let mut fo = git2::FetchOptions::new();
        // Always fetch all tags.
        // Perform a download and also update tips
        fo.download_tags(git2::AutotagOption::All);
        remote.fetch(refs, Some(&mut fo), None)?;

        // If there are local objects (we got a thin pack), then tell the user
        // how many objects we saved from having to cross the network.
        let stats = remote.stats();
        if stats.local_objects() > 0 {
            println!(
                "\rReceived {}/{} objects in {} bytes (used {} local \
             objects)",
                stats.indexed_objects(),
                stats.total_objects(),
                stats.received_bytes(),
                stats.local_objects()
            );
        } else {
            println!(
                "\rReceived {}/{} objects in {} bytes",
                stats.indexed_objects(),
                stats.total_objects(),
                stats.received_bytes()
            );
        }

        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        repo.reference_to_annotated_commit(&fetch_head)
    }

    fn fast_forward(
        repo: &git2::Repository,
        lb: &mut git2::Reference,
        rc: &git2::AnnotatedCommit,
    ) -> Result<(), git2::Error> {
        let name = match lb.name() {
            Some(s) => s.to_string(),
            None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
        };
        let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
        println!("{}", msg);
        lb.set_target(rc.id(), &msg)?;
        repo.set_head(&name)?;
        repo.checkout_head(Some(
            git2::build::CheckoutBuilder::default()
                // For some reason the force is required to make the working directory actually get updated
                // I suspect we should be adding some logic to handle dirty working directory states
                // but this is just an example so maybe not.
                .force(),
        ))?;
        Ok(())
    }

    fn normal_merge(
        repo: &git2::Repository,
        local: &git2::AnnotatedCommit,
        remote: &git2::AnnotatedCommit,
    ) -> Result<(), git2::Error> {
        let local_tree = repo.find_commit(local.id())?.tree()?;
        let remote_tree = repo.find_commit(remote.id())?.tree()?;
        let ancestor = repo
            .find_commit(repo.merge_base(local.id(), remote.id())?)?
            .tree()?;
        let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

        if idx.has_conflicts() {
            repo.checkout_index(Some(&mut idx), None)?;
            return Ok(());
        }
        let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
        // now create the merge commit
        let msg = format!("Merge: {} into {}", remote.id(), local.id());
        let sig = repo.signature()?;
        let local_commit = repo.find_commit(local.id())?;
        let remote_commit = repo.find_commit(remote.id())?;
        // Do our merge commit and set current branch head to that commit.
        let _merge_commit = repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &msg,
            &result_tree,
            &[&local_commit, &remote_commit],
        )?;
        // Set working tree to match head.
        repo.checkout_head(None)?;
        Ok(())
    }

    fn do_merge<'a>(
        repo: &'a git2::Repository,
        remote_branch: &str,
        fetch_commit: git2::AnnotatedCommit<'a>,
    ) -> Result<(), git2::Error> {
        // 1. do a merge analysis
        let analysis = repo.merge_analysis(&[&fetch_commit])?;

        // 2. Do the appropriate merge
        if analysis.0.is_fast_forward() {
            // do a fast forward
            let refname = format!("refs/heads/{}", remote_branch);
            match repo.find_reference(&refname) {
                Ok(mut r) => {
                    fast_forward(repo, &mut r, &fetch_commit)?;
                }
                Err(_) => {
                    // The branch doesn't exist so just set the reference to the
                    // commit directly. Usually this is because you are pulling
                    // into an empty repository.
                    repo.reference(
                        &refname,
                        fetch_commit.id(),
                        true,
                        &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
                    )?;
                    repo.set_head(&refname)?;
                    repo.checkout_head(Some(
                        git2::build::CheckoutBuilder::default()
                            .allow_conflicts(true)
                            .conflict_style_merge(true)
                            .force(),
                    ))?;
                }
            };
        } else if analysis.0.is_normal() {
            // do a normal merge
            let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
            normal_merge(repo, &head_commit, &fetch_commit)?;
        }
        Ok(())
    }

    pub fn pull(
        repo: git2::Repository,
        remote_name: &str,
        remote_branch: &str,
    ) -> Result<(), git2::Error> {
        let mut remote = repo.find_remote(remote_name)?;
        let fetch_commit = do_fetch(&repo, &[remote_branch], &mut remote)?;
        do_merge(&repo, remote_branch, fetch_commit)
    }
}
