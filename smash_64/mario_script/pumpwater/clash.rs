unsafe extern "C" fn mario_pumpwater_clash_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn mario_pumpwater_clash_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("die"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(mario_pumpwater_die_main_loop as *const () as _))
}

unsafe extern "C" fn mario_pumpwater_clash_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let current_frame = weapon.global_table[0xE].get_f32();
    if current_frame == 5.0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn mario_pumpwater_clash_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    GroundModule::set_correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
    0.into()
}