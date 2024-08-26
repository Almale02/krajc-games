use krajc::{
    abi::prelude::*,
    engine_runtime::{
        schedule_manager::{
            runtime_schedule::*,
            system_params::{system_param::FunctionSystem, system_resource::Res},
        },
        target_fps::TargetFps,
    },
    prelude::*,
};

#[stabby::export]
#[stabby]
pub extern "C" fn get_plugin() -> SystemPlugin {
    SystemPlugin { register_systems }
}
extern "C" fn register_systems(reg: SystemPluginRegister) {
    reg.start_register::<RuntimeUpdateSchedule>()
        .register(FunctionSystem::new_with_name(system, "custom_system"));
}
#[system_fn]
fn system(mut fps: Res<TargetFps>) {
    dbg!("aaaaaa");
    fps.0 += 15.;
}
