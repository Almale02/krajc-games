static mut ENGINE: TypedAddr<EngineRuntime> = TypedAddr::default();

use krajc::{
    abi::prelude::*,
    engine_runtime::{
        schedule_manager::{
            runtime_schedule::*,
            system_params::{
                system_engine::UnsafeEngineAccess, system_local::Local,
                system_param::FunctionSystem, system_resource::Res,
            },
        },
        target_fps::TargetFps,
        EngineRuntime,
    },
    prelude::*,
    typed_addr::TypedAddr,
};

#[stabby::export]
#[stabby]
pub extern "C" fn get_plugin() -> SystemPlugin {
    SystemPlugin { register_systems }
}
extern "C" fn register_systems(
    engine: TypedAddr<EngineRuntime>,
    reg: SystemPluginRegister,
    leak: TypedAddr<u32>,
) {
    dump_memory(engine.get());
    println!("leak was: {}", leak.get());
    *leak.get() += 1;
    println!("engine address in plugin is {}", engine.addr);
    unsafe { ENGINE = engine };
    reg.start_register::<RuntimeUpdateSchedule>()
        .register(FunctionSystem::new_with_name(system, "custom_system"));
}
#[system_fn]
fn system(mut engine: UnsafeEngineAccess) {
    engine.test += 2;
}
