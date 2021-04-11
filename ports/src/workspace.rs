use crate::errors::*;
use std::fs::{create_dir_all, File};
use crate::sources::Source;
use std::process::{Command, Stdio};
use std::path::Path;
use std::io::copy;
use crate::util::convert_to_str;
use specfile::SpecFile;
use std::collections::HashMap;
use std::io::prelude::*;
use std::env;

static DEFAULTWORKSPACEROOT: &str = "~/.ports/wks";
static DEFAULTARCH: &str = "i386";
static DEFAULTTAR: &str = "gtar";

pub struct Workspace {
    root: String,
    source_dir: String,
    build_dir: String,
    proto_dir: String,
}

fn init_root(ws: &Workspace) -> Result<()> {
    create_dir_all(ws.root.clone())?;
    create_dir_all(ws.build_dir.clone())?;
    create_dir_all(ws.source_dir.clone())?;
    create_dir_all(ws.proto_dir.clone())?;

    Ok(())
}

impl Workspace {
    pub fn new(root: &str) -> Result<Workspace> {

        let root_dir = if root == "" {
            DEFAULTWORKSPACEROOT
        } else {
            root
        };

        let expanded_root_dir = shellexpand::full(root_dir)?.to_string();

        let ws = Workspace{
            root: expanded_root_dir.clone(),
            build_dir: expanded_root_dir.clone() + "/build/" + DEFAULTARCH,
            source_dir: expanded_root_dir.clone()+ "/sources/",
            proto_dir: expanded_root_dir.clone()+ "/build/proto",
        };

        init_root(&ws)?;

        Ok(ws)
    }

    pub fn expand_source_path(&self, fname: &str) -> String {
        self.source_dir.clone() + "/" + fname
    }

    pub fn get_proto_dir(&self) -> String {
        self.proto_dir.clone()
    }

    pub fn get_build_dir(&self) -> String {
        self.build_dir.clone()
    }

    pub fn get_macros(&self) -> HashMap<String, String> {
        let mut hm = HashMap::<String, String>::new();
        hm.insert("proto_dir".to_owned(), self.proto_dir.clone());
        hm.insert("build_dir".to_owned(), self.build_dir.clone());
        hm.insert("source_dir".to_owned(), self.source_dir.clone());
        hm
    }

    pub fn get_sources(&self, sources: Vec<String>) -> Result<Vec<Source>> {
        let mut src_vec: Vec<Source> = vec![];
        for src in sources {
            let src_struct = Source::new(&src, self.source_dir.clone().as_str())?;
            let bytes = reqwest::blocking::get(src_struct.url.as_str())?.bytes()?;
            let mut out = File::create(&src_struct.local_name)?;
            copy(&mut bytes.as_ref(), &mut out)?;

            src_vec.push(src_struct);
        }

        Ok(src_vec)
    }

    pub fn unpack_all_sources(&self, sources: Vec<Source>) -> Result<()> {
        for src in sources {
            self.unpack_source(&src)?;
        }

        Ok(())
    }

    pub fn unpack_source(&self, src: &Source) -> Result<()> {
        match Path::new(&src.local_name).extension() {
            Some(ext) => {
                if !convert_to_str(ext.to_str())?.contains("tar") {
                    return Err(ErrorKind::NotExtractableSource(src.local_name.clone()).into());
                }
                //TODO support inspecting the tar file to see if we have a top level directory or not
                let mut tar_cmd = Command::new(DEFAULTTAR)
                    .args(&["-C", &self.build_dir, "-xaf", &src.local_name, "--strip-components=1"])
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()?;

                let status = tar_cmd.wait()?;
                if !status.success() {
                    return Err(ErrorKind::Msg("tar command failed".to_owned()).into());
                }
            }
            None => {
                return Err(ErrorKind::NotExtractableSource(src.local_name.clone()).into());
            }
        }

        Ok(())
    }

    pub fn build(&self, build_script: String) -> Result<()> {
        let build_script_path = self.build_dir.clone() + "/build_script.sh";
        let mut file = File::create(&build_script_path)?;
        file.write_all(b"#!/usr/bin/env bash\n")?;
        file.write_all(build_script.as_bytes())?;
        file.write_all(b"\n")?;
        let bash = which::which("bash")?;
        let filtered_env : HashMap<String, String> =
            env::vars().filter(|&(ref k, _)|
                k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH"
            ).collect();
        let mut shell = Command::new(bash)
            .args(&["-x", &build_script_path])
            .env_clear()
            .envs(&filtered_env)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        let status = shell.wait()?;
        if !status.success() {
            return Err(ErrorKind::Msg("build script failed".to_owned()).into());
        }
        Ok(())
    }
}