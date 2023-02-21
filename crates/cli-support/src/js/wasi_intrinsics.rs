use anyhow::{anyhow, bail, Error};
use walrus::MemoryId;

use crate::intrinsic::WasiIntrinsic;

use super::Context;

impl<'a> Context<'a> {
    fn get_memory(&self) -> Result<MemoryId, Error> {
        let mut memories = self.module.memories.iter();
        let memory = memories
            .next()
            .ok_or_else(|| anyhow!("no memory found to return in memory intrinsic"))?
            .id();
        if memories.next().is_some() {
            bail!(
                "multiple memories found, unsure which to return \
                    from memory intrinsic"
            );
        }
        Ok(memory)
    }

    pub fn invoke_wasi_intrinsic(
        &mut self,
        intrinsic: &WasiIntrinsic,
        args: &[String],
        prelude: &mut String,
    ) -> Result<String, Error> {
        let expr = match intrinsic {
            WasiIntrinsic::ClockTimeGet => {
                assert_eq!(args.len(), 3);

                let mem = self.expose_int64_memory(self.get_memory()?);
                let res_ptr = &args[2];

                prelude.push_str(&format!(
                    "
                    let time = BigInt(new Date().getTime());
                    {mem}()[{res_ptr} / 8] = time;
                "
                ));

                "0".to_string()
            }
            WasiIntrinsic::FdWrite => {
                assert_eq!(args.len(), 4);

                "8".to_string()
            }
            WasiIntrinsic::FdRead => {
                assert_eq!(args.len(), 4);

                "8".to_string()
            }
            WasiIntrinsic::FdSeek => "8".to_string(),
            WasiIntrinsic::SchedYield => {
                assert_eq!(args.len(), 0);
                String::default()
            }
            WasiIntrinsic::RandomGet => {
                assert_eq!(args.len(), 2);

                let mem = self.expose_uint8_memory(self.get_memory()?);
                let ptr = &args[0];
                let len = &args[1];

                prelude.push_str(&format!(
                    "
                    crypto.getRandomValues({mem}().subarray({ptr}, {ptr} + {len}));
                "
                ));

                "0".to_string()
            }
            WasiIntrinsic::EnvironGet => {
                assert_eq!(args.len(), 2);
                "0".to_string()
            }
            WasiIntrinsic::EnvironSizesGet => {
                assert_eq!(args.len(), 2);

                let mem = self.expose_uint32_memory(self.get_memory()?);
                let count_ptr = &args[0];
                let size_ptr = &args[1];

                prelude.push_str(&format!(
                    "
                    {mem}()[{count_ptr} / 4] = 0;
                    {mem}()[{size_ptr} / 4] = 0;
                "
                ));

                "0".to_string()
            }
            WasiIntrinsic::FdClose => "8".to_string(),
            WasiIntrinsic::FdFdStatGet => "8".to_string(),
            WasiIntrinsic::FdFdStatSetFlags => "8".to_string(),
            WasiIntrinsic::FdPrestatGet => "8".to_string(),
            WasiIntrinsic::FdPrestatDirName => "8".to_string(),
            WasiIntrinsic::PathOpen => "-1".to_string(),
            WasiIntrinsic::ProcExit => {
                let code = &args[0];
                format!("throw \"proc_exit called with code \" + {code};")
            }
            WasiIntrinsic::ArgsGet => "0".to_string(),
            WasiIntrinsic::ArgsSizesGet => {
                let mem = self.expose_uint32_memory(self.get_memory()?);
                let count_ptr = &args[0];
                let size_ptr = &args[1];

                prelude.push_str(&format!(
                    "
                    {mem}()[{count_ptr} / 4] = 0;
                    {mem}()[{size_ptr} / 4] = 0;
                "
                ));

                "0".to_string()
            }
            WasiIntrinsic::FdFileStatGet => "8".to_string(),
            WasiIntrinsic::PathFileStatGet => "-1".to_string(),
        };
        Ok(expr)
    }
}
