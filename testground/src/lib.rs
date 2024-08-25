use krajc::{
    abi::prelude::*,
    engine_runtime::{
        schedule_manager::{
            runtime_schedule::*, schedule::IntoSystem, system_params::system_resource::Res,
        },
        target_fps::TargetFps,
    },
};

#[stabby::export]
#[stabby]
pub extern "C" fn get_plugin() -> SystemPlugin {
    SystemPlugin { register_systems }
}
extern "C" fn register_systems(reg: SystemPluginRegister) {
    reg.start_register::<RuntimeUpdateSchedule>();
}
fn system(mut fps: Res<TargetFps>) {
    fps.0 += 0.01;
}
