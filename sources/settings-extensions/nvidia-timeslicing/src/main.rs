use bottlerocket_settings_sdk::{BottlerocketSetting, NullMigratorExtensionBuilder};
use settings_extension_nvidia_timeslicing::NvidiaTimeslicingV1;
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();

    match NullMigratorExtensionBuilder::with_name("nvidia-timeslicing")
        .with_models(vec![BottlerocketSetting::<NvidiaTimeslicingV1>::model()])
        .build()
    {
        Ok(extension) => extension.run(),
        Err(e) => {
            println!("{}", e);
            ExitCode::FAILURE
        }
    }
}
