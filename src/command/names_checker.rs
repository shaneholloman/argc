use crate::param::{EnvParam, FlagOptionParam, Param, PositionalParam};
use crate::parser::Position;

use anyhow::{bail, Result};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub(crate) struct NamesChecker {
    pub(crate) flag_options: HashMap<String, (Position, String)>,
    pub(crate) envs: HashMap<String, Position>,
    pub(crate) positionals: HashMap<String, Position>,
}

impl NamesChecker {
    pub(crate) fn check_flag_option(
        &mut self,
        param: &FlagOptionParam,
        pos: Position,
    ) -> Result<()> {
        let tag_name = param.tag_name();
        let names = param.list_names();
        for name in names.iter() {
            if let Some((exist_pos, _)) = self.flag_options.get(name) {
                bail!("{}", Self::conflict_error(tag_name, pos, name, *exist_pos));
            }
            self.flag_options
                .insert(name.to_string(), (pos, format!("{tag_name} {name}")));
        }
        Ok(())
    }

    pub(crate) fn check_env(&mut self, param: &EnvParam, pos: Position) -> Result<()> {
        let name = param.id();
        if let Some(exist_pos) = self.envs.get(name) {
            bail!(
                "{}",
                Self::conflict_error(param.tag_name(), pos, name, *exist_pos)
            );
        }
        self.envs.insert(name.to_string(), pos);
        Ok(())
    }

    pub(crate) fn check_positional(
        &mut self,
        param: &PositionalParam,
        pos: Position,
    ) -> Result<()> {
        let name = param.id();
        if let Some(exist_pos) = self.positionals.get(name) {
            bail!(
                "{}",
                Self::conflict_error(param.tag_name(), pos, name, *exist_pos)
            );
        }
        self.positionals.insert(name.to_string(), pos);
        Ok(())
    }

    fn conflict_error(
        tag_name: &str,
        pos: Position,
        name_desc: &str,
        exist_pos: Position,
    ) -> String {
        format!("{tag_name}(line {pos}) has '{name_desc}' already exists at line {exist_pos}",)
    }
}
