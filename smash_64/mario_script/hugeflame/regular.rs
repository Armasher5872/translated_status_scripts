unsafe extern "C" fn mario_hugeflame_regular_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn mario_hugeflame_regular_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let x_1 = 0.0;
    let y_1 = 0.0;
    let z_1 = 0.0;
    let x_2 = 0.0;
    let y_2 = 0.0;
    let z_2 = 0.0;
    let vector_1 = weapon.Vector3__create(x_1.into(), y_1.into(), z_1.into());
    let vec_x_1 = vector_1["x"].get_f32();
    let vec_y_1 = vector_1["y"].get_f32();
    let vec_z_1 = vector_1["z"].get_f32();
    let vector_2 = weapon.Vector3__create(x_2.into(), y_2.into(), z_2.into());
    let vec_x_2 = vector_2["x"].get_f32();
    let vec_y_2 = vector_2["y"].get_f32();
    let vec_z_2 = vector_2["z"].get_f32();
    ModelModule::joint_global_position(weapon.module_accessor, Hash40::new("fire1"), &Vector3f{x: vec_x_1, y: vec_y_1, z: vec_z_1}, true);
    ModelModule::joint_global_position(weapon.module_accessor, Hash40::new("fire2"), &Vector3f{x: vec_x_2, y: vec_y_2, z: vec_z_2}, true);
    WorkModule::set_float(weapon.module_accessor, vec_x_1, WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS1_X);
    WorkModule::set_float(weapon.module_accessor, vec_y_1, WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS1_Y);
    WorkModule::set_float(weapon.module_accessor, vec_x_2, WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS2_X);
    WorkModule::set_float(weapon.module_accessor, vec_y_2, WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS2_Y);
    0.into()
}

unsafe extern "C" fn mario_hugeflame_regular_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        mario_hugeflame_regular_sub_status(weapon, false.into());
    }
    weapon.global_table[0x15].assign(&L2CValue::Ptr(mario_hugeflame_regular_sub_status as *const () as _));
    weapon.fastshift(L2CValue::Ptr(mario_hugeflame_regular_main_loop as *const () as _))
}

unsafe extern "C" fn mario_pumpwater_regular_sub_status(weapon: &mut L2CWeaponCommon, should_dec_life: L2CValue) -> L2CValue {
    if should_dec_life {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn mario_hugeflame_regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let module_accessor = weapon.global_table[0x5].get_ptr() as *mut BattleObjectModuleAccessor;
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_INT_LIFE);
    if life > 0 {
        let x_1 = 0.0;
        let y_1 = 0.0;
        let z_1 = 0.0;
        let x_2 = 0.0;
        let y_2 = 0.0;
        let z_2 = 0.0;
        let pos1_x = WorkModule::get_float(weapon.module_accessor, *WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS1_X);
        let pos1_y = WorkModule::get_float(weapon.module_accessor, *WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS1_Y);
        let pos2_x = WorkModule::get_float(weapon.module_accessor, *WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS2_X);
        let pos2_y = WorkModule::get_float(weapon.module_accessor, *WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS2_Y);
        let vector_1 = weapon.Vector3__create(x_1.into(), y_1.into(), z_1.into());
        let vector_2 = weapon.Vector3__create(x_2.into(), y_1.into(), z_2.into());
        let vec_x_1 = vector_1["x"].get_f32();
        let vec_y_1 = vector_1["y"].get_f32();
        let vec_z_1 = vector_1["z"].get_f32();
        let vec_x_2 = vector_2["x"].get_f32();
        let vec_y_2 = vector_2["y"].get_f32();
        let vec_z_2 = vector_2["z"].get_f32();
        vec_x_1.assign(&L2CValue::F32(pos1_x));
        vec_y_1.assign(&L2CValue::F32(pos1_y));
        vec_x_2.assign(&L2CValue::F32(pos2_x));
        vec_y_2.assign(&L2CValue::F32(pos2_y));
        let request_effect_1 = WeaponSpecializer_MarioHugeFlame::request_effect(module_accessor, Hash40::new("fire1"), &Vector3f{x: vec_x_1, y: vec_y_1, z: 0.0});
        let request_effect_2 = WeaponSpecializer_MarioHugeFlame::request_effect(module_accessor, Hash40::new("fire2"), &Vector3f{x: vec_x_2, y: vec_y_2, z: 0.0});
        vec_x_1.assign(&L2CValue::F32(request_effect_1.x));
        vec_y_1.assign(&L2CValue::F32(request_effect_1.y));
        vec_x_2.assign(&L2CValue::F32(request_effect_2.x));
        vec_y_2.assign(&L2CValue::F32(request_effect_2.y));
        WorkModule::set_float(weapon.module_accessor, vec_x_1, WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS1_X);
        WorkModule::set_float(weapon.module_accessor, vec_y_1, WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS1_Y);
        WorkModule::set_float(weapon.module_accessor, vec_x_2, WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS2_X);
        WorkModule::set_float(weapon.module_accessor, vec_y_2, WEAPON_MARIO_HUGE_FLAME_INSTANCE_WORK_ID_FLOAT_EFFECT_POS2_Y);
    }
    else {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn mario_hugeflame_regular_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::detach_all(weapon.module_accessor, 5);
    0.into()
}